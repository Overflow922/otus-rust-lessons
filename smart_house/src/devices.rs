use crate::reports::DeviceInfoProvider;
use network::server::{TcpServer, UdpMessageProcessor, UdpServer};
use network::{MessageProcessor, NetworkConnection, NetworkListener};
use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::net::ToSocketAddrs;

pub trait SmartDevice {
    fn get_name(&self) -> &str;
    fn get_room(&self) -> &str;
}

#[derive(PartialEq, Debug, Clone)]
pub struct SmartSocket {
    pub room_name: String,
    pub device_name: String,
    pub turned_on: RefCell<bool>,
}

#[derive(Debug, Clone)]
struct NetworkSmartSocketMessageProcessor {
    socket: SmartSocket,
}

impl NetworkSmartSocketMessageProcessor {
    fn create(socket: SmartSocket) -> Self {
        NetworkSmartSocketMessageProcessor { socket }
    }
}

impl MessageProcessor for &NetworkSmartSocketMessageProcessor {
    fn process(&mut self, mut conn: impl NetworkConnection) {
        if let Ok(str) = conn.recv_request() {
            if str == "status" {
                let _ = conn.send_response(self.socket.status());
            } else if str == "turn 1" {
                self.socket.turn_on();
                let _ = conn.send_response("socket turned on");
            } else if str == "turn 0" {
                self.socket.turn_off();
                let _ = conn.send_response("socket turned off");
            } else {
                let _ = conn.send_response("unknown command");
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct NetworkSmartSocket {
    pub socket: SmartSocket,
    pub server: TcpServer,
    processor: NetworkSmartSocketMessageProcessor,
}

impl NetworkSmartSocket {
    pub fn create(addr: &str, room_name: String, device_name: String) -> Self {
        let socket = SmartSocket {
            room_name,
            device_name,
            turned_on: RefCell::new(false),
        };
        Self {
            socket: socket.clone(),
            server: NetworkListener::create(addr),
            processor: NetworkSmartSocketMessageProcessor::create(socket),
        }
    }
    pub fn listen(&self) {
        self.server.listen(&self.processor);
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct SmartThermometer {
    pub room_name: String,
    pub device_name: String,
    pub temp: RefCell<u16>,
}

impl SmartSocket {
    pub fn new(room: String, device_name: String) -> Self {
        Self {
            room_name: room,
            device_name,
            turned_on: RefCell::new(false),
        }
    }

    pub fn status(&self) -> String {
        format!(
            "name {}, room: {}, is turned on: {}",
            self.device_name,
            self.device_name,
            self.turned_on.borrow()
        )
    }

    pub fn turn_on(&self) {
        self.turned_on.replace(true);
    }

    pub fn turn_off(&self) {
        self.turned_on.replace(false);
    }
}

impl SmartDevice for SmartSocket {
    fn get_name(&self) -> &str {
        &self.device_name
    }

    fn get_room(&self) -> &str {
        &self.room_name
    }
}

impl SmartDevice for NetworkSmartSocket {
    fn get_name(&self) -> &str {
        self.socket.get_name()
    }

    fn get_room(&self) -> &str {
        self.socket.get_room()
    }
}

#[derive(Clone)]
pub struct ThermometerUdpMessageProcessor {
    therm: SmartThermometer,
}
impl ThermometerUdpMessageProcessor {
    pub fn create(therm: SmartThermometer) -> Self {
        Self { therm }
    }
}

impl UdpMessageProcessor for &ThermometerUdpMessageProcessor {
    fn process(&mut self, message: network::server::UdpMessage) {
        println!("updating temp {}", message.message);
        self.therm
            .temp
            .replace(message.message.parse::<u16>().unwrap());
    }
}

#[derive(Clone)]
pub struct UdpSmartThermometer {
    pub thermometer: SmartThermometer,
    udp: UdpServer,
    processor: ThermometerUdpMessageProcessor,
}

impl SmartDevice for UdpSmartThermometer {
    fn get_name(&self) -> &str {
        self.thermometer.get_name()
    }

    fn get_room(&self) -> &str {
        self.thermometer.get_room()
    }
}

impl UdpSmartThermometer {
    pub fn create<Addr>(therm: SmartThermometer, addr: Addr) -> Self
    where
        Addr: ToSocketAddrs,
    {
        Self {
            processor: ThermometerUdpMessageProcessor::create(therm.clone()),
            thermometer: therm.clone(),
            udp: UdpServer::bind(addr).expect("connection error"),
        }
    }

    pub fn listen(&self) {
        let _ = self.udp.listen(&self.processor);
    }
}

impl SmartThermometer {
    pub fn new(room_name: String, device_name: String) -> Self {
        Self {
            room_name,
            device_name,
            temp: RefCell::new(0),
        }
    }
}

impl SmartDevice for SmartThermometer {
    fn get_name(&self) -> &str {
        &self.device_name
    }

    fn get_room(&self) -> &str {
        &self.room_name
    }
}

pub struct SmartHouse {
    devices: HashMap<String, HashMap<String, Box<dyn SmartDevice>>>,
}

pub struct SmartHouseBuilder {
    devices: HashMap<String, HashMap<String, Box<dyn SmartDevice>>>,
}

impl SmartHouseBuilder {
    pub fn new() -> SmartHouseBuilder {
        Self {
            devices: HashMap::default(),
        }
    }
    pub fn add(mut self, device: Box<dyn SmartDevice>) -> SmartHouseBuilder {
        let entry = self
            .devices
            .entry(device.get_room().to_string())
            .or_default();
        entry.insert(device.get_name().to_string(), device);
        self
    }

    pub fn add_room(mut self, name: &'static str) -> SmartHouseBuilder {
        self.devices.entry(name.to_string()).or_default();
        self
    }

    pub fn build(self) -> SmartHouse {
        SmartHouse::new(self.devices)
    }
}

impl SmartHouse {
    fn new(map: HashMap<String, HashMap<String, Box<dyn SmartDevice>>>) -> Self {
        Self { devices: map }
    }

    pub fn builder() -> SmartHouseBuilder {
        SmartHouseBuilder::new()
    }

    pub fn get_room(&self, name: String) -> Option<&HashMap<String, Box<dyn SmartDevice>>> {
        self.devices.get(&name)
    }

    pub fn get_rooms(&self) -> Vec<&str> {
        // Размер возвращаемого массива можно выбрать самостоятельно
        self.devices
            .keys()
            // .iter()
            .map(|k| k.as_str())
            .collect::<Vec<&str>>()
    }
    //  `self.devices.keys().map(|k| k.as_str())`
    pub fn devices(&self, room: String) -> Option<&HashMap<String, Box<dyn SmartDevice>>> {
        // Размер возвращаемого массива можно выбрать самостоятельно
        self.devices.get(&room)
    }

    pub fn add_room(&mut self, name: String) -> Result<(), &'static str> {
        match self.devices.entry(name) {
            Entry::Occupied(_) => Err("duplicate room. Can't add"),
            Entry::Vacant(v) => {
                v.insert(HashMap::default());
                Ok(())
            }
        }
    }

    pub fn add_device(
        &mut self,
        room_name: String,
        device: Box<dyn SmartDevice>,
    ) -> Result<(), &'static str> {
        if self.devices.contains_key(&room_name) {
            Err("room not found")
        } else {
            self.devices.entry(room_name).and_modify(|v| {
                v.insert(device.get_name().to_string(), device);
            });
            Ok(())
        }
    }
    // self.devices.get(&room_name)
    //     .expect("room not found")
    //     .insert(String::from(device.get_name()), device)
    //     .ok_or_else(|| "room not found")

    pub fn create_report<'a, T>(&'a self, provider: &'a T) -> Result<String, &str>
    where
        T: DeviceInfoProvider,
    {
        provider.get_state(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn socket_creation() {
        let socket = SmartSocket::new(String::from("room1"), String::from("device1"));
        assert_eq!(socket.room_name, "room1");
        assert_eq!(socket.device_name, "device1");
    }

    #[test]
    fn thermometer_creation() {
        let therm = SmartThermometer::new(String::from("room1"), String::from("device1"));
        assert_eq!(therm.room_name, "room1");
        assert_eq!(therm.device_name, "device1");
    }

    #[test]
    fn smart_house_creation() {
        let therm = SmartThermometer::new(String::from("room1"), String::from("device1"));
        let socket = SmartSocket::new(String::from("room2"), String::from("device1"));
        let house = SmartHouse::builder()
            .add(Box::new(therm))
            .add(Box::new(socket))
            .build();
        let rooms = house.get_rooms();
        assert_eq!(rooms.len(), 2);
        assert_eq!(
            house
                .devices(rooms[0].to_string())
                .expect("should not be empty")
                .len(),
            1
        );
        assert_eq!(
            house
                .devices(rooms[1].to_string())
                .expect("should not be empty")
                .len(),
            1
        );
    }
}

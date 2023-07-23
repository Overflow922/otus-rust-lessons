use crate::hashmap;
use crate::reports::DeviceInfoProvider;
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub struct SmartSocket {
    pub room_name: &'static str,
    pub device_name: &'static str,
}

#[derive(PartialEq, Debug)]
pub struct SmartThermometer {
    pub room_name: &'static str,
    pub device_name: &'static str,
}

impl SmartSocket {
    pub fn new(room: &'static str, device_name: &'static str) -> Self {
        Self {
            room_name: room,
            device_name,
        }
    }
}

impl SmartThermometer {
    pub fn new(room_name: &'static str, device_name: &'static str) -> Self {
        Self {
            room_name,
            device_name,
        }
    }
}

#[derive(Debug)]
pub struct SmartHouse {
    rooms: Vec<&'static str>,
    devices: HashMap<&'static str, Vec<&'static str>>,
}

impl SmartHouse {
    pub fn new() -> Self {
        Self {
            rooms: vec!["room1", "room2"],
            devices: hashmap!("room1" => vec!["thermo", "thermo1", "socket1"],
            "room2" => vec!["thermo2", "socket2", "socket3"]),
        }
    }

    pub fn get_rooms(&self) -> &Vec<&str> {
        // Размер возвращаемого массива можно выбрать самостоятельно
        &self.rooms
    }

    pub fn devices(&self, room: &str) -> Option<&Vec<&str>> {
        // Размер возвращаемого массива можно выбрать самостоятельно
        self.devices.get(room)
    }

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
        let socket = SmartSocket::new("room1", "device1");
        assert_eq!(socket.room_name, "room1");
        assert_eq!(socket.device_name, "device1");
    }

    #[test]
    fn thermometer_creation() {
        let therm = SmartThermometer::new("room1", "device1");
        assert_eq!(therm.room_name, "room1");
        assert_eq!(therm.device_name, "device1");
    }

    #[test]
    fn smart_house_creation() {
        let house = SmartHouse::new();
        let rooms = house.get_rooms();
        assert_eq!(rooms.len(), 2);
        assert_eq!(house.devices(rooms[0]).len(), 3);
        assert_eq!(house.devices(rooms[1]).len(), 3);
    }
}

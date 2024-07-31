use crate::devices::{SmartDevice, SmartHouse, SmartSocket, SmartThermometer};

pub trait DeviceInfoProvider {
    fn get_state(&self, house: &SmartHouse) -> Result<String, &str>;

    fn check(house: &SmartHouse, room_name: &str, device: &dyn SmartDevice) -> bool {
        if !house.get_rooms().contains(&room_name) {
            false
        } else {
            match house.devices(room_name.to_string()) {
                Some(el) => el.contains_key(device.get_name()),
                None => false,
            }
        }
    }
}

// Могут как хранить устройства, так и заимствывать.
pub struct OwningDeviceInfoProvider {
    socket: SmartSocket,
}

pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a SmartSocket,
    thermo: &'b SmartThermometer,
}

impl OwningDeviceInfoProvider {
    pub fn new(socket: SmartSocket) -> Self {
        Self { socket }
    }
}

impl<'a, 'b> BorrowingDeviceInfoProvider<'a, 'b> {
    pub fn new(socket: &'a SmartSocket, thermo: &'b SmartThermometer) -> Self {
        Self { socket, thermo }
    }
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn get_state(&self, house: &SmartHouse) -> Result<String, &str> {
        if !<OwningDeviceInfoProvider as DeviceInfoProvider>::check(
            house,
            &self.socket.room_name,
            &self.socket,
        ) {
            return Err("cant find device");
        }
        Ok(format!(
            "device {} in room {} is active",
            self.socket.device_name, self.socket.device_name
        ))
    }
}

impl DeviceInfoProvider for BorrowingDeviceInfoProvider<'_, '_> {
    fn get_state(&self, house: &SmartHouse) -> Result<String, &str> {
        if !<BorrowingDeviceInfoProvider as DeviceInfoProvider>::check(
            house,
            &self.socket.room_name,
            self.socket,
        ) {
            return Err("device not found");
        }
        let result = format!(
            "device {} in room {} is active",
            self.socket.device_name, self.socket.room_name
        );

        if !<BorrowingDeviceInfoProvider as DeviceInfoProvider>::check(
            house,
            &self.thermo.room_name,
            self.thermo,
        ) {
            return Err("cant find device");
        }
        Ok(format!(
            "{}\ndevice {} in room {} is active",
            result, self.thermo.device_name, self.thermo.room_name
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn own_device_creation() {
        let socket = SmartSocket::new(String::from("room1"), String::from("socket1"));
        let provider = OwningDeviceInfoProvider::new(socket);
        assert_eq!(provider.socket.room_name, "room1");
        assert_eq!(provider.socket.device_name, "socket1")
    }

    #[test]
    fn borrow_device_creation() {
        let socket = SmartSocket::new(String::from("room1"), String::from("socket1"));
        let thermometer = SmartThermometer::new(String::from("room2"), String::from("therm2"));
        let provider = BorrowingDeviceInfoProvider::new(&socket, &thermometer);
        assert_eq!(*provider.socket, socket);
        // assert_eq!(*provider.thermo, thermometer)
    }
}

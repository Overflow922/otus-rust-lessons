// ***** Пример библиотеки "Умный дом" со статическим содержимым

use std::collections::HashMap;
macro_rules! hashmap {
    ($($key:expr => $val:expr),*) => {
        {
            let mut map = HashMap::new();
            $(map.insert($key, $val);)*
            map
        }
    }
}

#[derive(Debug)]
struct SmartHouse {
    rooms: Vec<&'static str>,
    devices: HashMap<&'static str, Vec<&'static str>>,
}

impl SmartHouse {
    fn new() -> Self {
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

    pub fn devices(&self, room: &str) -> &Vec<&str> {
        // Размер возвращаемого массива можно выбрать самостоятельно
        match self.devices.get(room) {
            Some(v) => v,
            None => panic!("no room found"),
        }
    }

    fn create_report<T>(&self, provider: &T) -> String
        where
            T: DeviceInfoProvider,
    {
        provider.get_state(self)
    }
}

trait DeviceInfoProvider {
    fn get_state(&self, house: &SmartHouse) -> String;

    fn check(house: &SmartHouse, room_name: &str, device_name: &str) -> bool {
        house.get_rooms().contains(&room_name) && house.devices(room_name).contains(&device_name)
    }
}

// ***** Пример использования библиотеки умный дом:

// Пользовательские устройства:
struct SmartSocket {
    room_name: &'static str,
    device_name: &'static str,
}

struct SmartThermometer {
    room_name: &'static str,
    device_name: &'static str,
}

// Пользовательские поставщики информации об устройствах.
// Могут как хранить устройства, так и заимствывать.
struct OwningDeviceInfoProvider {
    socket: SmartSocket,
}

struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a SmartSocket,
    thermo: &'b SmartThermometer,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn get_state(&self, house: &SmartHouse) -> String {
        if !<OwningDeviceInfoProvider as DeviceInfoProvider>::check(
            house,
            self.socket.room_name,
            self.socket.device_name,
        ) {
            panic!("cant find device");
        }
        format!(
            "device {} in room {} is active",
            self.socket.device_name, self.socket.device_name
        )
    }
}

impl DeviceInfoProvider for BorrowingDeviceInfoProvider<'_, '_> {
    fn get_state(&self, house: &SmartHouse) -> String {
        if !<BorrowingDeviceInfoProvider as DeviceInfoProvider>::check(
            house,
            self.socket.room_name,
            self.socket.device_name,
        ) {
            panic!("cant find device");
        }
        let result = format!(
            "device {} in room {} is active",
            self.socket.device_name, self.socket.room_name
        );

        if !<BorrowingDeviceInfoProvider as DeviceInfoProvider>::check(
            house,
            self.thermo.room_name,
            self.thermo.device_name,
        ) {
            panic!("cant find device");
        }
        format!(
            "{}\ndevice {} in room {} is active",
            result, self.thermo.device_name, self.thermo.room_name
        )
    }
}

fn main() {
    // Инициализация устройств
    let socket1 = SmartSocket {
        room_name: "room1",
        device_name: "socket1",
    };

    let socket2 = SmartSocket {
        room_name: "room2",
        device_name: "socket2",
    };
    let thermo = SmartThermometer {
        room_name: "room1",
        device_name: "thermo1",
    };

    // Инициализация дома
    let house = SmartHouse::new();

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };

    let report1 = house.create_report(&info_provider_1);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };

    let report2 = house.create_report(&info_provider_2);

    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}

mod devices;
mod reports;
mod utils;

use network::

use crate::devices::{SmartHouse, SmartSocket, SmartThermometer};
use crate::reports::{BorrowingDeviceInfoProvider, OwningDeviceInfoProvider};

fn main() {
    // Инициализация устройств
    let socket1 = Box::new(SmartSocket::new(
        String::from("room1"),
        String::from("socket1"),
    ));

    let socket2 = Box::new(SmartSocket::new(
        String::from("room2"),
        String::from("socket2"),
    ));
    let thermo = Box::new(SmartThermometer::new(
        String::from("room1"),
        String::from("thermo1"),
    ));

    // Инициализация дома
    let mut house = SmartHouse::builder()
        .add(socket1.clone())
        .add(socket2.clone())
        .add(thermo.clone())
        .build();

    if let Err(err) = house.add_room("room3".to_string()) {
        eprintln!("Failed to add room. reason: {err}")
    }

    if let Err(err) = house.add_device(
        "room3".to_string(),
        Box::new(SmartThermometer::new(
            "room3".to_string(),
            "therm2".to_string(),
        )),
    ) {
        eprintln!("Failed to add device. Reason: {err}")
    }

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider::new(*socket1);

    match house.create_report(&info_provider_1) {
        Ok(s) => println!("Report #1: {s}"),
        Err(e) => println!("Error occurred: {e}"),
    }

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider::new(&socket2, &thermo);

    match house.create_report(&info_provider_2) {
        Ok(s) => println!("Report #2:\n{s}"),
        Err(e) => println!("Error occurred:\n{e}"),
    }
}

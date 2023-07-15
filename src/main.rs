mod devices;
mod reports;
mod utils;

use crate::devices::{SmartHouse, SmartSocket, SmartThermometer};
use crate::reports::{BorrowingDeviceInfoProvider, OwningDeviceInfoProvider};

fn main() {
    // Инициализация устройств
    let socket1 = SmartSocket::new("room1", "socket1");

    let socket2 = SmartSocket::new("room2", "socket2");
    let thermo = SmartThermometer::new("room1", "thermo1");

    // Инициализация дома
    let house = SmartHouse::new();

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider::new(socket1);

    let report1 = house.create_report(&info_provider_1);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider::new(&socket2, &thermo);

    let report2 = house.create_report(&info_provider_2);

    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}

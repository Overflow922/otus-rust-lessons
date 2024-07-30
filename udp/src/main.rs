use core::time;
use std::thread::sleep;

use client::SmartThermometerClient;

fn main() {
    let client = SmartThermometerClient::new("127.0.0.1:32001", "127.0.0.1:33440").unwrap();
    let mut temp: u16 = 0;
    loop {
        println!("updating temp: {}", temp);
        let _ = client.update_temp(temp);
        temp += 1;
        sleep(time::Duration::from_secs(3));
    }
}

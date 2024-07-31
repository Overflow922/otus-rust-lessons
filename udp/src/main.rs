use core::time;
use std::thread::sleep;

use client::SmartThermometerClient;

#[tokio::main]
async fn main() {
    let client = SmartThermometerClient::new("127.0.0.1:32001", "127.0.0.1:33440".to_string())
        .await
        .unwrap();
    let mut temp: u16 = 0;
    loop {
        println!("updating temp: {}", temp);
        let _ = client.update_temp(temp).await;
        temp += 1;
        sleep(time::Duration::from_secs(3));
    }
}

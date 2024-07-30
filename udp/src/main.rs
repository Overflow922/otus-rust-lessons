use client::SmartThermometerClient;

fn main() {
    let client = SmartThermometerClient::new("127.0.0.1:32001", "127.0.0.1:33440").unwrap();

    let _ = client.update_temp(32);
}

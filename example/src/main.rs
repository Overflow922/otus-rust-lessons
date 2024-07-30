use client::SmartSocketClient;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut client = SmartSocketClient::new("127.0.0.1:33445")?;
    client.turn_on()?;
    let mut client = SmartSocketClient::new("127.0.0.1:33445")?;
    client.status()?;
    client.turn_off()?;
    // assert_eq!(response, "Hello, client");
    Ok(())
}

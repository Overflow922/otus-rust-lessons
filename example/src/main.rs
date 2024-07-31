use client::SmartSocketClient;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = SmartSocketClient::new("127.0.0.1:33445").await?;
    client.turn_on().await?;
    let mut client = SmartSocketClient::new("127.0.0.1:33445").await?;
    client.status().await?;
    client.turn_off().await?;
    // assert_eq!(response, "Hello, client");
    Ok(())
}

use network::client::{HttpConnectionClient, RequestError};
use network::utils::ConnectResult;
use std::net::ToSocketAddrs;

pub struct SmartSocketClient {
    pub(crate) client: HttpConnectionClient,
}

impl SmartSocketClient {
    pub fn new<Addr: ToSocketAddrs>(addr: Addr) -> ConnectResult<Self> {
        let client = HttpConnectionClient::connect(addr)?;
        Ok(Self { client })
    }

    pub fn turn_on(&mut self) -> Result<(), RequestError> {
        self.turn(1)
    }

    pub fn turn_off(&mut self) -> Result<(), RequestError> {
        self.turn(0)
    }

    fn turn(&mut self, state: u16) -> Result<(), RequestError> {
        let result = self.client.send_request(format!("turn {}", state))?;
        println!("response is: {}", result);
        Ok(())
    }

    pub fn status(&mut self) -> Result<(), RequestError> {
        let result = self.client.send_request("status")?;
        println!("status is: {}", result);
        Ok(())
    }
}

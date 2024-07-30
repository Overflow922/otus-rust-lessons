use network::client::{HttpConnectionClient, RequestError};
use network::server::UdpServer;
use network::utils::{ConnectResult, SendResult};
use std::net::{SocketAddr, ToSocketAddrs};

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

pub struct SmartThermometerClient {
    pub client: UdpServer,
    target: SocketAddr,
}

impl SmartThermometerClient {
    pub fn new<Addr: ToSocketAddrs>(addr: Addr, target: Addr) -> ConnectResult<Self> {
        let client = UdpServer::bind(addr)?;
        let t = target
            .to_socket_addrs()
            .expect("wrong addr string")
            .next()
            .unwrap();
        Ok(Self { client, target: t })
    }

    pub fn update_temp(&self, temp: u16) -> SendResult {
        self.client.send(temp.to_string(), &self.target)
    }
}

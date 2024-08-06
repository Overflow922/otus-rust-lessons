use network::client::{HttpConnectionClient, RequestError};
use network::server::UdpServer;
use network::utils::{ConnectResult, SendResult};
use tokio::net::ToSocketAddrs;

pub struct SmartSocketClient {
    pub(crate) client: HttpConnectionClient,
}

impl SmartSocketClient {
    pub async fn new<Addr: ToSocketAddrs>(addr: Addr) -> ConnectResult<Self> {
        let client = HttpConnectionClient::connect(addr).await.unwrap();
        Ok(Self { client })
    }

    pub async fn turn_on(&mut self) -> Result<String, RequestError> {
        self.turn(1).await
    }

    pub async fn turn_off(&mut self) -> Result<String, RequestError> {
        self.turn(0).await
    }

    async fn turn(&mut self, state: u16) -> Result<String, RequestError> {
        let result = self.client.send_request(format!("turn {}", state)).await?;
        println!("response is: {}", result);
        Ok(result)
    }

    pub async fn status(&mut self) -> Result<String, RequestError> {
        let result = self.client.send_request("status").await?;
        println!("status is: {}", result);
        Ok(result)
    }
}

pub struct SmartThermometerClient {
    pub client: UdpServer,
    target: String,
}

impl SmartThermometerClient {
    pub async fn new<Addr: ToSocketAddrs>(addr: Addr, target: String) -> ConnectResult<Self> {
        let client = UdpServer::bind(addr).await?;
        Ok(Self { client, target })
    }

    pub async fn update_temp(&self, temp: u16) -> SendResult {
        self.client.send(temp.to_string(), &self.target).await
    }
}

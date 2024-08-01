use crate::utils::{ConnectError, ConnectResult, RecvError, RecvResult, SendResult};
use crate::{NetworkConnection, NetworkListener};
use core::str;
use std::io::{self};
use std::net::SocketAddr;
use std::rc::Rc;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs, UdpSocket};

const PROTO_VER: &[u8; 4] = b"0001";

#[derive(Debug, Clone)]
pub struct TcpServer {
    server: Rc<TcpListener>,
}

impl TcpServer {
    async fn accept(&self) -> ConnectResult<TcpConnection> {
        let (con, _) = self.server.accept().await?;
        TcpServer::try_handshake(con).await
    }

    async fn try_handshake(mut stream: TcpStream) -> ConnectResult<TcpConnection> {
        let mut buf: [u8; 4] = [0; 4];
        stream.read_exact(&mut buf).await?;
        println!(
            "start handshaking. Expected ver is {:?}, actual: {:?}",
            PROTO_VER, buf
        );
        if &buf != PROTO_VER {
            let msg = format!("received: {:?}", buf);
            return Err(ConnectError::BadHandshake(msg));
        }
        println!("Sending back version");
        stream.write_all(PROTO_VER).await?;
        Ok(TcpConnection { stream })
    }
}

impl NetworkListener for TcpServer {
    async fn create<Addr>(addr: Addr) -> Self
    where
        Addr: ToSocketAddrs,
    {
        println!("Creating server");
        TcpServer {
            server: Rc::new(TcpListener::bind(addr).await.unwrap()),
        }
    }

    async fn listen(&self) {
        println!("listening for incoming connection...");

        loop {
            let con = match self.accept().await {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Can't establish connection: {}", e);
                    continue;
                }
            };
            tokio::spawn(async move { TcpServer::process(con).await });
        }
    }

    async fn process(mut conn: impl NetworkConnection) {
        if let Ok(str) = conn.recv_request().await {
            if str == "status" {
                let _ = conn.send_response("status").await;
            } else if str == "turn 1" {
                let _ = conn.send_response("socket turned on").await;
            } else if str == "turn 0" {
                let _ = conn.send_response("socket turned off").await;
            } else {
                let _ = conn.send_response("unknown command").await;
            }
        }
    }
}

pub struct TcpConnection {
    stream: TcpStream,
}

impl NetworkConnection for TcpConnection {
    async fn send_response<Resp: AsRef<str>>(&mut self, response: Resp) -> SendResult {
        crate::send_string(response, &mut self.stream).await
    }

    /// Receive requests from client
    async fn recv_request(&mut self) -> RecvResult {
        crate::recv_string(&mut self.stream).await
    }

    /// Address of connected client
    fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.stream.peer_addr()
    }
}

pub trait UdpMessageProcessor {
    fn process(&mut self, message: UdpMessage);
}

#[derive(Clone)]
pub struct UdpServer {
    server: Arc<UdpSocket>,
}

pub struct UdpMessage {
    pub source: SocketAddr,
    pub message: String,
}

impl UdpServer {
    pub async fn bind<Addr>(addr: Addr) -> ConnectResult<Self>
    where
        Addr: ToSocketAddrs,
    {
        let conn = UdpSocket::bind(addr).await.unwrap();
        Ok(Self {
            server: Arc::new(conn),
        })
    }

    pub async fn listen(&self, mut processor: impl UdpMessageProcessor) -> Result<(), RecvError> {
        let mes = self.recv_string().await;

        match mes {
            Ok(udp) => {
                processor.process(udp);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub async fn send<Resp: AsRef<str>>(&self, mess: Resp, source: &String) -> SendResult {
        self.send_string(mess, source).await
    }

    pub async fn send_string<Data: AsRef<str>>(&self, data: Data, source: &String) -> SendResult {
        let bytes = data.as_ref().as_bytes();
        let len = bytes.len() as u32;
        let len_bytes = len.to_be_bytes();
        self.server.send_to(&len_bytes, source).await?;
        self.server.send_to(bytes, source).await?;
        Ok(())
    }

    async fn recv_string(&self) -> Result<UdpMessage, RecvError> {
        let mut buf = [0; 4];
        self.server.recv_from(&mut buf).await?;
        let len = u32::from_be_bytes(buf);

        let mut buf = vec![0; len as _];
        let (_, source) = self.server.recv_from(&mut buf).await?;
        match String::from_utf8(buf) {
            Ok(v) => Ok(UdpMessage { source, message: v }),
            Err(_) => Err(RecvError::BadEncoding),
        }
    }
}

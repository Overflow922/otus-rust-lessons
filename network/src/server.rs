use crate::utils::{ConnectError, ConnectResult, RecvError, RecvResult, SendResult};
use crate::{MessageProcessor, NetworkConnection, NetworkListener};
use core::str;
use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs, UdpSocket};
use std::rc::Rc;
use std::sync::Arc;

const PROTO_VER: &[u8; 4] = b"0001";

#[derive(Debug, Clone)]
pub struct TcpServer {
    server: Rc<TcpListener>,
}

impl TcpServer {
    fn incoming(&self) -> impl Iterator<Item = ConnectResult<TcpConnection>> + '_ {
        self.server.incoming().map(|s| match s {
            Ok(s) => TcpServer::try_handshake(s),
            Err(e) => Err(ConnectError::Io(e)),
        })
    }

    fn try_handshake(mut stream: TcpStream) -> ConnectResult<TcpConnection> {
        let mut buf: [u8; 4] = [0; 4];
        stream.read_exact(&mut buf)?;
        println!(
            "start handshaking. Expected ver is {:?}, actual: {:?}",
            PROTO_VER, buf
        );
        if &buf != PROTO_VER {
            let msg = format!("received: {:?}", buf);
            return Err(ConnectError::BadHandshake(msg));
        }
        println!("Sending back version");
        stream.write_all(PROTO_VER)?;
        Ok(TcpConnection { stream })
    }
}

impl NetworkListener for TcpServer {
    fn create<Addr>(addr: Addr) -> Self
    where
        Addr: ToSocketAddrs,
    {
        println!("Creating server");
        TcpServer {
            server: Rc::new(TcpListener::bind(addr).unwrap()),
        }
    }

    fn listen(&self, mut processor: impl MessageProcessor) {
        println!("listening for incoming connection...");
        for conn in self.incoming() {
            processor.process(conn.unwrap());
        }
    }
}

struct TcpConnection {
    stream: TcpStream,
}

impl NetworkConnection for TcpConnection {
    fn send_response<Resp: AsRef<str>>(&mut self, response: Resp) -> SendResult {
        crate::send_string(response, &mut self.stream)
    }

    /// Receive requests from client
    fn recv_request(&mut self) -> RecvResult {
        crate::recv_string(&mut self.stream)
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
    pub fn bind<Addr>(addr: Addr) -> ConnectResult<Self>
    where
        Addr: ToSocketAddrs,
    {
        let conn = UdpSocket::bind(addr)?;
        Ok(Self {
            server: Arc::new(conn),
        })
    }

    pub fn listen(&self, mut processor: impl UdpMessageProcessor) -> Result<(), RecvError> {
        match self.recv_string() {
            Ok(udp) => {
                processor.process(udp);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn send<Resp: AsRef<str>>(&self, mess: Resp, source: &SocketAddr) -> SendResult {
        self.send_string(mess, source)
    }

    fn send_string<Data: AsRef<str>>(&self, data: Data, source: &SocketAddr) -> SendResult {
        let bytes = data.as_ref().as_bytes();
        let len = bytes.len() as u32;
        let len_bytes = len.to_be_bytes();
        self.server.send_to(&len_bytes, source)?;
        self.server.send_to(bytes, source)?;
        Ok(())
    }

    fn recv_string(&self) -> Result<UdpMessage, RecvError> {
        let mut buf = [0; 4];
        self.server.recv_from(&mut buf)?;
        let len = u32::from_be_bytes(buf);

        let mut buf = vec![0; len as _];
        let (_, source) = self.server.recv_from(&mut buf)?;
        match String::from_utf8(buf) {
            Ok(v) => Ok(UdpMessage { source, message: v }),
            Err(_) => Err(RecvError::BadEncoding),
        }
    }
}

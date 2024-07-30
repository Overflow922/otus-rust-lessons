use crate::utils::{ConnectError, ConnectResult, RecvResult, SendResult};
use crate::{MessageProcessor, NetworkConnection, NetworkListener};
use std::io;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
use std::rc::Rc;

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

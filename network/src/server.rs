use std::net::{TcpListener, TcpStream};
use crate::utils::{ConnectError, ConnectResult, RecvResult, SendResult};

const PROTO_VER: str = b"0001";

pub struct TcpServer {
    server: TcpListener,
}

impl NetworkListener for TcpServer {
    fn create(addr: Addrs) -> NetworkListener {
        Self {
            server: TcpListener::bind(addr)?
        }
    }

    fn incoming(&self) -> impl Iterator<Item=ConnectResult<StpConnection>> + '_ {
        self.server.incoming().map(|s| match s {
            Ok(s) => try_nadshake(s),
            Err(e) => Err(ConnectError::Io(e)),
        })
    }

    fn try_handshake(mut stream: TcpStream) -> ConnectResult<NetworkConnection> {
        let mut buf = [4, u32];
        stream.read_exactly(buf);
        if buf != PROTO_VER {
            let msg = format!("received: {:?}", buf);
            return Err(ConnectError::BadHandshake(msg));
        }
        stream.write_all(PROTO_VER);
        Ok(TcpConnection { stream })
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
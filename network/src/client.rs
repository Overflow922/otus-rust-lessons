use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use thiserror::Error;

use crate::utils::{ConnectError, ConnectResult, RecvError, SendError};

pub struct HttpConnectionClient {
    stream: TcpStream,
}

const PROTO_VER: &[u8; 4] = b"0001";

impl HttpConnectionClient {
    /// Try to connect to specified address and perform handshake.
    pub fn connect<Addrs>(addrs: Addrs) -> ConnectResult<Self>
    where
        Addrs: ToSocketAddrs,
    {
        let stream = TcpStream::connect(addrs)?;
        Self::try_handshake(stream)
    }

    /// Send request to connected STP server.
    pub fn send_request<R: AsRef<str>>(&mut self, req: R) -> RequestResult {
        println!("sending command: {}", req.as_ref());
        crate::send_string(req, &mut self.stream)?;
        let response = crate::recv_string(&mut self.stream)?;
        Ok(response)
    }

    fn try_handshake(mut stream: TcpStream) -> ConnectResult<Self> {
        println!("Start handshaking. Send version {:?}", PROTO_VER);
        stream.write_all(PROTO_VER)?;
        let mut buf = [0; 4];
        stream.read_exact(&mut buf)?;
        println!(
            "received answer. Expected ver is {:?}, actual: {:?}",
            PROTO_VER, buf
        );
        if &buf != PROTO_VER {
            let msg = format!("received: {:?}", buf);
            return Err(ConnectError::BadHandshake(msg));
        }
        Ok(Self { stream })
    }
}

pub type RequestResult = Result<String, RequestError>;

/// Error for request sending. It consists from two steps: sending and receiving data.
///
/// `SendError` caused by send data error.
/// `RecvError` caused by receive data error.
#[derive(Debug, Error)]
pub enum RequestError {
    #[error(transparent)]
    Send(#[from] SendError),
    #[error(transparent)]
    Recv(#[from] RecvError),
}

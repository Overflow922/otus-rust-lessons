use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, ToSocketAddrs};

use crate::utils::{self, ConnectError, ConnectResult, RecvError, SendError};

pub struct HttpConnectionClient {
    stream: TcpStream,
}

const PROTO_VER: &[u8; 4] = b"0001";

impl HttpConnectionClient {
    /// Try to connect to specified address and perform handshake.
    pub async fn connect<Addrs>(addrs: Addrs) -> ConnectResult<Self>
    where
        Addrs: ToSocketAddrs,
    {
        let stream = TcpStream::connect(addrs).await?;
        Self::try_handshake(stream).await
    }

    /// Send request to connected STP server.
    pub async fn send_request<R: AsRef<str>>(&mut self, req: R) -> RequestResult {
        println!("sending command: {}", req.as_ref());
        crate::send_string(req, &mut self.stream).await?;
        let response = crate::recv_string(&mut self.stream).await?;
        Ok(response)
    }

    async fn try_handshake(mut stream: TcpStream) -> ConnectResult<Self> {
        println!("Start handshaking. Send version {:?}", PROTO_VER);
        stream.write_all(PROTO_VER).await?;
        let mut buf = [0; 4];
        stream.read_exact(&mut buf).await?;
        println!(
            "received answer. Expected ver is {:?}, actual: {:?}",
            PROTO_VER, buf
        );
        if !utils::handshake(buf, *PROTO_VER) {
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

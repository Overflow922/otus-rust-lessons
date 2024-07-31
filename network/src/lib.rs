use crate::utils::{RecvError, RecvResult, SendResult};
use std::io;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, ToSocketAddrs};

pub mod client;
pub mod server;
pub mod utils;

pub trait MessageProcessor {
    fn process(&mut self, conn: impl NetworkConnection) -> impl std::future::Future<Output = ()> + Send;
    // fn process(conn: TcpConnection) -> impl std::future::Future<Output = ()> + Send;
}

pub trait NetworkListener {
    async fn create<Addr>(addr: Addr) -> Self
    where
        Addr: ToSocketAddrs;

    async fn listen(&self);

    async fn process(conn: impl NetworkConnection);    
}

pub trait NetworkConnection {
    fn send_response<Resp: AsRef<str> + std::marker::Send>(
        &mut self,
        response: Resp,
    ) -> impl std::future::Future<Output = SendResult> + Send;

    /// Receive requests from client
    fn recv_request(&mut self) -> impl std::future::Future<Output = RecvResult> + Send;

    /// Address of connected client
    fn peer_addr(&self) -> io::Result<SocketAddr>;
}

async fn send_string<Data: AsRef<str>>(data: Data, writer: &mut TcpStream) -> SendResult {
    let bytes = data.as_ref().as_bytes();
    let len = bytes.len() as u32;
    let len_bytes = len.to_be_bytes();
    writer.write_all(&len_bytes).await?;
    writer.write_all(bytes).await?;
    Ok(())
}

async fn recv_string(reader: &mut TcpStream) -> RecvResult {
    let mut buf = [0; 4];
    reader.read_exact(&mut buf).await?;
    let len = u32::from_be_bytes(buf);

    let mut buf = vec![0; len as _];
    reader.read_exact(&mut buf).await?;
    String::from_utf8(buf).map_err(|_| RecvError::BadEncoding)
}

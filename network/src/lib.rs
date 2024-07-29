use crate::utils::{RecvError, RecvResult, SendResult};
use std::io;
use std::io::{Read, Write};
use std::net::{SocketAddr, ToSocketAddrs};

pub mod server;
mod utils;

pub trait MessageProcessor {
    fn process(&self, conn: impl NetworkConnection);
}

pub trait NetworkListener {
    fn create<Addr>(addr: Addr) -> Self
    where
        Addr: ToSocketAddrs;

    fn listen(&self, processor: impl MessageProcessor);
}

pub trait NetworkConnection {
    fn send_response<Resp: AsRef<str>>(&mut self, response: Resp) -> SendResult;

    /// Receive requests from client
    fn recv_request(&mut self) -> RecvResult;

    /// Address of connected client
    fn peer_addr(&self) -> io::Result<SocketAddr>;
}

fn send_string<Data: AsRef<str>, Writer: Write>(data: Data, mut writer: Writer) -> SendResult {
    let bytes = data.as_ref().as_bytes();
    let len = bytes.len() as u32;
    let len_bytes = len.to_be_bytes();
    writer.write_all(&len_bytes)?;
    writer.write_all(bytes)?;
    Ok(())
}

fn recv_string<Reader: Read>(mut reader: Reader) -> RecvResult {
    let mut buf = [0; 4];
    reader.read_exact(&mut buf)?;
    let len = u32::from_be_bytes(buf);

    let mut buf = vec![0; len as _];
    reader.read_exact(&mut buf)?;
    String::from_utf8(buf).map_err(|_| RecvError::BadEncoding)
}

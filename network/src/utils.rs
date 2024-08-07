use core::str;
use std::{
    ffi::{c_char, CString},
    io,
};
use thiserror::Error;

use crate::server::UdpMessage;

pub type ConnectResult<T> = Result<T, ConnectError>;

/// Connection error. Includes IO and handshake error.
#[derive(Debug, Error)]
pub enum ConnectError {
    #[error("Unexpected handshake response: {0}")]
    BadHandshake(String),
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

pub type SendResult = Result<(), SendError>;

/// Send data error
#[derive(Debug, Error)]
pub enum SendError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

pub type RecvResult = Result<String, RecvError>;

/// Send data error. Includes IO and encoding error.
#[derive(Debug, Error)]
pub enum RecvError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("bad encoding")]
    BadEncoding,
}

pub type UdpRecvResult = Result<UdpMessage, RecvError>;

#[link(name = "dyn_lib")]
extern "C" {
    pub fn do_handshake(left: *const c_char, right: *const c_char) -> bool;
}

pub fn handshake(left: [u8; 4], right: [u8; 4]) -> bool {
    let left = CString::new(std::str::from_utf8(&left).unwrap()).unwrap();
    let right = CString::new(std::str::from_utf8(&right).unwrap()).unwrap();

    unsafe { do_handshake(left.as_ptr(), right.as_ptr()) }
}

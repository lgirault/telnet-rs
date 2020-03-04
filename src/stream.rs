use std::time::Duration;
use std::io::{Result};
use tokio::io::{AsyncRead,AsyncWrite};
use tokio::net::TcpStream;
use std::net::Shutdown;

pub trait Stream: AsyncRead + AsyncWrite {
//    fn set_read_timeout(&self, dur: Option<Duration>) -> Result<()>;
    fn shutdown(&self)-> Result<()>;
}

impl Stream for TcpStream {
    // fn set_read_timeout(&self, dur: Option<Duration>) -> Result<()> {
    //     tokio::net::TcpStream::set_read_timeout(self, dur)
    // }
    fn shutdown(&self)-> Result<()> {
        TcpStream::shutdown(self, Shutdown::Both)
    }
}
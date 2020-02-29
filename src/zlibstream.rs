extern crate flate2;
extern crate replace_with;

use std::time::Duration;
use std::io::{Read, Write, Result};
use zlibstream::flate2::read::ZlibDecoder;
use stream::Stream;
use zcstream::ZCStream;

enum ZlibStreamSwitch<T> {
    Plain(T),
    Encoded(ZlibDecoder<T>)
}

/// A wrapper which can enable and disable zlib decompression for downstream at runtime.
///
/// # Examples
///
/// ```ignore
/// let mut stream = ZlibStream::from_stream(old_stream);
/// stream.begin_zlib();
/// ```
pub struct ZlibStream<T> {
    stream: ZlibStreamSwitch<T>
}

impl <T> ZlibStream<T> where T: Read {
    pub fn from_stream(stream: T) -> Self {
        ZlibStream::<T>{stream: ZlibStreamSwitch::Plain(stream)}
    }
}


impl <T> Read for ZlibStream<T> where T: Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self.stream {
            ZlibStreamSwitch::Plain(ref mut stream) => stream.read(buf),
            ZlibStreamSwitch::Encoded(ref mut stream) => stream.read(buf)
        }
    }
}

impl <T> Write for ZlibStream<T> where T: Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        match self.stream {
            ZlibStreamSwitch::Plain(ref mut stream) => stream.write(buf),
            ZlibStreamSwitch::Encoded(ref mut stream) => stream.get_mut().write(buf)
        }
    }
    fn flush(&mut self) -> Result<()> {
        match self.stream {
            ZlibStreamSwitch::Plain(ref mut stream) => stream.flush(),
            ZlibStreamSwitch::Encoded(ref mut stream) => stream.get_mut().flush()
        }
    }
}

impl <T> Stream for ZlibStream<T> where T: Stream {
    fn set_nonblocking(&self, nonblocking: bool) -> Result<()> {
        match self.stream {
            ZlibStreamSwitch::Plain(ref stream) => stream.set_nonblocking(nonblocking),
            ZlibStreamSwitch::Encoded(ref stream) => stream.get_ref().set_nonblocking(nonblocking)
        }
    }

    fn set_read_timeout(&self, dur: Option<Duration>) -> Result<()> {
        match self.stream {
            ZlibStreamSwitch::Plain(ref stream) => stream.set_read_timeout(dur),
            ZlibStreamSwitch::Encoded(ref stream) => stream.get_ref().set_read_timeout(dur)
        }
    }

    fn shutdown(&self)-> Result<()> {
        match self.stream {
            ZlibStreamSwitch::Plain(ref stream) => stream.shutdown(),
            ZlibStreamSwitch::Encoded(ref stream) => stream.get_ref().shutdown()
        }
    }
}

impl <T> ZCStream for ZlibStream<T> where T: Stream {
    fn begin_zlib(&mut self) {
        replace_with::replace_with_or_abort(&mut self.stream, |stream| 
            match stream {
                ZlibStreamSwitch::Plain(stream) => ZlibStreamSwitch::Encoded(ZlibDecoder::new(stream)),
                e => e
            }
        )
    }
    fn end_zlib(&mut self) {
        replace_with::replace_with_or_abort(&mut self.stream, |stream| 
            match stream {
                ZlibStreamSwitch::Encoded(stream) => ZlibStreamSwitch::Plain(stream.into_inner()),
                p => p
            }
        )
    }
}

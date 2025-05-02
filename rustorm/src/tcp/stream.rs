use std::net::TcpStream;
use std::io::{Read, Write};

pub struct PgStream {
    stream: TcpStream,
}

impl PgStream {
    pub fn connect(host: &str, port: u16) -> std::io::Result<Self> {
        let stream = TcpStream::connect((host, port))?;
        Ok(PgStream { stream })
    }

    pub fn send(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.stream.write_all(buf)
    }

    pub fn receive(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.stream.read(buf)
    }
}
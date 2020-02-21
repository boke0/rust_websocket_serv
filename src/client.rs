use std::net::TcpStream;
use std::io::prelude::*;
use super::dataframe::DataFrame;

pub struct Client {
    pub stream: TcpStream,
}

impl Client{
    pub fn new(stream: TcpStream) -> Client {
        Client {
            stream: stream,
        }
    }
    pub fn run(&mut self) {
        println!("started!");
        loop {
            let mut buf= [0; 2*1024*1024];
            self.stream.read(&mut buf).unwrap();
            let data=DataFrame::from(&buf);
            if data.opcode == 1 {
                println!("{}",String::from_utf8_lossy(data.as_slice()));
            }
        }
    }
}

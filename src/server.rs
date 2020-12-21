use std::net::TcpListener;
use std::io::{Read, Write};
use std::convert::TryFrom;
use crate::http::Request;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }
    pub fn run(self) {
        println!("Our server is listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, socket)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}\nFrom socket address {}", String::from_utf8_lossy(&buffer), socket);

                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    write!(stream, "HTTP/1.1 404 Not Found\r\n\r\n");
                                },
                                Err(e) => println!("Failed to parse a request: {}", e)
                            }
                            //let res: &Result<Request, _> = &buffer[..].try_into();
                        },
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                },
                Err(e) => println!("Failed to establish a connection: {}", e),
            }

            let res = listener.accept();
            if res.is_err() {
                continue;
            }

            let (stream, addr) = res.unwrap();
        }
    }
}

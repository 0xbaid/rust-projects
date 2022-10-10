use crate::http::{request, Request};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;
pub struct Server {
    addr: String,
}

impl Server {
    //associated function
    //associated with struct types, but dont need instance of struct
    //similar like static function in other languages
    //used :: to access it directly
    pub fn new(addr: String) -> Server {
        Server { addr }
    }
    //method
    //takes first parameter called self, self represents instance of struct
    //on which the method is called on
    pub fn run(&self) {
        println!("Listening to {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Recived a request: {}", String::from_utf8_lossy(&buffer));
                            // normally it converts but here we explicitly convert to byte slice by using ..
                            match Request::try_from(&buffer[..]) {
                                Ok((request)) => {}
                                Err(e) => println!(),
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection"),
            }
        }
    }
}

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    //associated function
    //associated with struct types, but dont need instance of struct
    //similar like static function in other languages
    //used :: to access it directly
    fn new(addr: String) -> Server {
        Server { addr }
    }
    //method
    //takes first parameter called self, self represents instance of struct
    //on which the method is called on
    fn run(&self) {
        println!("Listening to {}", self.addr);
    }
}

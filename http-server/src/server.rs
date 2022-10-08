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
        let listener = TcpListener::bind(&self.addr);
    }
}

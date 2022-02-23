use std::net::TcpListener;
use std::io::Read;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            // addr: addr
            // or, to simplify, simply:
            addr
        }
    }
    pub fn run(self) {
        println!("listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    // println!("ok");
                    let mut buffer = [0; 1024]; // for production, this should be more dynamically sized
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                        },
                        Err(e) => println!("Failed to read from connection: {}", e)
                    };
                },
                Err(e) => println!("Error establishing connection: {}", e),
            }
        }
    }
}

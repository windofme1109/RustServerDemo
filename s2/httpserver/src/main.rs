mod handler;
mod router;
mod server;

use crate::server::Server;


fn main() {
    // println!("Hello, world!");

    let s = Server::new("localhost:10086");

    s.run();
}

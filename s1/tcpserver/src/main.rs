use std::net::TcpListener;
use std::io::{Read, Write};
fn main() {
    // 监听来自 127.0.0.1:3000 的连接
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    println!("Running on port 3000 ...");

    for stream in listener.incoming() {

        println!("Cooection established");
        
        let mut stream = stream.unwrap();
        
        // 建立二进制缓冲区
        let mut buffer = [0; 1024];

        stream.read(&mut buffer);

        stream.write(&mut buffer);
    }
}

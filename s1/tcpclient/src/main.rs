use std::net::TcpStream;
use std::io::{Read, Write};
use std::str;

fn main() {
    
    // 使用 TcpStream 这个包中的 connect 函数来建立客户端与服务端的链接，返回值是一个 Stream
    let mut stream = TcpStream::connect("localhost:3000").unwrap();

    stream.write("hello".as_bytes()).unwrap();

    let mut buffer = [0; 5];

    stream.read(&mut buffer).unwrap();

    println!("Response from server: {:?}", str::from_utf8(&buffer).unwrap())
    
}

use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;
fn main() {

    let address = String::from("localhost:10000");


    // 使用 connect 函数建立一个连接
    let mut stream = TcpStream::connect(address).unwrap();

    stream.write("hello".as_bytes());

    let mut buf = [0; 5];

    stream.read(&mut buf).unwrap();


    println!("the response is {:?}", str::from_utf8(&buf).unwrap());



}

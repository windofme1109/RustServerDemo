use std::net::TcpListener;
use std::io::{Read, Write};
use std::str::from_utf8;

fn main() {

    let address = String::from("localhost:10000");

    let listener = TcpListener::bind(&address).unwrap();

    println!("Server is running on {}", address);

    // 当 listener 收到一个连接的时候。incoming 返回一个迭代器（iterator）
    // 因此可以使用 for in 进行遍历
    for stream in listener.incoming() {
        
        println!("Received Connection");
        // 
        let mut stream = stream.unwrap();

        // 新建一个 buffer
        // 建立一个 1024 字节长度的缓冲区
        let mut buf = [0; 1024];
        // 必须从 io 模块中引入 Read 和 Write 这两个 trait
        // stream
        let content_length = stream.read(&mut buf).unwrap();
        
        let validBuf = &buf[0..content_length];

        // 将收到的内容打印出来
        println!("{:?}", from_utf8(validBuf).unwrap());

        stream.write(&buf);

    }
}

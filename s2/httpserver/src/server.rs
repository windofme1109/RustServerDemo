// 定义基本的 Http Server

use std::net::TcpListener;
use  std::io::{Read, Write};
use http::httprequest::HttpRequest;

use super::router::Router;

pub struct Server<'a> {
    socket_addr: &'a str
}

impl<'a> Server<'a> {
    fn new(addr: &'a str) -> Self {
        Self { socket_addr: addr }
    }


    fn run(&self) {
        let listener = TcpListener::bind(self.socket_addr).unwrap();

        for stram in listener.incoming() {
            let mut stream = stram.unwrap();

            let mut read_buffer = [0; 1024];
            
            // 将获取到的字节流写入 buffer 中
            stream.read(&mut read_buffer);

            // 将 read_buffer 转化为 HttpRequest
            let req: HttpRequest = String::from_utf8(read_buffer.to_vec()).unwrap().into();

            // 在 Route 中进行处理
            Router::router(req, &mut stream);
        }
    }
}
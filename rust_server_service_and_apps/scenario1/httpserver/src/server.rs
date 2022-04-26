use super::router::Router;
use http::httprequest::HttpRequest;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;

pub struct Server<'a> {
    socket_addr: &'a str,
}
impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Server { socket_addr }
    }
    pub fn run(&self) {
        // Start a server listening on socket address
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Running on {}", self.socket_addr);
        // 수신
        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection established");
            let mut read_buffer = [0; 90];
            stream.read(&mut read_buffer).unwrap();
            let req: HttpRequest = String::from_utf8(read_buffer.to_vec()).unwrap().into();

            // Router에게 생성한 HttpRequest를 넘겨 비즈니스 로직을 처리
            Router::route(req, &mut stream);
        }
    }
}


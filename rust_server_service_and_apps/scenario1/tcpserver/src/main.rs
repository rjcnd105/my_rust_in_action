use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    // 소켓서버 초기화, 127.0.0.1:3000에 바인딩
    let connection_listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000");
    // 소켓서버는 들어오는 연결을 기다림
    for stream in connection_listener.incoming() {
        let mut _stream = stream.unwrap(); // Result<TcpStream,Error>
        println!("Connection established");
        let mut buffer = [0; 1024];
        // 읽은(read) 데이터를 그대로 클라이언트에게 돌려줌(write)
        _stream.read(&mut buffer).unwrap();
        _stream.write(&mut buffer).unwrap();
    }
}

use std::net::TcpListener;

fn main() {
    // 소켓서버 초기화, 127.0.0.1:3000에 바인딩
    let connection_listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000");
    // 소켓서버는 들어오는 연결을 기다림
    for stream in connection_listener.incoming() {
        let _stream = stream.unwrap(); // Result<TcpStream,Error>
        println!("Connection established");
    }
}

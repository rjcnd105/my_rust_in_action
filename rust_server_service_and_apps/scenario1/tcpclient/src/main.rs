use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    stream.write("Hello!".as_bytes()).unwrap();
    let mut buffer = [0; 6];
    stream.read(&mut buffer); // 서버에서 받은 바이트 읽어서 buffer에 씀
    println!(
        "God response from server:{:?}",
        str::from_utf8(&buffer).unwrap()
    )
}

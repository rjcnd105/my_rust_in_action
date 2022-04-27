mod handler;
mod server;
mod router;
use server::Server;

fn main() {
    println!("Hello, world!");

    let server = Server::new("localhost:3433");

}

//
// #[cfg(test)]
// mod tests {
//     use std::net::TcpStream;
//
//     fn stream_is_impl_Write_trait() {
//         let mut stream = TcpStream::
//     }
// }
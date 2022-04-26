mod handler;
mod server;
mod router;
use server::Server;

fn main() {
    println!("Hello, world!");

    let server = Server::new("localhost:3323");

}

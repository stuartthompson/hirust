use std::net::TcpStream;
use std::io::{Write};

const ADDR: &str = "127.0.0.1:6000";

pub fn say_hello() {
    println!("[Client] Saying hello to server.");

    let msg: &str = "Hello, Server!";
    
    println!("[Client] Connecting to server.");
    let mut stream = TcpStream::connect(ADDR).expect("[Client] Error connecting to server.");
    println!("[Client] Sending message to server.");
    stream.write(msg.as_bytes()).expect("[Client] Failed to send message to server.");
}
use std::net::TcpStream;
use std::sync::mpsc;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

pub fn say_hello() {
    println!("[Client] Saying hello to server.");

    // let mut client = TcpStream::connect(LOCAL).expect("[Client] Stream failed to connect.");
    // client.set_nonblocking(true).expect("[Client] Failed to initialize non-blocking.");

    // let tx = mpsc::channel::<String>();

    // tx.send("Hello, Server!").expect("[Client] Error sending message.");
}
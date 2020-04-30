use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

const ADDR: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 512;

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("[Server] Incoming connection from: {}", stream.peer_addr()?);
    let mut buff = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buff)?;
        if bytes_read == 0 {
            return Ok(());
        }
        stream.write(&buff[..bytes_read])?;
    }
}

pub fn start_server() {
    let server = TcpListener::bind(ADDR).expect("Listener failed to bind.");
    for stream in server.incoming() {
        match stream {
            Ok(stream) => thread::spawn(move || {
                handle_client(stream)
                    .unwrap_or_else(|error| println!("[Server] Error {:?}", error));
            }),
            Err(e) => thread::spawn(move || { eprintln!("Failed: {}", e) }),
        };
    }

    server
        .set_nonblocking(true)
        .expect("Failed to initialize server in non-blocking mode.");

    let mut server_hb = 0;

    // Live for 5 seconds
    while server_hb < 5 {
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {} connected", addr);

            let mut buff = vec![0; MSG_SIZE];

            match socket.read_exact(&mut buff) {
                Ok(_) => {
                    println!("[Server] Buffering message.");
                    // let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                    println!("[Server] Converting to utf-8");
                    // let msg = String::from_utf8(msg).expect("Invalid uft8 message.");

                    // println!("{}: {:?}", addr, msg);
                }
                Err(_) => {
                    println!("[Server] Error reading message from client.");
                }
            }
        }
        server_hb = server_hb + 1;
        println!("[Server] Heartbeat. Iteration {:?}", server_hb);

        // Sleep for one second
        thread::sleep(Duration::from_millis(1000));
    }

    println!("[Server] Stopped.");
}

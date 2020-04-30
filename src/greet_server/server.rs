use std::io::Read;
use std::net::TcpListener;
use std::thread;
use std::time::Duration;

const ADDR: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 512;

pub fn start_server() {
    let server = TcpListener::bind(ADDR).expect("Listener failed to bind.");

    server
        .set_nonblocking(true)
        .expect("Failed to initialize server in non-blocking mode.");

    let mut server_hb = 0;

    // Live for 5 seconds
    while server_hb < 5 {
        if let Ok((mut socket, addr)) = server.accept() {
            println!("[Server] Client {} connected", addr);

            let mut buff = vec![0; MSG_SIZE];

            match socket.read(&mut buff) {
                Ok(_) => {
                    let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                    let msg = String::from_utf8(msg).expect("Invalid uft8 message.");

                    println!("[Server] Client {} says: {:?}", addr, msg);
                }
                Err(e) => {
                    println!("[Server] Error reading message from client: {:?}", e);
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

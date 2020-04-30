use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const LOCAL_IP: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

pub fn start_server() {
    let server = TcpListener::bind(LOCAL_IP).expect("Listener failed to bind.");
    server.set_nonblocking(true).expect("Failed to initialize server in non-blocking mode.");

    let mut clients: Vec<std::net::TcpStream> = vec![];
    let (tx, rx) = mpsc::channel::<String>();

    let mut server_hb = 0;

    // Live for 5 seconds
    while server_hb < 5 {
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {} connected", addr);

            // let tx = tx.clone();
            // clients.push(socket.try_clone().expect("Failed to clone client."));

            // thread::spawn(move || loop {
            //     let mut buff = vec![0; MSG_SIZE];
                
            //     match socket.read_exact(&mut buff) {
            //         Ok(_) => {
            //             let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
            //             let msg = String::from_utf8(msg).expect("Invalid uft8 message.");

            //             println!("{}: {:?}", add, msg);
            //             tx.send(msg).expect("Failed to send message to receiver.");
            //         }
            //     }
            // })
        }
        server_hb = server_hb + 1;
        println!("[Server] Heartbeat. Iteration {:?}", server_hb);

        // Sleep for one second
        thread::sleep(Duration::from_millis(1000));
    }

    println!("[Server] Stopped.");
}

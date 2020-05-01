use std::sync::mpsc::{Receiver, TryRecvError};

pub fn start_listening(rx: Receiver<&str>) {
    let mut listening: bool = true;

    while listening {
        // Check for message
        match rx.try_recv() {
            Ok(msg) => {
                println!("[Channel Listener] Received message: {:?}.", msg);
                listening = false;
            }
            Err(TryRecvError::Disconnected) => {
                println!("[Channel Listener] Channel disconnected.");
                listening = false;
            }
            Err(TryRecvError::Empty) => {
            }
        }
    }
}

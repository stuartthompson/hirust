/*
    Hello, Rust!

    A simple program to learn about the Rust programming language.
*/

extern crate chrono;
extern crate json;
mod greeter;
mod greet_server;
mod greet_client;
mod parsing_greeter;

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Say hello directly
    println!("Hello, World! I'm Rust, nice to meet you.");

    // Say hello from function within a greeter module
    greeter::simple_greeter::say_hello();
    greeter::datetime_greeter::say_hello();

    // Say hello from parsed JSON
    parsing_greeter::greet_from_json();

    // Start a greeting server
    let (tx, rx) = mpsc::channel();
    let server_thread = thread::spawn(move || {
        greet_server::server::start_server(rx);
    });

    // Say hello from the client
    greet_client::client::say_hello();

    // Shut down server after a few seconds
    println!("[Main] Terminating server in 3 seconds.");
    thread::sleep(Duration::from_millis(3000));
    
    // Shutdown server
    let _ = tx.send(true);
    
    // Wait for server to quit
    server_thread.join().expect("The server thread panicked.");
}

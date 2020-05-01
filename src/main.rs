/*
    Hello, Rust!

    A simple program to learn about the Rust programming language.
*/

extern crate chrono;
extern crate json;
mod channel_greeter;
mod greet_client;
mod greet_server;
mod greeter;
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

    // Say hello to a channel listener
    let (ctx, crx) = mpsc::channel();
    let channel_thread = thread::spawn(move || {
        channel_greeter::start_listening(crx);
    });
    let _ = ctx.send("Hello!");

    // Start a greeting server
    let (stx, srx) = mpsc::channel();
    let server_thread = thread::spawn(move || {
        greet_server::server::start_server(srx);
    });

    // Say hello from the client
    greet_client::client::say_hello();

    // Shut down server after a few seconds
    println!("[Main] Terminating server in 3 seconds.");
    thread::sleep(Duration::from_millis(3000));
    // Shutdown server
    let _ = stx.send(true);

    // Wait for server to quit
    channel_thread.join().expect("The channel thread panicked.");
    server_thread.join().expect("The server thread panicked.");
}

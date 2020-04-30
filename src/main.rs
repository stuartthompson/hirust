/*
    Hello, Rust!

    A simple program to learn about the Rust programming language.
*/

extern crate chrono;
mod greeter;
mod greet_server;
mod greet_client;

use std::thread;

fn main() {
    // Say hello directly
    println!("Hello, World! I'm Rust, nice to meet you.");

    // Say hello from function within a greeter module
    greeter::simple_greeter::say_hello();
    greeter::datetime_greeter::say_hello();

    // Start a greeting server
    let server_thread = thread::spawn(|| {
        greet_server::server::start_server();
    });

    // Say hello from the client
    greet_client::client::say_hello();

    // Wait for server
    server_thread.join().expect("The server thread panicked.");
}

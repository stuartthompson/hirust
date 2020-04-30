/*
    Hello, Rust!

    A simple program to learn about the Rust programming language.
*/

mod greeter;

fn main() {
    // Say hello directly
    println!("Hello, World! I'm Rust, nice to meet you.");

    // Say hello from a function within a module
    greeter::simple_greeter::say_hello();
}

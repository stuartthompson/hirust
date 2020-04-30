/*
 * Says hello with an included timestamp.
 */
pub fn say_hello() {
    let current_time = chrono::offset::Utc::now();

    println!(
        "Hello, World! From a module! It's currently {:?} (UTC)",
        current_time
    );
}

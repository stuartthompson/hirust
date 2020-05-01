use json::JsonValue;

const GREETING: &str = "{ \"greeting\": \"Hello, from JSON!\" }";

pub fn greet_from_json() {
    let parsed: JsonValue = json::parse(GREETING).unwrap();
    println!("{}", parsed["greeting"]);
}

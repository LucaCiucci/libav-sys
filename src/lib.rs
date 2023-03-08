

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn hello_there() {
    println!("Hello, there!");
}
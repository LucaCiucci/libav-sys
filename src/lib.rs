

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn hello_there() {
    println!("Hello, there!");
}
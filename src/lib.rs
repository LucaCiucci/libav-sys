

pub mod aaa {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
pub fn main() {
    println!("Hello, world!");

    use aaa::avcodec as avc;

    unsafe {
        aaa::avformat::avformat_network_init();
    }
}

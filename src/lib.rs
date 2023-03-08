//! Rust bindings for system libav libraries
//! 
//! This crate searches for system installation of ffmpeg, it tries
//! to generate bindings for all the 

#![allow(deprecated)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(clashing_extern_declarations)]


/// This module contains the generated bindings for the system libav libraries.
/// 
/// 
/// # Warning
/// Documentation for this crate cannot be generated on [`docs.rs`](//docs.rs) because it requires
/// a system installation of ffmpeg.
/// 
/// Also, the bindings may be different depending on the particular version of ffmpeg installed.
/// 
/// To generate the documentation locally, you can use the `cargo rustdoc --open` command.
pub mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}


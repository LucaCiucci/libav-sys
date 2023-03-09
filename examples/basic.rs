
use std::{os::raw::c_char, error::Error};
use libav_sys::ffi::{
    avformat_open_input,
    AVFormatContext,
    avformat_network_init,
    av_strerror,
    avformat_find_stream_info,
    av_dump_format
};

type MyResult = Result<(), Box<dyn Error>>;


fn main() -> MyResult {
    println!("Hello, world!");

    init_libav()?;

    //let url = r#"file:C:/Users/lucac/downloads/BigBuckBunny.mp4"#;
    let url = r#"http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4"#;

    // convert url to c string
    let c_url = std::ffi::CString::new(url)?;

    // we will store the av format context here
    let mut format_context_ptr: *mut AVFormatContext = std::ptr::null_mut();

    // try to open the input
    handle_libav_result(
        unsafe {
            avformat_open_input(
                &mut format_context_ptr,
                c_url.as_ptr(),
                std::ptr::null_mut(),
                std::ptr::null_mut()
            )
        },
        "avformat_open_input failed"
    )?;

    println!("Input opened!");
    println!("format_context_ptr: {:?}", format_context_ptr);

    // find the stream info
    handle_libav_result(
        unsafe {
            avformat_find_stream_info(
                format_context_ptr,
                std::ptr::null_mut()
            )
        },
        "failed to find stream info"
    )?;

    // dumpo fomat info to console
    unsafe {
        av_dump_format(
            format_context_ptr,
            0,
            c_url.as_ptr() as *const c_char,
            0
        );
    }

    deinit_libav()?;

    Ok(())
}

fn init_libav() -> MyResult {
    // TODO ??? libav_sys::ffi::av_register_all();

    handle_libav_result(
        unsafe { avformat_network_init() },
        "avformat_network_init failed"
    )
}

fn deinit_libav() -> MyResult {
    handle_libav_result(
        unsafe { libav_sys::ffi::avformat_network_deinit() },
        "avformat_network_deinit failed"
    )
}

/// Convert an error code to a string
fn av_error_to_string(error_code: std::os::raw::c_int) -> String {
    // create a buffer for the error message
    let mut error_buffer: Vec<std::os::raw::c_char> = vec![0; 1024];

    // get the error message
    let result = unsafe {
        av_strerror(
            error_code,
            error_buffer.as_mut_ptr(),
            error_buffer.len()
        )
    };
    if result != 0 {
        panic!("av_strerror failed");
    }

    // convert to rust string
    error_buffer.iter().take_while(|c| **c != 0).map(|c| *c as u8 as char).collect()
}

/// Handle a result from a libav function
fn handle_libav_result(result: std::os::raw::c_int, error_message: &str) -> MyResult {
    if result < 0 {
        return Err(format!("{}: {}", error_message, av_error_to_string(result)).into());
    }

    Ok(())
}

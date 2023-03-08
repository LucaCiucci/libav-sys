# libav-sys

Bindings to libav for Rust.

This crate provides bindings for the libraries shipped with a
system installation of ffmpeg.

In order to make this work, you have to manually install ffmpeg and set the
`FFMPEG_DIR` environment variable or install ffmpeg using a package manager.

## Examples

You can find some usage examples in the [`examples/`](./examples/) folder.

## TODOs

- [x] find ffmpeg using `FFMPEG_DIR`
- [ ] find ffmpeg using [`system-deps`](https://crates.io/crates/system-deps) like in [this example](https://github.com/rust-av/libav-rs/tree/master/libav-sys)
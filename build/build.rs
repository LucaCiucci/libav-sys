use std::{path::{Path, PathBuf}, fmt::{Debug}, error::Error, fs::create_dir_all};

use bindgen::callbacks::ParseCallbacks;

pub mod env_install;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // special case for docs.rs
    if std::env::var("DOCS_RS").is_ok() {
        return empty_bindings();
    }
    
    let suite = find_suite()?;

    add_linker_search_dir(&suite.lib_dir());

    for lib in suite.lib_names() {
        link_lib(&lib);
    }

    bindgen_for_suite(suite.as_ref())?;

    Ok(())
}

/// Generate an empty bindings module.
/// 
/// This is used when the crate is being built on docs.rs, because docs.rs
/// does not have access to the system ffmpeg installation.
fn empty_bindings() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);
    let bindings_module_path = out_dir.join("bindings.rs");
    std::fs::write(&bindings_module_path, "")?;
    Ok(())
}

pub trait FfmpegSuite: Debug {
    fn include_dir(&self) -> PathBuf;
    fn lib_dir(&self) -> PathBuf;
    fn lib_names(&self) -> Vec<String>;
    fn main_include_for_lib(&self, lib: &str) -> String;
}

fn find_suite() -> Result<Box<dyn FfmpegSuite>, Box<dyn std::error::Error>> {
    let mut not_found_reasons: Vec<String> = Vec::new();

    if let Ok(suite) = env_install::find() {
        return Ok(suite);
    } else if let Err(err) = env_install::find() {
        not_found_reasons.push(format!("env_install not found: {}", err));
    }

    Err(format!("No FFMPEG suite found: \n{}", not_found_reasons.join("\n\n")).into())
}

fn add_linker_search_dir(dir: &Path) {
    println!("cargo:rustc-link-search=native={}", dir.to_string_lossy());
}

fn link_lib(lib: &str) {
    println!("cargo:rustc-link-lib=dylib={}", lib);
}

fn bindgen_for_suite(suite: &dyn FfmpegSuite) -> Result<(), Box<dyn Error>> {

    let clang_arg = format!("-I{}", suite.include_dir().to_string_lossy());

    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);

    let bindings_outdir = out_dir.clone();
    create_dir_all(&bindings_outdir)?;

    let mut builder = bindgen::Builder::default()
            .clang_arg(&clang_arg)
            .parse_callbacks(Box::new(CommentProcessor));

    for lib in suite.lib_names() {
        let header = suite.include_dir()
            .join(format!("lib{lib}"))
            .join(suite.main_include_for_lib(&lib));

        if !header.exists() {
            return Err(format!("Could not find {header:?}").into());
        }

        builder = builder.header(header.to_string_lossy());

        //let module_name = lib;
        //let module_path = bindings_outdir.join(format!("{module_name}.rs"));

        //bindings_modules.push(module_name);
    }

    let bindings = builder
        // see https://doc.rust-lang.org/beta/rustc/lints/listing/warn-by-default.html
        // see https://github.com/rust-av/libav-rs/blob/master/libav-sys/build.rs
        //.raw_line("#![allow(deprecated)]")
        //.raw_line("#![allow(dead_code)]")
        //.raw_line("#![allow(non_camel_case_types)]")
        //.raw_line("#![allow(non_snake_case)]")
        //.raw_line("#![allow(non_upper_case_globals)]")
        //.raw_line("#![allow(clashing_extern_declarations)]")
        .generate()
        .expect("Unable to generate bindings");

    let bindings_module_path = out_dir.join("bindings.rs");

    bindings
        .write_to_file(bindings_module_path)
        .expect("Couldn't write bindings!");

    //std::fs::write(&bindings_module_path, module_content)?;
    //panic!("module_path: {:?}\n{:?}\n{:?}", out_dir, out_dir.join("bindings.rs"), bindings_module_path);

    //panic!("bindings: {:?}", 1);

    Ok(())
}


#[derive(Debug)]
struct CommentProcessor;

impl ParseCallbacks for CommentProcessor {
    fn process_comment(&self, comment: &str) -> Option<String> {
        // TODO doesn't work really well...
        Some(doxygen_rs::transform(comment))
    }
    //fn item_name(&self, _original_item_name: &str) -> Option<String> {
    //    return Some("aaa::".to_string() + _original_item_name);
    //}
}
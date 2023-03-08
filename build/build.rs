use std::{path::{Path, PathBuf}, fmt::{Debug}, error::Error, fs::create_dir_all};

pub mod env_install;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let suite = find_suite()?;

    add_linker_search_dir(&suite.lib_dir());

    for lib in suite.lib_names() {
        link_lib(&lib);
    }

    bindgen_for_suite(suite.as_ref())?;

    //panic!("r:");

    Ok(())
}

pub trait FfmpegSuite: Debug {
    fn include_dir(&self) -> PathBuf;
    fn lib_dir(&self) -> PathBuf;
    fn lib_names(&self) -> Vec<String>;
    fn main_include_for_lib(&self, lib: &str) -> String {
        format!("{}.h", lib)
    }
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

    //let bindings_outdir = out_dir.join("bindings");
    let bindings_outdir = out_dir.clone();
    create_dir_all(&bindings_outdir)?;

    let mut bindings_modules: Vec<String> = Vec::new();
    
    for lib in suite.lib_names() {
        let header = suite.include_dir()
            .join(format!("lib{lib}"))
            .join(suite.main_include_for_lib(&lib));

        if !header.exists() {
            return Err(format!("Could not find {header:?}").into());
        }

        let builder = bindgen::Builder::default()
            .clang_arg(&clang_arg)
            .header(header.to_string_lossy());

        let bindings = builder
            // see https://doc.rust-lang.org/beta/rustc/lints/listing/warn-by-default.html
            // see https://github.com/rust-av/libav-rs/blob/master/libav-sys/build.rs
            .raw_line("#![allow(deprecated)]")
            .raw_line("#![allow(dead_code)]")
            .raw_line("#![allow(non_camel_case_types)]")
            .raw_line("#![allow(non_snake_case)]")
            .raw_line("#![allow(non_upper_case_globals)]")
            .raw_line("#![allow(clashing_extern_declarations)]")
            .generate()
            .expect("Unable to generate bindings");

        let module_name = lib;
        let module_path = bindings_outdir.join(format!("{module_name}.rs"));

        bindings
            .write_to_file(module_path)
            .expect("Couldn't write bindings!");

        bindings_modules.push(module_name);
    }

    let module_content: String = bindings_modules.iter()
        .map(|module_name| format!("pub mod {module_name};"))
        .collect::<Vec<String>>()
        .join("\n");

    let bindings_module_path = out_dir.join("bindings.rs");
    std::fs::write(&bindings_module_path, module_content)?;
    //panic!("module_path: {:?}\n{:?}\n{:?}", out_dir, out_dir.join("bindings.rs"), bindings_module_path);

    //panic!("bindings: {:?}", 1);

    Ok(())
}
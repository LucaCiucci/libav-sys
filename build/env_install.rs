use std::{env, path::PathBuf};

use crate::FfmpegSuite;

#[derive(Debug)]
pub struct EnvInstallSuite {
    dir: PathBuf,
    include_dir: PathBuf,
    lib_dir: PathBuf,
}

impl EnvInstallSuite {
    pub fn ffmpeg_dir(&self) -> &PathBuf {
        &self.dir
    }

    pub fn include_subdirs(&self) -> Vec<PathBuf> {
        std::fs::read_dir(&self.include_dir)
            .expect("Failed to read include dir")
            // discard invalid entries and convert to paths
            .filter_map(|entry| if let Ok(entry) = entry { Some(entry.path()) } else { None })
            // discard non-directories
            .filter(|path| path.is_dir())
            .collect()
    }
}

impl FfmpegSuite for EnvInstallSuite {
    fn include_dir(&self) -> PathBuf {
        self.include_dir.clone()
    }

    fn lib_dir(&self) -> PathBuf {
        self.lib_dir.clone()
    }

    fn lib_names(&self) -> Vec<String> {
        let subdirs = self.include_subdirs();

        const PREFIX: &str = "lib";

        subdirs.iter()
            .map(|lib_include_dir| lib_include_dir.file_name().unwrap().to_string_lossy())
            .filter(|lib| lib.starts_with(PREFIX))
            .map(|lib| lib[PREFIX.len()..].to_string())
            .collect()
    }

    fn main_include_for_lib(&self, lib: &str) -> String {
        if lib == "postproc" {
            return "postprocess.h".into();
        }

        format!("{}.h", lib)
    }
}

pub fn find() -> Result<Box<dyn FfmpegSuite>, Box<dyn std::error::Error>> {
    let dir: PathBuf = env::var("FFMPEG_DIR")
        .map_err(|err| format!("Error getting FFMPEG_DIR: {}", err))?
        .into();

    let include_dir = dir.join("include");
    if !include_dir.exists() {
        return Err(format!(
            "FFMPEG_DIR/include does not exist: {}",
            include_dir.display()
        )
        .into());
    }

    let lib_dir = dir.join("lib");
    if !lib_dir.exists() {
        return Err(format!(
            "FFMPEG_DIR/lib does not exist: {}",
            lib_dir.display()
        )
        .into());
    }

    Ok(Box::new(EnvInstallSuite {
        dir,
        include_dir,
        lib_dir,
    }))
}
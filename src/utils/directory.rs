use std::{fs, path::PathBuf};

use directories::ProjectDirs;

pub struct Directory;

impl Directory {
    pub fn local_data_dir() -> Option<PathBuf> {
        match ProjectDirs::from("dev", "flow", "Flow-App") {
            Some(p) => {
                let dir = p.data_local_dir();
                if !dir.exists() {
                    if let Err(e) = fs::create_dir_all(dir) {
                        eprintln!("{}", e.to_string());
                    }
                }
                Some(dir.to_path_buf())
            }
            None => None,
        }
    }

    pub fn get_local_dir(dirname: &str) -> Option<PathBuf> {
        match Self::local_data_dir() {
            Some(p) => {
                let dir = p.join(dirname);
                if !dir.exists() {
                    if let Err(e) = fs::create_dir(&dir) {
                        eprintln!("{}", e.to_string());
                    }
                }
                Some(dir)
            }
            None => None,
        }
    }
}

use std::{fs, path::PathBuf};

use directories::ProjectDirs;

use crate::utils::get_current_exe;

pub struct Directory;

impl Directory {
    pub fn local_data_dir() -> PathBuf {
        match ProjectDirs::from("dev", "flow", "Flow-App") {
            Some(p) => {
                let dir = p.data_local_dir();
                if !dir.exists() {
                    if let Err(e) = fs::create_dir_all(dir) {
                        eprintln!("{}", e);
                    }
                }
                dir.to_path_buf()
            }
            None => {
                // Create a folder in the current folder!
                let mut app_dir = get_current_exe().unwrap();
                app_dir.push(".flow");
                if !app_dir.exists() {
                    if let Err(e) = fs::create_dir(&app_dir) {
                        eprintln!("{}", e);
                    }
                }
                app_dir
            }
        }
    }

    pub fn get_local_dir(dirname: &str) -> Option<PathBuf> {
        let mut path_dir = Self::local_data_dir();
        path_dir.push(dirname);
        if !path_dir.exists() {
            if let Err(e) = fs::create_dir(&path_dir) {
                eprintln!("{}", e);
            }
        }
        Some(path_dir)
    }
}

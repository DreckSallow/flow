use std::{fs, path::PathBuf};

use crate::directory::Directory;

pub fn current_project_path() -> PathBuf {
    let mut path_current_project = Directory::local_data_dir();
    path_current_project.push("current-project.txt");

    if !path_current_project.exists() {
        let _ = fs::File::create(&path_current_project);
    }
    path_current_project
}

pub fn current_project() -> u32 {
    let current_path = current_project_path();
    let first_line = fs::read_to_string(&current_path)
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .get(0)
        .map(|s| s.to_string());

    match first_line {
        Some(id) => id.trim().to_string().parse().unwrap_or(1),
        None => {
            fs::write(current_path, "1").unwrap();
            1
        } // The first project inserted!!!
    }
}

pub fn switch_current_project(id: u32) -> Result<(), std::io::Error> {
    let project_path = current_project_path();
    fs::write(project_path, id.to_string())
}

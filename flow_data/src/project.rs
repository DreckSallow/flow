use std::path::PathBuf;

use serde::Serialize;

use crate::task::Task;

#[derive(Debug, Serialize)]
pub struct Project {
    pub id: u32,
    pub path: PathBuf,
    pub tasks: Vec<Task>,
}

impl Project {
    pub fn new(id: u32, path: PathBuf, tasks: Vec<Task>) -> Self {
        Self { id, path, tasks }
    }
}

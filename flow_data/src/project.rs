use std::path::PathBuf;

use crate::task::Task;

#[derive(Debug)]
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

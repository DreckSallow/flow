use std::path::PathBuf;

pub struct ProjectParams {
    pub new: bool,
    pub path: PathBuf,
}

impl ProjectParams {
    pub fn new(new: bool, path: PathBuf) -> Self {
        ProjectParams { new, path }
    }
}

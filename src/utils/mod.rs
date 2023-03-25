use std::{
    env, io,
    path::{Path, PathBuf},
};

pub mod directory;
pub mod table;

pub mod test_utils;

pub fn get_current_directory() -> io::Result<PathBuf> {
    env::current_dir()
}

pub fn canonicalize_path<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
    const VERBATIM_PREFIX: &str = r#"\\?\"#;
    let p = path.as_ref().canonicalize()?.display().to_string();
    let slice_path = if p.starts_with(VERBATIM_PREFIX) {
        p[VERBATIM_PREFIX.len()..].to_string()
    } else {
        p
    };
    let mut new_path = PathBuf::new();
    new_path.push(slice_path);
    Ok(new_path)
}

pub mod config;
mod db_model;
mod project;
mod test;
pub use project::Project;

use self::db_model::ProjectModelUtils;
use crate::{db::Db, utils};
pub use config::ProjectParams;

use std::fs;

pub struct ProjectProgram;

impl ProjectProgram {
    pub fn project_run(params: ProjectParams, db: &Db) {
        let projects = match ProjectModelUtils::get_projects_db(db) {
            Ok(projects) => projects,
            Err(e) => return eprintln!("Not found Projects: {}", e),
        };

        let path_complete = match utils::get_current_directory() {
            Err(e) => {
                return eprintln!("{}", e.to_string());
            }
            Ok(mut p) => {
                p.push(&params.path);
                p
            }
        };

        if params.new && !path_complete.exists() {
            match fs::create_dir(&path_complete) {
                Ok(()) => {
                    println!("folder created!: {}", path_complete.display());
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        }

        let path_complete = match utils::canonicalize_path(&params.path) {
            Ok(p) => p,
            Err(e) => return eprintln!("{}", e),
        };

        if projects.contains_key(&path_complete.display().to_string()) {
            return eprintln!(
                "{}",
                format!("The path {:?} already exists!", path_complete)
            );
        }

        if let Err(e) = ProjectModelUtils::create_project(db, &path_complete) {
            return eprintln!("{:?}", e);
        }
    }
}

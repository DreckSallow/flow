pub mod config;
mod db_model;
mod project;
mod test;
pub use project::Project;

use self::db_model::ProjectModelUtils;
use crate::{
    db::Db,
    utils::{self, table},
};
pub use config::ProjectParams;

use std::fs;

pub struct ProjectProgram;

impl ProjectProgram {
    pub fn run_default(params: ProjectParams, db: &Db) {
        let path_complete = match utils::get_current_directory() {
            Err(e) => return eprintln!("{}", e.to_string()),
            Ok(mut p) => {
                p.push(&params.path);
                p
            }
        };
        if params.new && !path_complete.exists() {
            match fs::create_dir(&path_complete) {
                Ok(()) => println!("Folder created!: {}", path_complete.display()),
                Err(e) => eprintln!("{}", e),
            }
        }
        if !params.new && !path_complete.exists() {
            return eprintln!("This folder not exist locally");
        }

        if !path_complete.is_dir() {
            return eprintln!("This is not a folder");
        }

        let path_complete = match utils::canonicalize_path(&params.path) {
            Ok(p) => p,
            Err(e) => return eprintln!("{}", e),
        };

        let projects = match ProjectModelUtils::get_projects(db) {
            Ok(projects) => projects,
            Err(e) => return eprintln!("Error getting the Projects: {}", e),
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

    pub fn run_list(db: &Db) {
        let projects = match ProjectModelUtils::get_projects(db) {
            Ok(projects) => projects,
            Err(e) => return eprintln!("Error getting the Projects: {}", e),
        };
        let mut table_format = table::Table::new();
        table_format.add_headers(vec!["Id", "Path"]);
        for (_, p) in projects.iter() {
            table_format.insert_row(vec![
                table::RowCell::Single(p.id.to_string()),
                table::RowCell::Single(p.path.display().to_string()),
            ]);
        }
        println!("{}", table_format.get_table(1));
    }
}

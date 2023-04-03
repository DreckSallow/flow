pub mod config;
pub mod db_model;
mod project;
mod test;
pub use project::Project;

use self::db_model::ProjectModelUtils;
use crate::{
    app_data::AppData,
    utils::{self, table},
};
pub use config::ProjectParams;

use std::fs;

pub struct ProjectProgram;

impl ProjectProgram {
    pub fn run_default(params: ProjectParams, app_data: &AppData) {
        let path_complete = match utils::get_current_directory() {
            Err(e) => return eprintln!("{}", e.to_string()),
            Ok(mut p) => {
                p.push(&params.path);
                p
            }
        };
        if params.new && !path_complete.exists() {
            match fs::create_dir(&path_complete) {
                Ok(()) => println!("Folder created!: {:?}", path_complete),
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

        let projects = match ProjectModelUtils::get_projects(&app_data.db) {
            Ok(projects) => projects,
            Err(e) => return eprintln!("Error getting the Projects: {}", e),
        };

        if projects.contains_key(&path_complete.display().to_string()) {
            return eprintln!(
                "{}",
                format!("The path {:?} already exists!", path_complete)
            );
        }

        if let Err(e) = ProjectModelUtils::create_project(&app_data.db, &path_complete) {
            return eprintln!("{:?}", e);
        }
    }

    pub fn run_list(app_data: &AppData) {
        let projects = match ProjectModelUtils::get_projects(&app_data.db) {
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
    pub fn remove_project(app_data: &AppData, id: u32) {
        let project_finded = match ProjectModelUtils::get_projects(&app_data.db) {
            Ok(p) => p.into_values().find(|p| p.id == id),
            Err(_) => return eprintln!("Error tryng to get the projects"),
        };

        if project_finded.is_none() {
            return eprintln!("The project with id: {} not exist.", id);
        }

        if id == 1 {
            return eprintln!("Cannot delete the main project.");
        }

        match ProjectModelUtils::remove_project(&app_data.db, id) {
            Ok(_) => println!(
                "Deleted Project: {}",
                project_finded.unwrap().path.display()
            ),
            Err(_) => eprintln!("Error removing the project."),
        }
    }

    pub fn run_switch(app_data: &AppData, id: u32) {
        let project = match ProjectModelUtils::get_by_id(&app_data.db, id) {
            Ok(p) => p,
            Err(_) => return eprintln!("The project with id: {} not exist", id),
        };

        match utils::data::switch_current_project(project.id) {
            Ok(_) => {
                println!("Change the current project to : {}", project.path.display());
            }
            Err(_) => {
                println!("It was not possible to make the change");
            }
        }
    }
}

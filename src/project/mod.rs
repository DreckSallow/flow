use crossterm::{style::Stylize, tty::IsTty};

use flow_data::{app_data::AppData, data, db::project_model::ProjectModelUtils, utils};

use crate::utils::table;

use std::{fs, io::stdout, path::Path};

pub struct ProjectProgram;

impl ProjectProgram {
    pub fn run_default<P: AsRef<Path>>(new: bool, path: P, app_data: &AppData) {
        let path_complete = match utils::get_current_directory() {
            Err(e) => return eprintln!("{}", e.to_string()),
            Ok(mut p) => {
                p.push(&path);
                p
            }
        };
        if new && !path_complete.exists() {
            match fs::create_dir(&path_complete) {
                Ok(()) => println!("Folder created!: {:?}", path_complete),
                Err(e) => eprintln!("{}", e),
            }
        }
        if !new && !path_complete.exists() {
            return eprintln!("This folder not exist locally");
        }

        if !path_complete.is_dir() {
            return eprintln!("This is not a folder");
        }

        let path_complete = match utils::canonicalize_path(&path) {
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
            let (id_cell, path_cell) = if p.id == app_data.current_project_id && stdout().is_tty() {
                let id = p.id.to_string().green().to_string();
                let path = p.path.display().to_string().green().to_string();
                (
                    table::RowCell::Styled(id, p.id.to_string()),
                    table::RowCell::Styled(path, p.path.display().to_string()),
                )
            } else {
                (
                    table::RowCell::Single(p.id.to_string()),
                    table::RowCell::Single(p.path.display().to_string()),
                )
            };

            table_format.insert_row(vec![id_cell, path_cell]);
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

        match data::switch_current_project(project.id) {
            Ok(_) => {
                if stdout().is_tty() {
                    println!(
                        "Change the current project to : {}",
                        project.path.display().to_string().green()
                    );
                } else {
                    println!("Change the current project to : {}", project.path.display());
                }
            }
            Err(_) => {
                println!("It was not possible to make the change");
            }
        }
    }
    pub fn run_use(app_data: &AppData) {
        let current_path = utils::get_current_directory().unwrap();

        let projects = match ProjectModelUtils::get_projects(&app_data.db) {
            Ok(list) => list,
            Err(_) => return eprintln!("Error getting the projects"),
        };

        let project = match projects.get(&current_path.display().to_string()) {
            Some(p) => p,
            None => {
                return eprintln!(
                    "The current path: {}, not is a project saved!\nCreate the project before!",
                    current_path.display()
                )
            }
        };

        match data::switch_current_project(project.id) {
            Ok(_) => println!(
                "Set the path: {} as current project",
                current_path.display()
            ),
            Err(_) => eprintln!("Error setting the path as current project"),
        }
    }
}

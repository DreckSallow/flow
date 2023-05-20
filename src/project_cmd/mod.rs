use anyhow::{anyhow, Result};
use crossterm::{style::Stylize, tty::IsTty};

use flow_data::{app_data::AppData, data, db::project::project_utils, utils};

use crate::utils::table;

use std::{fs, io::stdout, path::Path};

pub fn create_project<P: AsRef<Path>>(new: bool, path: P, app_data: &AppData) -> Result<()> {
    let mut path_complete = utils::get_current_directory()?;
    path_complete.push(&path);

    if new {
        if path_complete.exists() {
            return Err(anyhow!("The folder already exists"));
        }
        fs::create_dir(&path_complete)?;
        println!("Folder created: {:?}", path_complete);
    }

    if !new && !path_complete.exists() {
        return Err(anyhow!("This folder not exist locally"));
    }

    if !path_complete.is_dir() {
        return Err(anyhow!("This is not a folder"));
    }

    let path_complete = utils::canonicalize_path(path)?;
    let projects = project_utils::get_projects(&app_data.db)?;
    if projects.contains_key(&path_complete.display().to_string()) {
        return Err(anyhow!("The path {:?} already exists!", path_complete));
    }

    project_utils::create_project(&app_data.db, path_complete)?;

    Ok(())
}

pub fn list_projects(app_data: &AppData) -> Result<()> {
    let projects = project_utils::get_projects(&app_data.db)?;

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
    Ok(())
}

pub fn remove_project(app_data: &AppData, id: u32) -> Result<()> {
    if id == 1 {
        return Err(anyhow!("Cannot delete the main project"));
    }
    if app_data.current_project_id == id {
        return Err(anyhow!("Cannot delete the current project!"));
    }

    let project_finded = project_utils::get_projects(&app_data.db)?
        .into_values()
        .find(|p| p.id == id);

    match project_finded {
        Some(p) => {
            println!("project finded: {:?}", p);
            project_utils::remove_project(&app_data.db, id)?;
            println!("Deleted Project: {}", p.path.display());
            Ok(())
        }
        None => {
            return Err(anyhow!("The project with id: {} not exist.", id));
        }
    }
}

pub fn swicth_current_project(app_data: &AppData, id: u32) -> Result<()> {
    let project = project_utils::get_by_id(&app_data.db, id)?;

    data::switch_current_project(id)?;
    if stdout().is_tty() {
        println!(
            "Change the current project to : {}",
            project.path.display().to_string().green()
        );
    } else {
        println!("Change the current project to : {}", project.path.display());
    }
    Ok(())
}

pub fn read_path_to_switch(app_data: &AppData) -> Result<()> {
    let current_path = utils::get_current_directory().unwrap();
    let projects = project_utils::get_projects(&app_data.db)?;

    let project = match projects.get(&current_path.display().to_string()) {
        Some(p) => p,
        None => {
            return Err(anyhow!(
                "The current path: {}, not is a project saved!\nCreate the project before!",
                current_path.display()
            ))
        }
    };

    data::switch_current_project(project.id)?;
    println!(
        "Set the path: {} as current project",
        current_path.display()
    );
    Ok(())
}

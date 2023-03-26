use clap::{command, Parser, Subcommand};
use rusqlite::Error;
use std::path::PathBuf;

use crate::{
    app_data::AppData,
    db::Db,
    project::{ProjectParams, ProjectProgram},
    utils,
};
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Project is a subcommand to manage projects locally
    Project {
        #[arg(short, long)]
        new: bool,
        path: Option<PathBuf>,
        #[command(subcommand)]
        command: Option<ProjectCommands>,
    },
    /// Task is a subcommand to manage task related to the current project
    Task {
        /// Description of the task
        #[arg(short, long)]
        description: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum ProjectCommands {
    /// List all projects
    List,
    Switch {
        path: PathBuf,
    },
}

pub struct App;

#[derive(Debug)]
pub enum AppError {
    DbConnection(Error),
}

impl App {
    pub fn run() -> Result<(), AppError> {
        let cli = Cli::parse();
        let db = match Db::open() {
            Ok(conn) => conn,
            Err(e) => return Err(AppError::DbConnection(e)),
        };
        let current_project_id = utils::data::current_project();
        let app_data = AppData::new(db, current_project_id);

        match cli.command {
            Commands::Task { description } => {
                println!("description: {}", description);
                Ok(())
            }
            Commands::Project { new, path, command } => {
                if let Some(c) = command {
                    match c {
                        ProjectCommands::List => ProjectProgram::run_list(&app_data),
                        ProjectCommands::Switch { path } => {
                            ProjectProgram::run_switch(&app_data, path)
                        }
                    }
                } else if let Some(p) = path {
                    ProjectProgram::run_default(ProjectParams::new(new, p), &app_data);
                } else {
                    println!("The current project is: {}", app_data.current_project_id);
                }
                Ok(())
            }
        }
    }
}

use clap::{command, Parser, Subcommand};
use rusqlite::Error;
use std::path::PathBuf;

use crate::{
    app_data::AppData,
    db::Db,
    project::{ProjectParams, ProjectProgram},
    task::TaskProgram,
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
        description: Option<String>,
        #[command(subcommand)]
        command: Option<TaskCommands>,
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

#[derive(Subcommand, Debug)]
pub enum TaskCommands {
    ///List all tasks related to the current project
    List {
        /// Expand the table output adding more data
        #[arg(short, long)]
        expand: bool,
        ///Order by: "number" | "desc"
        #[arg(short, long)]
        order_by: Option<String>,
    },
    /// Remove a task by 'N-Id' column
    Rm { id: u32 },
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
            Commands::Task {
                description,
                command,
            } => {
                if let Some(c) = command {
                    match c {
                        TaskCommands::List { expand, order_by } => {
                            TaskProgram::run_list(&app_data, expand, order_by)
                        }
                        TaskCommands::Rm { id } => TaskProgram::run_delete(&app_data, id),
                    }
                } else if let Some(desc) = description {
                    TaskProgram::run_default(&app_data, &desc);
                }

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

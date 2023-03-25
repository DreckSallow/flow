use clap::{command, Parser, Subcommand};
use rusqlite::Error;
use std::path::PathBuf;

use crate::{
    db::Db,
    project::{ProjectParams, ProjectProgram},
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

        match cli.command {
            Commands::Task { description } => {
                println!("description: {}", description);
                Ok(())
            }
            Commands::Project { new, path, command } => {
                if let Some(c) = command {
                    match c {
                        ProjectCommands::List => ProjectProgram::run_list(&db),
                    }
                } else if let Some(p) = path {
                    ProjectProgram::run_default(ProjectParams::new(new, p), &db);
                }
                Ok(())
            }
        }
    }
}

use clap::{command, Parser, Subcommand};
use crossterm::{
    style::{Print, Stylize},
    tty::IsTty,
};
use std::{io::stdout, path::PathBuf};
use thiserror::Error;

use flow_api;

use flow_data::{
    app_data::AppData,
    data::current_project,
    db::{project::project_utils, Db},
    task::TaskStatus,
    Error,
};

use crate::{constants::FLOW_CLI_NAME, project_cmd, task_cmd};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Manage projects locally
    Project {
        /// Create a folder for the project
        #[arg(short, long)]
        new: bool,
        /// Local project path
        path: Option<PathBuf>,
        #[command(subcommand)]
        command: Option<ProjectCommands>,
    },
    /// Manage tasks related to the current project
    Task {
        /// Description of the task
        #[arg(short, long)]
        description: Option<String>,
        #[command(subcommand)]
        command: Option<TaskCommands>,
    },
    Preview,
}

#[derive(Subcommand, Debug)]
pub enum ProjectCommands {
    /// List all projects
    List,
    /// Set the current project, and get related tasks
    Switch {
        /// Project <ID>
        id: u32,
    },
    /// Remove a project by Id
    Rm {
        ///Project <ID>
        id: u32,
    },
    /// Get the path and set as current project
    Use,
}

#[derive(Subcommand, Debug)]
pub enum TaskCommands {
    ///List all tasks related to the current project
    List {
        /// Expand the table output adding more info
        #[arg(short, long)]
        expand: bool,
        /// Add colors to table
        #[arg(short, long)]
        color: bool,
        ///Order by: "number" | "desc"
        #[arg(short, long)]
        order_by: Option<String>,
    },
    /// Remove a task
    Rm {
        ///Task <N-ID>
        id: u32,
    },
    /// Mark task as In Progress
    Start {
        ///Task <N-ID>
        id: u32,
    },
    /// Mark task as stopped
    Stop {
        ///Task <N-ID>
        id: u32,
    },
    /// Mark tasks as completed
    Done {
        ///Task <N-IDS>
        ids: Vec<u32>,
    },
    /// Mark tasks as Not Started
    Reset {
        ///Task <N-IDS>
        ids: Vec<u32>,
    },
}

pub struct App;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Error connecting with the storage {0}")]
    DbConnection(Error),
}

impl App {
    pub fn run() -> Result<(), AppError> {
        let cli = Cli::parse();
        let db = match Db::open() {
            Ok(conn) => conn,
            Err(e) => return Err(AppError::DbConnection(e)),
        };
        let current_project_id = current_project();
        let app_data = AppData::new(db, current_project_id);

        match cli.command {
            Some(command) => match command {
                Commands::Task {
                    description,
                    command,
                } => {
                    if let Some(c) = command {
                        let err = match c {
                            TaskCommands::List {
                                expand,
                                order_by,
                                color,
                            } => task_cmd::list_tasks(&app_data, expand, order_by, color),
                            TaskCommands::Rm { id } => task_cmd::delete_one(&app_data, id),
                            TaskCommands::Start { id } => task_cmd::start_one_task(&app_data, id),
                            TaskCommands::Stop { id } => {
                                task_cmd::update_task_status(&app_data, vec![id], TaskStatus::Stop)
                            }
                            TaskCommands::Done { ids } => {
                                task_cmd::update_task_status(&app_data, ids, TaskStatus::Done)
                            }
                            TaskCommands::Reset { ids } => {
                                task_cmd::update_task_status(&app_data, ids, TaskStatus::NoStarted)
                            }
                        };
                        if let Err(e) = err {
                            eprintln!("Error: {:?}", e);
                        }
                    } else if let Some(desc) = description {
                        if let Err(e) = task_cmd::create_task(&app_data, &desc) {
                            eprintln!("Error: {:?}", e);
                        }
                    }

                    Ok(())
                }
                Commands::Project { new, path, command } => {
                    match command {
                        Some(c) => {
                            let res = match c {
                                ProjectCommands::List => project_cmd::list_projects(&app_data),
                                ProjectCommands::Switch { id } => {
                                    project_cmd::swicth_current_project(&app_data, id)
                                }
                                ProjectCommands::Rm { id } => {
                                    project_cmd::remove_project(&app_data, id)
                                }
                                ProjectCommands::Use => project_cmd::read_path_to_switch(&app_data),
                            };
                            if let Err(e) = res {
                                eprintln!("{:?}", e);
                            }
                        }
                        None => {
                            if let Some(p) = path {
                                if let Err(e) = project_cmd::create_project(new, p, &app_data) {
                                    eprintln!("{:?}", e)
                                }
                            } else {
                                match project_utils::get_by_id(
                                    &app_data.db,
                                    app_data.current_project_id,
                                ) {
                                    Ok(p) => {
                                        println!("Current Project: {}", p.path.display());
                                    }
                                    Err(_) => eprintln!("Cannot get current project :("),
                                }
                            }
                        }
                    }

                    Ok(())
                }
                Commands::Preview => {
                    flow_api::run_api();
                    Ok(())
                }
            },
            None => {
                print!("\n 🚀Flow - The best tool to improve your workflow and your life ✨");
                if stdout().is_tty() {
                    println!("{}", Print(FLOW_CLI_NAME.blue()));
                } else {
                    println!("{}", FLOW_CLI_NAME);
                }
                Ok(())
            }
        }
    }
}

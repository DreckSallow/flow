use clap::{command, Parser, Subcommand};
use crossterm::{
    style::{Print, Stylize},
    tty::IsTty,
};
use std::{io::stdout, path::PathBuf};

use flow_api;

use flow_data::{
    app_data::AppData,
    data::current_project,
    db::{project::project_utils, Db},
    task::TaskStatus,
    Error,
};

use crate::constants::FLOW_CLI_NAME;

use crate::project::ProjectProgram;
use crate::task::TaskProgram;

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
        let current_project_id = current_project();
        let app_data = AppData::new(db, current_project_id);

        match cli.command {
            Some(command) => match command {
                Commands::Task {
                    description,
                    command,
                } => {
                    if let Some(c) = command {
                        match c {
                            TaskCommands::List {
                                expand,
                                order_by,
                                color,
                            } => TaskProgram::run_list(&app_data, expand, order_by, color),
                            TaskCommands::Rm { id } => TaskProgram::run_delete(&app_data, id),
                            TaskCommands::Start { id } => TaskProgram::run_do_task(&app_data, id),
                            TaskCommands::Stop { id } => TaskProgram::run_update_status(
                                &app_data,
                                vec![id],
                                TaskStatus::Stop,
                            ),
                            TaskCommands::Done { ids } => {
                                TaskProgram::run_update_status(&app_data, ids, TaskStatus::Done)
                            }
                            TaskCommands::Reset { ids } => TaskProgram::run_update_status(
                                &app_data,
                                ids,
                                TaskStatus::NoStarted,
                            ),
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
                            ProjectCommands::Switch { id } => {
                                ProjectProgram::run_switch(&app_data, id)
                            }
                            ProjectCommands::Rm { id } => {
                                ProjectProgram::remove_project(&app_data, id)
                            }
                            ProjectCommands::Use => ProjectProgram::run_use(&app_data),
                        }
                    } else if let Some(p) = path {
                        ProjectProgram::run_default(new, p, &app_data);
                    } else {
                        match project_utils::get_by_id(&app_data.db, app_data.current_project_id) {
                            Ok(p) => {
                                println!("Current Project: {}", p.path.display());
                            }
                            Err(_) => eprintln!("Cannot get current project :("),
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

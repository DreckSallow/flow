use std::{io, path::PathBuf};

use clap::{command, Parser, Subcommand};
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
        path: PathBuf,
    },
    /// Task is a subcommand to manage task related to the current project
    Task {
        /// Description of the task
        #[arg(short, long)]
        description: String,
    },
}

pub struct App;

impl App {
    pub fn run() -> io::Result<()> {
        let cli = Cli::parse();

        match cli.command {
            Commands::Task { description } => {
                println!("description: {}", description);
                Ok(())
            }
            Commands::Project { new, path } => {
                println!("new: {} , path {}", new, path.display());
                Ok(())
            }
        }
    }
}

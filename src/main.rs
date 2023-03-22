use cli::App;

mod cli;
mod db;
mod project;
mod task;
mod utils;

fn main() {
    if let Err(e) = App::run() {
        eprintln!("{:?}", e);
    }
}

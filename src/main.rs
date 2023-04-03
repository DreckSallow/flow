use cli::App;

mod app_data;
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

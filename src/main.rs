use cli::App;

mod api;
mod app_data;
mod cli;
mod constants;
mod db;
mod project;
mod task;
mod utils;

fn main() {
    if let Err(e) = App::run() {
        eprintln!("{:?}", e);
    }
}

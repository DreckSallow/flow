use cli::App;

mod cli;
mod constants;
mod project;
mod task;
mod utils;

fn main() {
    if let Err(e) = App::run() {
        eprintln!("{:?}", e);
    }
}

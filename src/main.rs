use cli::App;

mod cli;
mod constants;
mod project_cmd;
mod task_cmd;
mod utils;

fn main() {
    if let Err(e) = App::run() {
        eprintln!("{:?}", e);
    }
}

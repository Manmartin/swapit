use clap::Parser;
use std::process;

mod action;
mod args;
mod dir;
use args::Cli;

fn main() {
    let args = Cli::parse();

    let action = action::select_action(&args);
    let visit_method = dir::select_visit_method(&args);

    for path in args.paths {
        if path.is_dir() {
            visit_method(&path, action).unwrap_or_else(|error| {
                eprintln!("Error reading directory: {}", error);
                process::exit(1);
            });
        } else {
            action(&path);
        }
    }
}

//! Definition of ways of visit a diretory

use std::fs;
use std::io::Result;
use std::path::Path;

use crate::action::ActionFunction;
use crate::args::Cli;

type VisitFunction = fn(&Path, ActionFunction, &Cli) -> Result<()>;

pub fn select_visit_method(args: &Cli) -> VisitFunction {
    if args.recursive {
        visit_dir_recursive
    } else {
        visit_dir
    }
}

fn visit_dir(path: &Path, swap_function: ActionFunction, args: &Cli) -> Result<()> {
    let entries = fs::read_dir(path)?
        .filter_map(|res| res.ok())
        .map(|entry| entry.path())
        .filter(|path| !path.is_dir());
    for entry in entries {
        swap_function(&entry, &args);
    }
    Ok(())
}

fn visit_dir_recursive(path: &Path, swap_function: ActionFunction, args: &Cli) -> Result<()> {
    let entries = fs::read_dir(path)?
        .filter_map(|res| res.ok())
        .map(|entry| entry.path());
    for entry in entries {
        if entry.is_dir() {
            visit_dir_recursive(&entry, swap_function, args)?;
        } else {
            swap_function(&entry, &args);
        }
    }
    Ok(())
}

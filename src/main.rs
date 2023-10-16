use std::fs;
use std::io::Result;
use std::path::{Path, PathBuf};
use std::process;

use clap::Parser;

const START: &str = "#SWAP";
const END: &str = "#SWAPEND";

/// Search for SWAPBLOCKS in a file and swap comments.
#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Cli {
    /// Path to the file or directory to read
    path: PathBuf,
    /// Change every file in directory and subdirectories
    #[arg(short = 'r', long = "recursive")]
    is_recursive: bool,
}

fn visit_dir(path: &Path, swap_function: fn(&Path)) -> Result<()> {
    let entries = fs::read_dir(path)?
        .filter_map(|res| res.ok())
        .map(|entry| entry.path())
        .filter(|path| !path.is_dir());
    for entry in entries {
        swap_function(&entry);
    }
    Ok(())
}

fn visit_dir_recursive(path: &Path, swap_function: fn(&Path)) -> Result<()> {
    let entries = fs::read_dir(path)?
        .filter_map(|res| res.ok())
        .map(|entry| entry.path());
    for entry in entries {
        if entry.is_dir() {
            visit_dir_recursive(&entry, swap_function)?;
        } else {
            swap_function(&entry);
        }
    }
    Ok(())
}

struct SwapBlock {
    start: usize,
    end: usize,
    indentation: usize,
}

impl SwapBlock {
    fn new(start: usize, indentation: usize) -> Self {
        Self {
            start,
            end: 0,
            indentation,
        }
    }

    fn set_end(&mut self, end: usize) {
        self.end = end;
    }
    fn is_complete(&self) -> bool {
        self.end != 0
    }
}

fn swap(path: &Path) {
    let Ok(file) = fs::read_to_string(path) else {
        return;
    };
    let Some(extension) = path.extension() else {
        return;
    };
    if extension != "tf" {
        return;
    }
    let mut swap_blocks: Vec<SwapBlock> = vec![];
    let mut lines = vec![];
    for (index, line) in file.lines().enumerate() {
        if line.contains(END) {
            swap_blocks
                .iter_mut()
                .last()
                .expect(format!("Not expected to fail in {}", path.to_str().unwrap()).as_str())
                .set_end(index);
        } else if line.contains(START) {
            let indentation = line
                .chars()
                .position(|c| c == '#')
                .expect("Not expected to fail");
            swap_blocks.push(SwapBlock::new(index, indentation));
        }
        lines.push(line.to_owned());
    }

    if swap_blocks.len() == 0 {
        return;
    }
    for swap_block in &swap_blocks {
        if !swap_block.is_complete() {
            return;
        }
    }

    for i in 0..swap_blocks.len() {
        for line in &mut lines[swap_blocks[i].start + 1..swap_blocks[i].end] {
            if line.contains("# ") {
                *line = line.replacen("# ", "", 1);
            } else {
                let second_half = line.split_off(swap_blocks[i].indentation);
                *line = format!("{line}# {second_half}");
            }
        }
    }
    let mut contents = lines.join("\n");
    contents.push('\n');
    fs::write(path, contents).unwrap();
}

fn main() {
    let args = Cli::parse();

    if args.path.is_dir() {
        if args.is_recursive {
            visit_dir_recursive(&args.path, swap).unwrap_or_else(|error| {
                eprintln!("Error reading directory: {}", error);
                process::exit(1);
            })
        } else {
            visit_dir(Path::new(&args.path), swap).unwrap_or_else(|error| {
                eprintln!("Error reading directory: {}", error);
                process::exit(1);
            })
        };
    } else {
        swap(&args.path);
    }
}

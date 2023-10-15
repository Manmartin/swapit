use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
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

fn visit_dir(path: &Path) -> io::Result<Vec<PathBuf>> {
    let entries = fs::read_dir(path)?
        .filter_map(|res| res.ok())
        .map(|entry| entry.path())
        .filter(|path| !path.is_dir());
    Ok(entries.collect())
}

fn visit_dir_recursive(path: &Path) -> io::Result<Vec<PathBuf>> {
    let (dirs, mut files): (Vec<_>, Vec<_>) = fs::read_dir(path)?
        .filter_map(|res| res.ok())
        .map(|entry| entry.path())
        .partition(|path| path.is_dir());
    let mut dir_files = dirs
        .iter()
        .filter_map(|dir| visit_dir_recursive(dir).ok())
        .flatten()
        .collect::<Vec<PathBuf>>();
    files.append(&mut dir_files);
    Ok(files)
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
        let entries = if args.is_recursive {
            visit_dir_recursive(Path::new(".")).unwrap_or_else(|error| {
                eprintln!("Error reading directory: {}", error);
                process::exit(1);
            })
        } else {
            visit_dir(Path::new(&args.path)).unwrap_or_else(|error| {
                eprintln!("Error reading directory: {}", error);
                process::exit(1);
            })
        };
        for entry in entries {
            swap(&entry);
        }
    } else {
        swap(&args.path);
    }
}

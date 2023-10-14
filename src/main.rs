use std::fs;
use std::io;
use std::path::Path;
use std::process;

const START: &str = "#SWAP";
const END: &str = "#SWAPEND";

fn visit_dir(path: &Path) -> io::Result<Vec<String>> {
    let entries = fs::read_dir(path)?
        .filter_map(|res| res.ok())
        .filter_map(|file_path| {
            if file_path.path().is_dir() {
                None
            } else {
                Some(file_path.file_name())
            }
        })
        .filter_map(|file| file.into_string().ok());
    Ok(entries.collect())
}

fn visit_dir_recursive(path: &Path) -> io::Result<Vec<String>> {
    let (dirs, files): (Vec<_>, Vec<_>) = fs::read_dir(path)?
        .filter_map(|res| res.ok())
        .map(|entry| entry.path())
        .partition(|path| path.is_dir());
    let dir_files = dirs
        .iter()
        .filter_map(|dir| visit_dir_recursive(dir).ok())
        .flatten();
    let files = files
        .iter()
        .filter_map(|path| path.file_name())
        .filter_map(|file_name| file_name.to_owned().into_string().ok())
        .chain(dir_files);
    Ok(files.collect())
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
}

fn swap(path: &str) {
    let Ok(file) = fs::read_to_string(path) else {
        return;
    };
    let mut swap_blocks: Vec<SwapBlock> = vec![];
    let mut lines = vec![];
    for (index, line) in file.lines().enumerate() {
        if line.contains(END) {
            swap_blocks
                .iter_mut()
                .last()
                .expect("Not expected to fail")
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
        if swap_block.end == 0 {
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
    let entries = if true {
        visit_dir_recursive(Path::new(".")).unwrap_or_else(|error| {
            eprintln!("Error reading directory: {}", error);
            process::exit(1);
        })
    } else {
        visit_dir(Path::new(".")).unwrap_or_else(|error| {
            eprintln!("Error reading directory: {}", error);
            process::exit(1);
        })
    };

    for entry in &entries {
        swap(entry);
        //println!("{}", entry);
    }
}

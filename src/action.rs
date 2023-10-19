//! Definition of actions to apply on a file

use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::args::{Cli, Subcommands};

pub type ActionFunction = fn(&Path, &Cli);

pub fn select_action(args: &Cli) -> ActionFunction {
    match args.command {
        Some(Subcommands::List) => list,
        None => swap,
    }
}

const START: &str = "#SWAP";
const END: &str = "#SWAPEND";

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

fn swap(path: &Path, _args: &Cli) {
    let Some(extension) = path.extension() else {
        return;
    };
    if extension != "tf" {
        return;
    }
    let Ok(file) = File::open(path) else {
        return;
    };
    let file = BufReader::new(file);
    let mut swap_blocks: Vec<SwapBlock> = vec![];
    let mut lines = vec![];
    for (index, line) in file.lines().map_while(|line| line.ok()).enumerate() {
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

    if swap_blocks.is_empty() {
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

fn list(path: &Path, _args: &Cli) {
    let Some(extension) = path.extension() else {
        return;
    };
    if extension != "tf" {
        return;
    }
    let Ok(file) = File::open(path) else {
        return;
    };
    let file = BufReader::new(file);
    let mut swap_blocks: Vec<SwapBlock> = vec![];
    let mut lines = vec![];
    for (index, line) in file.lines().map_while(|line| line.ok()).enumerate() {
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

    if swap_blocks.is_empty() {
        return;
    }
    for swap_block in &swap_blocks {
        if !swap_block.is_complete() {
            return;
        }
    }

    let Some(path_to_print) = path.to_str() else {
        return;
    };

    println!("{}", path_to_print);
    for i in 0..swap_blocks.len() {
        println!("Block {}", i + 1);
        for line in &lines[swap_blocks[i].start + 1..swap_blocks[i].end] {
            println!("{}", line);
        }
        println!()
    }

    let mut contents = lines.join("\n");
    contents.push('\n');
    fs::write(path, contents).unwrap();
}

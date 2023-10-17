//! Definition of input parameters

use clap::Parser;
use std::path::PathBuf;

/// Search for SWAPBLOCKS in a file and swap comments.
#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Cli {
    /// Path to the file or directory to read
    pub paths: Vec<PathBuf>,
    /// Change every file in directory and subdirectories
    #[arg(short = 'r', long = "recursive")]
    pub recursive: bool,
}

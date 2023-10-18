//! Definition of input parameters

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Search for SWAPBLOCKS in a file and swap comments.
#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Subcommands>,

    /// Paths to files or directories to read
    #[arg(global = true)]
    pub paths: Vec<PathBuf>,

    /// Apply action to every file in directory and subdirectories
    #[arg(short = 'r', long = "recursive", global = true)]
    pub recursive: bool,
}

#[derive(Subcommand, Debug)]
pub enum Subcommands {
    /// List all Swapblocks
    List,
}

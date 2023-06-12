use std::path::PathBuf;

use clap::{Parser};

#[derive(Debug)]
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(value_name = "FILE")]
    pub input: Option<PathBuf>,

    #[arg(short, value_name = "FILE")]
    pub output: Option<PathBuf>,
}

pub fn parse_args() -> Cli{
    Cli::parse()
}

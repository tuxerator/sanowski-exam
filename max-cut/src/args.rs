use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    pub file: std::path::PathBuf,

    /// Write output into specified file
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,
}

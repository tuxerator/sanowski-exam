mod args;
use std::{fs, process};

use args::Args;
use max_cut::graph_parser;

use clap::Parser;

fn main() {
    let args = Args::parse();

    let input_graph = fs::read_to_string(&args.file).unwrap_or_else(|err| {
        eprintln!("could not open \'{}\': {err}", args.file.to_str().unwrap());
        process::exit(1);
    });

    let graph = graph_parser::parse_graph(&input_graph).unwrap_or_else(|err| {
        eprintln!("parse error: \n\t{err}");
        process::exit(1);
    });

    println!("parsed \'{}\':\n{:?}", args.file.to_str().unwrap(), graph);
}

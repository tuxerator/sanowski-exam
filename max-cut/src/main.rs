mod args;
use core::time;
use std::{fs, process, thread, time::{Instant, Duration}};

use args::Args;
use max_cut::{aprox, graph_parser, heuristic, ilp};

use clap::Parser;

fn main() {
    let args = Args::parse();

    let input_graph = fs::read_to_string(&args.file).unwrap_or_else(|err| {
        eprintln!("could not open \'{}\': {err}", args.file.to_str().unwrap());
        process::exit(1);
    });

    let graph = if args.file.extension().unwrap() == "gr" {
        graph_parser::parse_pace_graph(&input_graph).unwrap_or_else(|err| {
            eprintln!("parse error: \n\t{err}");
            process::exit(1);
        })
    } else {
        graph_parser::parse_rudy(&input_graph).unwrap_or_else(|err| {
            eprintln!("parse error: \n\t{err}");
            process::exit(1);
        })
    };

    if !args.bench {
        println!("parsed \'{}\'", args.file.to_str().unwrap());
    }

    if args.ilp {
        let ilp = ilp::MaxCutIlp::new(&graph);

        let file = args.file.to_str().unwrap().to_owned();

        let timeout = Instant::now();
        thread::spawn(move || {
            while timeout.elapsed() <= Duration::from_secs(3600 * 2) {};
                println!("{}, timeout", file);
                process::exit(2);
        });
        let start = Instant::now();

        let exact = ilp.solve().unwrap_or_else(|err| {
            eprintln!("ilp error: {err}");
            process::exit(1);
        });

        let end = start.elapsed();

        if args.bench {
            let solution_size = exact.len();

            println!(
                "{}, {}, {}, {}",
                args.file.to_str().unwrap(),
                graph.size(),
                solution_size,
                end.as_millis(),
            );
        } else {
            println!(
                "Maximum cut for \'{}\':\n\n{:?}",
                args.file.to_str().unwrap(),
                exact
            );
        }
    }
}

mod args;
use std::{
    fs, process, thread,
    time::{Duration, Instant}, sync::Arc,
};

use args::Args;
use max_cut::{approx, graph_parser, heuristic, ilp};

use clap::Parser;

fn main() {
    let args = Args::parse();

    let input_graph = fs::read_to_string(&args.file).unwrap_or_else(|err| {
        eprintln!("could not open \'{}\': {err}", args.file.to_str().unwrap());
        process::exit(1);
    });

    let graph = Arc::new(if args.file.extension().unwrap() == "gr" {
        graph_parser::parse_pace_graph(&input_graph).unwrap_or_else(|err| {
            eprintln!("parse error: \n\t{err}");
            process::exit(1);
        })
    } else {
        graph_parser::parse_rudy(&input_graph).unwrap_or_else(|err| {
            eprintln!("parse error: \n\t{err}");
            process::exit(1);
        })
    });

    if !args.bench {
        println!("parsed \'{}\'", args.file.to_str().unwrap());
    }

    if args.ilp {
        let ilp = ilp::MaxCutIlp::new(&graph);

        let file = args.file.to_str().unwrap().to_owned();

        let timeout = Instant::now();
        // thread::spawn(move || {
        //     while timeout.elapsed() <= Duration::from_secs(3600 * 2) {}
        //     println!("{}, timeout", file);
        //     process::exit(2);
        // });
        let start = Instant::now();

        let exact = ilp.solve().unwrap_or_else(|err| {
            eprintln!("ilp error: {err}");
            process::exit(1);
        });

        let end = start.elapsed();

        if args.bench {
            let solution_size = exact.len();

            println!(
                "{}, {}, {}, {}, {}",
                args.file.to_str().unwrap(),
                graph.size(),
                graph.edge_size(),
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

    if args.approx {
        let start;
        let cut;
        let end;

        if args.improved {
            start = Instant::now();
            cut = approx::max_cut_greedy_impr(&graph);
            end = start.elapsed();
        } else {
            start = Instant::now();
            cut = approx::max_cut_greedy(&graph);
            end = start.elapsed();
        }

        if args.bench {
            println!(
                "{}, {}, {}, {}, {}",
                args.file.to_str().unwrap(),
                graph.size(),
                graph.edge_size(),
                cut.len(),
                end.as_millis(),
            );
        } else {
            println!(
                "Appriximated maximum cut for \'{}\': \n\n{:?}",
                args.file.to_str().unwrap(),
                cut
            )
        }
    }

    if args.heuristic {
        let start;
        let cut;
        let end;

        if args.improved {
            start = Instant::now();
            cut = heuristic::rand_approx_impr(Arc::clone(&graph)).unwrap();
            end = start.elapsed();
        } else if args.heuristic_parallel {
            start = Instant::now();
            cut = heuristic::rand_aprox_parallel(&graph).unwrap();
            end = start.elapsed();
        } else {
            start = Instant::now();
            cut = heuristic::rand_aprox(&graph);
            end = start.elapsed();
        }

        if args.bench {
            println!(
                "{}, {}, {}, {}, {}",
                args.file.to_str().unwrap(),
                graph.size(),
                graph.edge_size(),
                cut.len(),
                end.as_millis(),
            );
        } else {
            println!(
                "Appriximated maximum cut for \'{}\': \n\n{:?}",
                args.file.to_str().unwrap(),
                cut
            )
        }
    }
}

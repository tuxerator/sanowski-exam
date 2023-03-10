use std::process;
use std::fs;

use good_lp::Solution;
use max_cut::*;

pub fn setup() {
}

#[test]
fn ilp_test() -> Result<(), good_lp::ResolutionError> {
    let input_graph = fs::read_to_string("tests/test_recources/graph01.gr").unwrap_or_else(|err| {
        eprintln!("could not open \'test_recources/graph01.gr\': {err}");
        process::exit(1);
    });

    let graph = graph_parser::parse_graph(&input_graph).unwrap_or_else(|err| {
        eprintln!("parse error: \n\t{err}");
        process::exit(1);
    });

    let ilp = ilp::MaxCutIlp::new(&graph);

    println!("{:?}", ilp.solve()?);
    
    Ok(())
}

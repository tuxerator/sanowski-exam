use std::num::NonZeroUsize;
use std::thread::{available_parallelism, self, Result};

use crate::graph::{Graph, Edge};
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

pub fn rand_aprox(graph: &Graph) -> Vec<Edge> {
    let mut s = vec![];
    let mut rand = SmallRng::from_entropy();

    for vertex in 0..graph.size() {
        match rand.gen_bool(1.0 / 2.0) {
            true => s.push(vertex),
            false => continue,
        }
    }

    let edges = graph.all_edges();
    let mut cut: Vec<Edge> = Vec::new();

    for edge in edges {
        if s.contains(&edge.0) && s.contains(&edge.1) || s.contains(&edge.0) && s.contains(&edge.1)
        {
            continue;
        } else {
            cut.push(edge);
        }
    }

    cut
}

pub fn rand_aprox_parallel(graph: &Graph) -> Result<Vec<Edge>>{
    let mut s = vec![];
    let cores = available_parallelism().unwrap_or(NonZeroUsize::new(8).unwrap()).get();
    let mut slices = vec![vec![]; cores];
    let mut core = 0;

    for vertex in 0..graph.size() {
        slices[core].push(vertex);
        core = if core < cores - 1 { core + 1} else { 0 };
    }

    let mut handles = vec![];

    for slice in slices {
        let handle = thread::spawn(move || {
            let mut s = vec![];
            let mut rand = SmallRng::from_entropy();

            for vertex in slice {
                match rand.gen_bool(1.0 / 2.0) {
                    true => s.push(vertex),
                    false => continue,
                }
            }

            s
        });

        handles.push(handle);
    }

    let mut results = vec![];

    for handle in handles {
        results.push(handle.join()?);

    }


    for mut ele in results {
        s.append(&mut ele);
    }

    let edges = graph.all_edges();
    let mut cut: Vec<Edge> = Vec::new();

    for edge in edges {
        if s.contains(&edge.0) && s.contains(&edge.1) || s.contains(&edge.0) && s.contains(&edge.1)
        {
            continue;
        } else {
            cut.push(edge);
        }
    }


    Ok(cut)
}


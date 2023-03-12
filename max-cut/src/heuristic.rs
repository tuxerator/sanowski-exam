use std::num::NonZeroUsize;
use std::thread::{available_parallelism, self, Result};

use crate::graph::Graph;
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

pub fn rand_aprox(graph: &Graph) -> (Vec<usize>, Vec<usize>) {
    let mut cut = (Vec::new(), Vec::new());
    let mut rand = SmallRng::from_entropy();

    for vertex in 0..graph.size() {
        match rand.gen_bool(1.0 / 2.0) {
            true => cut.0.push(vertex),
            false => cut.1.push(vertex),
        }
    }

    cut
}

pub fn rand_aprox_parallel(graph: &Graph) -> Result<(Vec<usize>, Vec<usize>)>{
    let mut cut = (vec![], vec![]);
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
            let mut partition = (vec![], vec![]);
            let mut rand = SmallRng::from_entropy();

            for vertex in slice {
                match rand.gen_bool(1.0 / 2.0) {
                    true => partition.0.push(vertex),
                    false => partition.1.push(vertex),
                }
            }

            partition
        });

        handles.push(handle);
    }

    let mut results = vec![];

    for handle in handles {
        results.push(handle.join()?);

    }


    for mut ele in results {
        cut.0.append(&mut ele.0);
        cut.1.append(&mut ele.1);
    }


    Ok(cut)
}


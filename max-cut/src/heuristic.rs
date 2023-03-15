use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex};
use std::thread::{self, available_parallelism, Result};

use crate::graph::{Edge, Graph};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

pub fn rand_aprox(graph: &Graph) -> Vec<Edge> {
    let mut s = vec![false; graph.size()];
    let mut rand = SmallRng::from_entropy();

    for vertex in 0..graph.size() {
        match rand.gen_bool(1.0 / 2.0) {
            true => s[vertex] = true,
            false => continue,
        }
    }

    let edges = graph.all_edges();
    let mut cut: Vec<Edge> = Vec::new();
    for edge in edges {
        let s_0 = s[edge.0];
        let s_1 = s[edge.1];
        if s_0 == s_1 {
            continue;
        } else {
            cut.push(edge);
        }
    }

    cut
}

pub fn rand_approx_impr(graph: Arc<Graph>) -> Result<Vec<Edge>> {
    let cores = available_parallelism()
        .unwrap_or(NonZeroUsize::new(8).unwrap())
        .get();

    let mut best = vec![];

    while (best.len() as f64) < graph.size() as f64 * 0.5 {
        let mut handles = vec![];
        for _core in 0..cores {
            let graph = Arc::clone(&graph);
            let handle = thread::spawn(move || rand_aprox(&graph));
            handles.push(handle);
        }

        let mut results = vec![];

        for handle in handles {
            let result = handle.join()?;
            results.push(result);
        }

        for result in results {
            if result.len() > best.len() {
                best = result;
            }
        }
    }

    Ok(best)
}

pub fn rand_aprox_parallel(graph: &Graph) -> Result<Vec<Edge>> {
    let s = Arc::new(Mutex::new(vec![false; graph.size()]));
    let cores = available_parallelism()
        .unwrap_or(NonZeroUsize::new(8).unwrap())
        .get();
    let mut slices = vec![vec![]; cores];
    let mut core = 0;

    for vertex in 0..graph.size() {
        slices[core].push(vertex);
        core = if core < cores - 1 { core + 1 } else { 0 };
    }

    let mut handles = vec![];

    for slice in slices {
        let s = Arc::clone(&s);
        let handle = thread::spawn(move || {
            let mut rand = SmallRng::from_entropy();

            for vertex in slice {
                match rand.gen_bool(1.0 / 2.0) {
                    true => s.lock().unwrap()[vertex] = true,
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

    let s = Arc::new(s.lock().unwrap().clone());

    let edges = graph.all_edges();
    let mut cut: Vec<Edge> = Vec::new();
    let mut slices: Vec<Vec<Edge>> = vec![vec![]; cores];
    let mut handles = vec![];

    for edge in edges {
        slices[core].push(edge);
        core = if core < cores - 1 { core + 1 } else { 0 };
    }

    for slice in slices {
        let s = Arc::clone(&s);
        let handle = thread::spawn(move || {
            let mut cut = vec![];

            for edge in slice {
                let s_0 = s[edge.0];
                let s_1 = s[edge.1];
                if s_0 == s_1 {
                    continue;
                } else {
                    cut.push(edge);
                }
            }

            cut
        });

        handles.push(handle);
    }

    let mut results = vec![];

    for handle in handles {
        results.push(handle.join()?);
    }

    for mut ele in results {
        cut.append(&mut ele);
    }

    Ok(cut)
}

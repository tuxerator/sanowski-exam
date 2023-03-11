use crate::graph::{Graph, Edge};

pub fn max_cut_greedy(graph: &Graph) -> Vec<Edge> {
    let mut partitions = (Vec::new(), Vec::new());

    for vertex in 0..graph.size() {
        let neighbors = graph.get_neighbors_undireted(vertex);
        let counts = neighbors.iter().fold((0usize, 0usize), |mut counts, neigh| {
            if partitions.0.contains(neigh) {
                counts.0 += 1;
            }
            if partitions.1.contains(neigh) {
                counts.1 += 1;
            }

            counts
        });

        if counts.0 <= counts.1 {
            partitions.0.push(vertex);
        }
        else {
            partitions.1.push(vertex);
        }
    }

    let edges = graph.all_edges();
    let mut cut: Vec<Edge> = Vec::new();

    for edge in edges{
        if partitions.0.contains(&edge.0) && partitions.0.contains(&edge.1) || partitions.1.contains(&edge.0) && partitions.1.contains(&edge.1) {
            continue;
        }
        else {
            cut.push(edge);
        }
    }
    
    cut
}

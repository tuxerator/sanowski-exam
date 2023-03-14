use crate::graph::{Edge, Graph};

pub fn max_cut_greedy(graph: &Graph) -> Vec<Edge> {
    let mut s = vec![];

    for vertex in 0..graph.size() {
        let neighbors = graph.get_neighbors(vertex);
        let counts = neighbors
            .iter()
            .fold((0usize, 0usize), |mut counts, neigh| {
                if s.contains(neigh) {
                    counts.0 += 1;
                } else {
                    counts.1 += 1;
                }

                counts
            });

        if counts.0 <= counts.1 {
            s.push(vertex);
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

pub fn max_cut_greedy_impr(graph: &Graph) -> Vec<Edge> {
    let mut table = vec![false; graph.size()];
    let mut cut = vec![];

    for vertex in 0..graph.size() {
        let neighbors = graph.get_neighbors(vertex);
        let mut counts = neighbors
            .iter()
            .fold((vec![], vec![]), |mut counts, neigh| {
                if *neigh <= vertex {
                    if table[*neigh] {
                        counts.0.push(Edge(vertex, *neigh));
                    } else {
                        counts.1.push(Edge(vertex, *neigh));
                    }
                }

                counts
            });

        if counts.0.len() <= counts.1.len() {
            table[vertex] = true;
            cut.append(&mut counts.1);
        } else {
            cut.append(&mut counts.0);
        }
    }
    cut
}

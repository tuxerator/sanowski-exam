pub mod node;
pub mod edge;
mod adjacency_matrix;

use edge::Edge;
use crate::quad_matrix_usize::QuadMatrixBool;

#[derive(Debug)]
pub struct Graph {
    adjacency_matrix: QuadMatrixBool,
}

impl Graph {
    pub fn new(n: usize) -> Graph {
        Graph { adjacency_matrix: QuadMatrixBool::new_epmty(n) }
    }
    pub fn add_edge(&mut self, edge: &Edge) {
        self.adjacency_matrix.set((edge.node_l, edge.node_r), true);
    }

    pub fn add_edges(&mut self, edges: &[Edge]) {
        let edge_iter = edges.iter();

        edge_iter.for_each(|edge| self.add_edge(&edge));
    }
}

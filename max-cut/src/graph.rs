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
    pub fn add_vertex(&mut self, id: usize) {
        if !self.adjacency_matrix. {
            self.vertecies.push(id);
        }
    }

    pub fn delete_vertex(&mut self, id: usize) {
        self.vertecies.swap_remove(id);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        if !self.edges.contains(&edge) {
            self.edges.push(edge);
        }
    }
}

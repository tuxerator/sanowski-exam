pub mod node;
pub mod edge;

use edge::Edge;

#[derive(Debug)]
pub struct Graph {
    vertecies: Vec<usize>,
    edges: Vec<Edge>,
}

impl Graph {
    pub fn add_vertex(&mut self, id: usize) {
        if !self.vertecies.contains(&id) {
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

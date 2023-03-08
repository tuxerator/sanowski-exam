use crate::quad_matrix_usize::QuadMatrixBool;
use super::edge::Edge;

#[derive(Debug)]
struct AdjacencyMatrix {
    // vericies: Vec<usize>,
    matrix: QuadMatrixBool,
}

impl AdjacencyMatrix {
    pub fn new_empty(n: usize) -> AdjacencyMatrix {
        AdjacencyMatrix { 
            // vericies: (0..n).collect(),
            matrix: QuadMatrixBool::new_epmty(n) 
        }
    }

    // pub fn new(edges: &Vec<Edge>) -> AdjacencyMatrix {
    //     let edge_iter = edges.iter();


    // }

    pub fn add_edge(&mut self, edge: &Edge) {
        // let i = (
        //     self.vericies.iter().position(|&x| x == edge.node_l).unwrap(),
        //     self.vericies.iter().position(|&x| x == edge.node_r).unwrap()
        // );

        let i = (edge.node_l, edge.node_r);
        self.matrix.set(i, true);
        let i = (i.1, i.0);
        self.matrix.set(i, true);
    }

    pub fn add_edges(&mut self, edges: &[Edge]) {
        let edges_iter = edges.iter();

        edges_iter.for_each(|edge| self.add_edge(edge));
    }

    pub fn delete_edge(&mut self, edge: &Edge) {
        let i = (edge.node_l, edge.node_r);
        self.matrix.set(i, false);
        let i = (i.1, i.0);
        self.matrix.set(i, false);
    }

    pub fn delete_edges(&mut self, edges: &[Edge]) {
        let edges_iter = edges.iter();

        edges_iter.for_each(|edge| self.delete_edge(edge));
    }

    pub fn contains_edge(&mut self, edge: Edge) -> bool {
        if edge.node_l >= self.matrix.size() || edge.node_r >= self.matrix.size() {
            return false;
        }
        self.matrix.get((edge.node_l, edge.node_r))
    }
}

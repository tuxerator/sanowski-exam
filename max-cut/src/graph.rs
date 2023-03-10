use std::{fmt::{self, Debug}, usize};

#[derive(PartialEq,Clone, Copy)]
pub struct Edge(pub usize, pub usize);

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0 + 1, self.1 + 1)
    }
}

impl Debug for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0 + 1, self.1 + 1)
    }
}

/// A graph type using an adjacency matrix.
/// For simplicity, once created, only edges can be added or removed.
#[derive(PartialEq,Debug)]
pub struct Graph {
    adjacency_matrix: Vec<Vec<bool>>,
}

#[derive(Debug)]
pub struct NotQuadError;

/// Error indicating that the suplied matrix for [`Graph`]
/// wasn't quadratic.
impl std::error::Error for NotQuadError {}

impl fmt::Display for NotQuadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Initial matrix is not quadratic!")
    }
}


impl Graph {
    pub fn new_empty(n: usize) -> Graph {
        Graph { adjacency_matrix: vec![vec![false; n]; n] }
    }
    
    /// Creates a [`Graph`] form `data`.
    ///
    /// # Errors
    ///
    /// This function returns an error if `data` is not quadratic [`NotQuadError`]
    pub fn new(data: Vec<Vec<bool>>) -> Result<Graph, NotQuadError> {
        if !data.iter().all(|x| x.len() == data.len()) {
            return Err(NotQuadError);
        }

        Ok(Graph { adjacency_matrix: data })
    }

    pub fn add_edge(&mut self, edge: &(usize, usize)) {
        self.adjacency_matrix[edge.0][edge.1] = true;
    }

    pub fn add_edges(&mut self, edges: &[(usize, usize)]) {
        let edge_iter = edges.iter();

        edge_iter.for_each(|edge| self.add_edge(edge));
    }

    pub fn contains_edge(&self, edge: &(usize, usize)) -> bool {
        self.adjacency_matrix[edge.0][edge.1]
    }

    pub fn all_edges(& self) -> Vec<Edge> {
        // Filter out all 'false' entries
        self.adjacency_matrix.iter().scan(0usize, |i, r| {
            let old = *i;
            *i += 1;
            Some(r.iter().scan((old, 0usize), |j, &e| {
                let edge = Edge(j.0, j.1);
                j.1 += 1;
                if e {
                    Some(Some(edge))
                } else {
                    Some(None)
                }
            }).filter_map(|x| x))
        }).flatten().collect()
    }

    pub fn edge_size(&self) -> usize {
        self.adjacency_matrix.iter().flatten().fold(0, |mut n, x| { if *x {n += 1}; n })
    }

    pub fn size(&self) -> usize {
        self.adjacency_matrix.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_edges() {
        let graph = Graph::new(vec![
            vec![false,true,true,false],
            vec![false,false,true,true],
            vec![true,false,false,false],
            vec![true,false,false,false]
        ]).unwrap();

        let expected = vec![Edge(0usize,1usize),Edge(0usize,2usize),Edge(1usize,2usize),Edge(1usize,3usize),Edge(2usize,0usize),Edge(3usize,0usize)];

        assert_eq!(graph.all_edges(), expected);
    }
}

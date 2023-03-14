use std::{
    fmt::{self, Debug},
    usize,
};

#[derive(PartialEq, Clone, Copy)]
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
#[derive(PartialEq, Debug)]
pub struct Graph {
    // adjacency_matrix: Vec<Vec<bool>>,
    adjacency_lists: Vec<Vec<usize>>,
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
        Graph {
            // adjacency_matrix: vec![vec![false; n]; n],
            adjacency_lists: vec![vec![]; n],
        }
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

        let mut adjacency_lists = vec![vec![]; data.len()];

        for row in 0..data.len() {
            for i in 0..data[row].len() {
                if data[row][i] {
                    if !adjacency_lists[row].contains(&i) {
                        adjacency_lists[row].push(i)
                    }
                    if !adjacency_lists[i].contains(&row) {
                        adjacency_lists[i].push(row)
                    }
                };
            }
        }

        Ok(Graph {
            // adjacency_matrix: data,
            adjacency_lists,
        })
    }

    pub fn add_edge(&mut self, edge: &(usize, usize)) {
        // self.adjacency_matrix[edge.0][edge.1] = true;
        if !self.adjacency_lists[edge.0].contains(&edge.1) {
            self.adjacency_lists[edge.0].push(edge.1);
        }
        if !self.adjacency_lists[edge.1].contains(&edge.0) {
            self.adjacency_lists[edge.1].push(edge.0);
        }
    }

    pub fn add_edges(&mut self, edges: &[(usize, usize)]) {
        let edge_iter = edges.iter();

        edge_iter.for_each(|edge| self.add_edge(edge));
    }

    pub fn contains_edge(&self, edge: &(usize, usize)) -> bool {
        // self.adjacency_matrix[edge.0][edge.1]
        self.adjacency_lists[edge.0].contains(&edge.1) || self.adjacency_lists[edge.1].contains(&edge.0)
    }

    pub fn all_edges(&self) -> Vec<Edge> {
        // Filter out all 'false' entries
        // self.adjacency_matrix
        //     .iter()
        //     .scan(0usize, |i, r| {
        //         let old = *i;
        //         *i += 1;
        //         Some(
        //             r.iter()
        //                 .scan((old, 0usize), |j, &e| {
        //                     let edge = Edge(j.0, j.1);
        //                     j.1 += 1;
        //                     if e {
        //                         Some(Some(edge))
        //                     } else {
        //                         Some(None)
        //                     }
        //                 })
        //                 .flatten(),
        //         )
        //     })
        //     .flatten()
        //     .collect()

        let mut edges = vec![];

        for i in 0..self.adjacency_lists.len() {
            self.adjacency_lists[i].iter().for_each(|x| {
                if *x > i {
                    edges.push(Edge(i, *x))
                }
            });
        }

        edges
    }

    pub fn edge_size(&self) -> usize {
        self.adjacency_lists
            .iter().enumerate()
            .fold(0usize, |mut n, x| {
                n += x.1.iter().fold(0, |mut m, z| {
                    if *z > x.0 {
                        m += 1
                    }

                    m
                });

                n
            })
    }

    pub fn size(&self) -> usize {
        self.adjacency_lists.len()
    }

    pub fn get_neighbors(&self, vertex: usize) -> Vec<usize> {
        self.adjacency_lists[vertex].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_edges() {
        let graph = Graph::new(vec![
            vec![false, true, true, false],
            vec![false, false, true, true],
            vec![true, false, false, false],
            vec![true, false, false, false],
        ])
        .unwrap();

        let expected = vec![
            Edge(0usize, 1usize),
            Edge(0usize, 2usize),
            Edge(0usize, 3usize),
            Edge(1usize, 2usize),
            Edge(1usize, 3usize),
        ];

        assert_eq!(graph.all_edges(), expected);
        assert_eq!(graph.edge_size(), expected.len());
    }

    #[test]
    fn get_neighbors() {
        let graph = Graph::new(vec![
            vec![false, true, true, false],
            vec![false, false, true, true],
            vec![true, false, false, false],
            vec![true, false, false, false],
        ])
        .unwrap();

        assert_eq!(graph.get_neighbors(0usize), vec![1usize, 2usize, 3usize]);
        assert_eq!(graph.get_neighbors(2usize), vec![0usize, 1usize])
    }
}

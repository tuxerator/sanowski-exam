#[derive(Debug)]
pub struct Edge {
    pub node_l: usize,
    pub node_r: usize,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        if self.node_l == other.node_l && self.node_r == other.node_r {
            true
        }
        else {
            false
        }
    }
}

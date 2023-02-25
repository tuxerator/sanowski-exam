#[derive(Debug)]
pub struct Edge {
    pub node_l: u32,
    pub node_r: u32,
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

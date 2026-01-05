use crate::node::Node;

pub struct Edge {
    pub source: Node,
    pub target: Node,
}

impl Edge {
    pub fn new(source: Node, target: Node) -> Self {
        Edge { source, target }
    }
}

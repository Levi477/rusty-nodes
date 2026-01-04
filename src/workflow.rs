use crate::edge::Edge;
use crate::node::Node;
use std::collections::HashMap;

pub struct Workflow {
    edges: Vec<Edge>,
}

impl Workflow {
    pub fn new(edges: Vec<Edge>) -> Self {
        Workflow { edges }
    }

    fn build_indegree_hashmap(&self) -> HashMap<&Node, usize> {
        let mut indegree_hashmap = HashMap::new();
        for edge in &self.edges {
            *indegree_hashmap.entry(&edge.target).or_insert(0) += 1;
        }
        indegree_hashmap
    }

    pub fn run(&self) {
        let mut indegree_hashmap = self.build_indegree_hashmap();
        let mut queue = Vec::new();
        // Kahn's Algorithm
}

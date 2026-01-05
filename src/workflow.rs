use crate::edge::Edge;
use crate::node::Node;
use rayon::prelude::*;
use std::collections::{HashMap, VecDeque};

pub struct Workflow {
    edges: Vec<Edge>,
}

impl Workflow {
    pub fn new(edges: Vec<Edge>) -> Self {
        Workflow { edges }
    }

    fn build_indegree_hashmap(&self) -> HashMap<&Node, usize> {
        let mut indegree = HashMap::new();

        for edge in &self.edges {
            // target gets +1
            *indegree.entry(&edge.target).or_insert(0) += 1;

            // source must exist with 0
            indegree.entry(&edge.source).or_insert(0);
        }

        indegree
    }

    pub fn run(&self) -> String {
        let mut indegree = self.build_indegree_hashmap();
        let mut queue = VecDeque::new();

        let mut outputs: HashMap<Node, String> = HashMap::new();

        // Initialize queue
        for (&node, &count) in &indegree {
            if count == 0 {
                queue.push_back(node);
            }
        }

        while !queue.is_empty() {
            let batch: Vec<&Node> = queue.drain(..).collect();

            let results: Vec<(&Node, String)> = batch
                .par_iter()
                .map(|node| {
                    //  Collect parent outputs safely
                    let parents: Vec<&Node> = self
                        .edges
                        .iter()
                        .filter(|e| &e.target == *node)
                        .map(|e| &e.source)
                        .collect();

                    let input = if parents.is_empty() {
                        String::new() // root node
                    } else {
                        parents
                            .iter()
                            .filter_map(|p| outputs.get(*p))
                            .cloned()
                            .collect::<Vec<_>>()
                            .join("")
                    };

                    let output = (node.execute)(&input);
                    (*node, output)
                })
                .collect();

            for (node, output) in results {
                outputs.insert(node.clone(), output);
            }

            for node in batch {
                for edge in self.edges.iter().filter(|e| &e.source == node) {
                    let target = &edge.target;
                    let count = indegree.get_mut(target).unwrap();
                    *count -= 1;
                    if *count == 0 {
                        queue.push_back(target);
                    }
                }
            }
        }

        // Final output = sink node(s)
        self.find_sink_nodes()
            .into_iter()
            .filter_map(|n| outputs.get(&n))
            .cloned()
            .collect::<Vec<_>>()
            .join("")
    }

    fn find_sink_nodes(&self) -> Vec<Node> {
        let mut has_outgoing = HashMap::new();

        for edge in &self.edges {
            has_outgoing.insert(edge.source.clone(), true);
            has_outgoing.entry(edge.target.clone()).or_insert(false);
        }

        has_outgoing
            .into_iter()
            .filter(|(_, has_out)| !*has_out)
            .map(|(node, _)| node)
            .collect()
    }
}

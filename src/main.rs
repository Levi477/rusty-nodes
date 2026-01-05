#![allow(unused)]

mod edge;
mod node;
mod workflow;
use node::Node;
use workflow::Workflow;

fn starting_function(ip: &str) -> String {
    "starting".to_string()
}

fn add1(ip: &str) -> String {
    ip.to_string() + "1"
}

fn add2(ip: &str) -> String {
    ip.to_string() + "2"
}

fn main() {
    // define nodes
    let starting_node = Node::new("startingNode".to_string(), starting_function);
    let node1 = Node::new("node1".to_string(), add1);
    let node2 = Node::new("node2".to_string(), add2);

    // define edges
    let edge1 = edge::Edge::new(starting_node.clone(), node1.clone());
    let edge2 = edge::Edge::new(node1.clone(), node2.clone());

    // define workflow
    let workflow = Workflow::new(vec![edge1, edge2]);

    // run workflow
    let output = workflow.run();
    println!("output is : {}", output);
}

#![allow(unused)]
#![allow(unpredictable_function_pointer_comparisons)]

mod edge;
mod node;
mod workflow;
use edge::Edge;
use node::Node;
use std::thread;
use std::time::Duration;
use workflow::Workflow;

fn start(_: &str) -> String {
    "raw_data".to_string()
}

fn parse(input: &str) -> String {
    format!("parsed({})", input)
}

fn normalize(input: &str) -> String {
    thread::sleep(Duration::from_secs(3));
    format!("normalized({})", input)
}

fn validate(input: &str) -> String {
    thread::sleep(Duration::from_secs(4));
    format!("validated({})", input)
}

fn enrich(input: &str) -> String {
    format!("enriched({})", input)
}

fn merge(input: &str) -> String {
    format!("merged[{}]", input)
}

fn finalize(input: &str) -> String {
    format!("FINAL => {}", input)
}

fn main() {
    // Nodes
    let start_node = Node::new("start".into(), start);
    let parse_node = Node::new("parse".into(), parse);
    let normalize_node = Node::new("normalize".into(), normalize);
    let validate_node = Node::new("validate".into(), validate);
    let enrich_node = Node::new("enrich".into(), enrich);
    let merge_node = Node::new("merge".into(), merge);
    let finalize_node = Node::new("finalize".into(), finalize);

    // Edges
    let edges = vec![
        Edge::new(start_node.clone(), parse_node.clone()),
        // fan-out
        Edge::new(parse_node.clone(), normalize_node.clone()),
        Edge::new(parse_node.clone(), validate_node.clone()),
        Edge::new(parse_node.clone(), enrich_node.clone()),
        // fan-in
        Edge::new(normalize_node.clone(), merge_node.clone()),
        Edge::new(validate_node.clone(), merge_node.clone()),
        Edge::new(enrich_node.clone(), merge_node.clone()),
        Edge::new(merge_node.clone(), finalize_node.clone()),
    ];

    let workflow = Workflow::new(edges);

    let output = workflow.run();
    println!("{}", output);
}

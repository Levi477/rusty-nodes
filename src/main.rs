mod nodeTemplate;
use nodeTemplate::Node;

fn startingFunction(ip: &str) -> String {
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
    let startingNode = Node::new("startingNode".to_string(), startingFunction);
    let node1 = Node::new("node1".to_string(), add1);
    let node2 = Node::new("node2".to_string(), add2);

    // arange nodes in an array in the order of execution
    let nodes = vec![startingNode, node1, node2];

    // define global context
    let mut currentInput = String::new();
    let mut currentOutput = String::new();

    // make a loop to execute nodes in order
    for node in nodes {
        currentInput = currentOutput.clone();
        currentOutput = node.execute(&currentInput);
    }

    // print the final output
    println!("{}", currentOutput);
}

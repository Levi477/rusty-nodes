pub struct Node {
    name: String,
    execute: fn(&str) -> String,
}

impl Node {
    pub fn new(name: String, execute: fn(&str) -> String) -> Self {
        Node { name, execute }
    }
}

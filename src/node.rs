#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Node {
    pub name: String,
    pub execute: fn(&str) -> String,
    pub is_checkpoint: bool,
}

impl Node {
    pub fn new(name: String, execute: fn(&str) -> String) -> Self {
        Node {
            name,
            execute,
            is_checkpoint: false,
        }
    }
}

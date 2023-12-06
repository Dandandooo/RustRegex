use std::collections::HashMap;
use std::clone::Clone;

#[derive(Debug, Clone, Eq)]
pub struct Node {
    pub(crate) id: i32,
    pub(crate) is_end: bool,
    pub(crate) transition: HashMap<String, usize>,
    pub(crate) epsilon_transitions: Vec<usize>
}


impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id;
    }
}

impl Node {
    pub fn new(is_end: bool, id: i32) -> Node {
        let m: HashMap<String, usize> = HashMap::new();
        let v: Vec<usize> = Vec::new();
        Node {
            id: id,
            is_end: is_end,
            transition: m,
            epsilon_transitions: v
        }
    }

    pub fn new_full(is_end: bool, transition: HashMap<String, usize>, epsilon_transitions: Vec<usize>, id: i32) -> Node {
        Node {
            id: id,
            is_end: is_end,
            transition: transition,
            epsilon_transitions: epsilon_transitions
        }
    }
}
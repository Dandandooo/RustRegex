use std::collections::HashMap;
use std::clone::Clone;

#[derive(Debug, Clone, Eq)]
pub struct Node {
    id: i32,
    is_end: bool,
    transition: HashMap<String, *mut Node>,
    epsilon_transitions: Vec<*mut Node>
}


impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id;
    }
}

impl Node {
    pub fn new(is_end: bool, id: i32) -> Node {
        let m: HashMap<String, *mut Node> = HashMap::new();
        let v: Vec<*mut Node> = Vec::new();
        Node {
            id: id,
            is_end: is_end,
            transition: m,
            epsilon_transitions: v
        }
    }

    pub fn new_full(is_end: bool, transition: HashMap<String, *mut Node>, epsilon_transitions: Vec<*mut Node>, id: i32) -> Node {
        Node {
            id: id,
            is_end: is_end,
            transition: transition,
            epsilon_transitions: epsilon_transitions
        }
    }
}

#[derive(Debug)]
pub struct NFA {
    all_nodes: Vec<Node>,
    start: *mut Node,
    end: *mut Node
}

impl NFA {
    pub fn new(postfixExp: String) -> NFA {
        let mut nfa = NFA {
            all_nodes: Vec::new(),
            start: &mut Node::new(false, 0) as *mut Node,
            end: &mut Node::new(false, 0) as *mut Node
        };
        let mut id = 0;
        if postfixExp == "" {
            nfa.fromEpsilon(&mut id);
            return nfa;
        }
        
        let mut stack: Vec<(*mut Node, *mut Node)> = Vec::new();
        
        for token in postfixExp.chars() {
            if(token == '*') {
                let mut n = stack.pop().unwrap();
                nfa.closure(n, &mut id);
                stack.push((nfa.start, nfa.end));
            } else if (token == '|') {
                let mut right = stack.pop().unwrap();
                let mut left = stack.pop().unwrap();
                nfa.union(left, right, &mut id);
                stack.push((nfa.start, nfa.end));
            } else if (token == '.') {
                let mut right = stack.pop().unwrap();
                let mut left = stack.pop().unwrap();
                nfa.concat(left, right);
                stack.push((nfa.start, nfa.end));
            } else {
                nfa.fromSymbol(token.to_string(), &mut id);
                stack.push((nfa.start, nfa.end));
            }
        }
                
        let x = stack.pop();
        nfa.start = x.unwrap().0;
        nfa.end = x.unwrap().1;
        return nfa;
    }

    pub fn fromEpsilon(&mut self, id: &mut i32) {
        let mut start = Node::new(false, *id);
        *id += 1;
        let mut end = Node::new(true, *id);
        *id += 1;
        self.all_nodes.push(start);
        self.all_nodes.push(end);
        let length = self.all_nodes.len();
        let startPtr = &mut self.all_nodes[length - 2] as *mut Node;
        let endPtr = &mut self.all_nodes[length - 1] as *mut Node;

        self.addEpsilonTransition(startPtr, endPtr);
        
        
        self.start = startPtr;
        self.end = endPtr;
    }

    pub fn fromSymbol(&mut self, symbol: String, id: &mut i32) {
        let mut start = Node::new(false, *id);
        *id += 1;
        let mut end = Node::new(true, *id);
        *id += 1;
        self.all_nodes.push(start);
        self.all_nodes.push(end);
        let length = self.all_nodes.len();
        let startPtr = &mut self.all_nodes[length - 2] as *mut Node;
        let endPtr = &mut self.all_nodes[length - 1] as *mut Node;
        //Start to end
        self.addTransition(startPtr, endPtr, symbol);
        
        
        
        self.start = startPtr;
        self.end = endPtr;
    }

    pub fn addEpsilonTransition(&mut self, from: *mut Node, to: *mut Node) {
       unsafe {
            (*from).epsilon_transitions.push(to);
        }
    }

    pub fn addTransition(&mut self, from: *mut Node, to: *mut Node, symbol: String) {
        unsafe {
            let el = (*from).transition.entry(symbol).or_insert(to);
            *el = to;    
        }
    }

    pub fn closure(&mut self, mut nfa: (*mut Node, *mut Node), id: &mut i32) {
        let mut start = Node::new(false, *id);
        *id += 1;
        let mut end = Node::new(true, *id);
        *id += 1;
        self.all_nodes.push(start);
        self.all_nodes.push(end);

        let length = self.all_nodes.len();
        let startPtr = &mut self.all_nodes[length - 2] as *mut Node;
        let endPtr = &mut self.all_nodes[length - 1] as *mut Node;  
        
        unsafe {
            (*nfa.1).is_end = false;
            self.addEpsilonTransition(nfa.1, endPtr);

            self.addEpsilonTransition(nfa.1, nfa.0);

            // Start to end node defined
            self.addEpsilonTransition(startPtr, endPtr);
            self.addEpsilonTransition(startPtr, nfa.0);
        }

        self.start = startPtr;
        self.end = endPtr;
    }

    pub fn concat(&mut self, mut first: (*mut Node, *mut Node), mut second: (*mut Node, *mut Node)) {
        self.addEpsilonTransition(first.1, second.0);
        
        unsafe {
            (*first.1).is_end = false;
        }
        
        self.start = first.0;
        self.end = second.1;
    }

    pub fn union(&mut self, mut first: (*mut Node, *mut Node), mut second: (*mut Node, *mut Node), id: &mut i32) {
        let mut start = Node::new(false, *id);
        *id += 1;
        let mut end = Node::new(true, *id);
        *id += 1;
        self.all_nodes.push(start);
        self.all_nodes.push(end);

        let length = self.all_nodes.len();
        let startPtr = &mut self.all_nodes[length - 2] as *mut Node;
        let endPtr = &mut self.all_nodes[length - 1] as *mut Node;  

        self.addEpsilonTransition(startPtr, first.0);
        self.addEpsilonTransition(startPtr, second.0);
        unsafe {
            self.addEpsilonTransition(first.1, endPtr);
            (*first.1).is_end = false;
            self.addEpsilonTransition(second.1, endPtr);
            (*second.1).is_end = false;
        }
    
        self.start = startPtr;
        self.end = endPtr;
    }
}

pub fn addNextState(state: *mut Node, next_states: &mut Vec<*mut Node>, visited: &mut Vec<*mut Node>) {
    unsafe {
        if ((*state).epsilon_transitions.len() > 0) {
            for st in &(*state).epsilon_transitions {
                if (!visited.clone().contains(&st)) {
                    visited.push((*st).clone());
                    addNextState(*st, next_states, visited);
                }
            }
        } else {
            next_states.push(state);
        }
    }
}

pub fn search(nfa: (*mut Node, *mut Node), word:String) -> bool{
    unsafe {
        let mut currentStates: Vec<*mut Node> = Vec::new();
        let mut visited: Vec<*mut Node> = Vec::new(); 
        addNextState(nfa.0, &mut currentStates, &mut visited);

        for symbol in word.chars() {
            let mut nextStates: Vec<*mut Node> = Vec::new();
            visited.clear(); 

            for state in &currentStates {
                let node = state;
                let nextState = (**node).transition.get(&symbol.to_string()); // This line sometimes causes STATUS_ACCESS_VIOLATION
                if nextState.is_some() {
                    addNextState(*nextState.unwrap(), &mut nextStates, &mut visited.clone());
                }
            }
            currentStates = nextStates;
        }

        for state in currentStates {
            if (*state).is_end {
                return true;
            }
        }
        return false;
    }
}

fn main() {
    let nfa = NFA::new("ab|*c.".to_string()); // Same as (a∣b)*c
    println!("{}", search((nfa.start, nfa.end), "abbabababbac".to_string()));
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_closure() {
        let nfa = NFA::new("a*".to_string());
        assert!(search((nfa.start, nfa.end), "".to_string()));
        assert!(search((nfa.start, nfa.end), "a".to_string()));
        assert!(search((nfa.start, nfa.end), "aa".to_string()));
        assert!(search((nfa.start, nfa.end), "aaaaaaaa".to_string()));
        assert!(!search((nfa.start, nfa.end), "b".to_string()));
        assert!(!search((nfa.start, nfa.end), "dsfsf".to_string()));
        assert!(!search((nfa.start, nfa.end), "abaa".to_string()));
    }


    #[test]
    fn test_concat() {
        let nfa = NFA::new("ab.".to_string());
        assert!(search((nfa.start, nfa.end), "ab".to_string()));
        assert!(!search((nfa.start, nfa.end), "".to_string()));
        assert!(!search((nfa.start, nfa.end), "a".to_string()));
        assert!(!search((nfa.start, nfa.end), "aba".to_string()));
        assert!(!search((nfa.start, nfa.end), "dfhrs".to_string()));
    }

    #[test]
    fn test_union() {
        let nfa = NFA::new("ab|".to_string()); // Same as a|b
        assert!(search((nfa.start, nfa.end), "a".to_string()));
        assert!(search((nfa.start, nfa.end), "b".to_string()));
        assert!(!search((nfa.start, nfa.end), "ab".to_string()));
        assert!(!search((nfa.start, nfa.end), "".to_string()));
        assert!(!search((nfa.start, nfa.end), "aba".to_string()));
        assert!(!search((nfa.start, nfa.end), "dfhrs".to_string()));
    }

    fn test_closure_and_concat() {
        let nfa = NFA::new("a*b.".to_string());
        assert!(search((nfa.start, nfa.end), "ab".to_string()));
        assert!(search((nfa.start, nfa.end), "b".to_string()));
        assert!(search((nfa.start, nfa.end), "aaaaaaaab".to_string()));
        assert!(!search((nfa.start, nfa.end), "".to_string()));
        assert!(!search((nfa.start, nfa.end), "aba".to_string()));
        assert!(!search((nfa.start, nfa.end), "dfhrs".to_string()));
        assert!(!search((nfa.start, nfa.end), "aa".to_string()));
    }

    fn test_all() {
        let nfa = NFA::new("ab∣*c.".to_string()); // Same as (a∣b)*c
        assert!(search((nfa.start, nfa.end), "c".to_string()));
        assert!(search((nfa.start, nfa.end), "aaaaaac".to_string()));
        assert!(search((nfa.start, nfa.end), "bbbbbbbbbc".to_string()));
        assert!(search((nfa.start, nfa.end), "aaabbbaababac".to_string()));
        assert!(!search((nfa.start, nfa.end), "".to_string()));
        assert!(!search((nfa.start, nfa.end), "aba".to_string()));
        assert!(!search((nfa.start, nfa.end), "dfhrs".to_string()));
        assert!(!search((nfa.start, nfa.end), "ca".to_string()));
    }

}




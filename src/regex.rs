
use std::clone::Clone;

// pub mod node;
// use rust_regex::Node;
use super::node::Node;

#[derive(Debug)]
pub struct Regex {
    pub(crate) all_nodes: Vec<Node>,
    pub(crate) start: usize,
    pub(crate) end: usize
}

impl Regex {
    pub fn new(regex_str: String) -> Regex {
        let mut nfa = Regex {
            all_nodes: Vec::new(),
            start: 0,
            end: 0
        };
        let postfixexp = nfa.to_postfix_insert_concat(regex_str);
        let mut id = 0;
        if postfixexp == "" {
            nfa.from_epsilon(&mut id);
            return nfa;
        }
        
        let mut stack: Vec<(usize, usize)> = Vec::new();
        
        let mut i = 0;
        while i < postfixexp.chars().count() {
            let token = postfixexp.chars().nth(i).unwrap();
            if token == '*' {
                let n = stack.pop().unwrap();
                nfa.asterisk(n, &mut id);
                stack.push((nfa.start, nfa.end));
            } else if token == '|' {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                nfa.union(left, right, &mut id);
                stack.push((nfa.start, nfa.end));
            } else if token == '{' {
                i += 1;
                let mut lower_bound = -1;
                let mut upper_bound = -1;
                while postfixexp.chars().nth(i).unwrap() != '}' {
                    if postfixexp.chars().nth(i).unwrap() == ',' {
                        if postfixexp.chars().nth(i - 1).unwrap() != '{' {
                            lower_bound = postfixexp.chars().nth(i - 1).unwrap().to_digit(10).unwrap() as i32;
                        }
                        if postfixexp.chars().nth(i + 1).unwrap() != '}' {
                            upper_bound = postfixexp.chars().nth(i + 1).unwrap().to_digit(10).unwrap() as i32;
                        }
                    }
                    i += 1;
                }
                if upper_bound == -1 && lower_bound == -1 {
                    upper_bound = postfixexp.chars().nth(i - 1).unwrap().to_digit(10).unwrap() as i32;
                    lower_bound = postfixexp.chars().nth(i - 1).unwrap().to_digit(10).unwrap() as i32;
                }
                i += 1;
                let mut n = stack.pop().unwrap();
                let mut dup = 0;
                let dup_start = n.0;
                let dup_end = n.1;
                let _nfa_length = nfa.all_nodes.len();
                let curr_nfa = nfa.all_nodes.clone();
                // For {n}
                if lower_bound == upper_bound {
                    while dup < lower_bound - 1 {
                        nfa.add(&curr_nfa, &mut id);
                        // nfa.duplicate(0, nfa_length - 1, &mut id);
                        nfa.concat(n, (n.1 + 1, n.1 + 1 + dup_end - dup_start));
                        n = (nfa.start, nfa.end);
                        dup += 1;
                    }
                    // Stop any more occurences
                    nfa.add(&curr_nfa, &mut id);
                    nfa.concat(n, (n.1 + 1, n.1 + 1 + dup_end - dup_start));
                    nfa.all_nodes[nfa.end].is_end = false;
                    let new_end = Node::new(true, id);
                    id += 1;
                    nfa.all_nodes.push(new_end);
                    let length = nfa.all_nodes.len();
                    nfa.end = length - 1;
                    nfa.add_epsilon_transition(n.1, nfa.end);
                    stack.push((nfa.start, nfa.end));
                    continue;
                }
                // For {n,}
                if upper_bound == -1 {
                    let mut prev_end = nfa.all_nodes.len();
                    while dup < lower_bound - 1 {
                        prev_end = nfa.all_nodes.len();
                        nfa.add(&curr_nfa, &mut id);
                        nfa.concat(n, (n.1 + 1, n.1 + 1 + dup_end - dup_start));
                        n = (nfa.start, nfa.end);
                        dup += 1;
                    }
                    // Allow unlimited occurences
                    let end = Node::new(true, id);
                    id += 1;
                    nfa.all_nodes.push(end);
                    let end_ptr = nfa.all_nodes.len() - 1;  
                    nfa.all_nodes[nfa.end].is_end = false;
                    nfa.add_epsilon_transition(nfa.end, end_ptr);
                    nfa.add_epsilon_transition(nfa.end, prev_end);
                    nfa.end = end_ptr;
                    stack.push((nfa.start, nfa.end));
                    continue;
                }
                // For {n,m}
                if lower_bound > -1 && upper_bound > -1 {
                    while dup < lower_bound - 1 {
                        nfa.add(&curr_nfa, &mut id);
                        nfa.concat(n, (n.1 + 1, n.1 + 1 + dup_end - dup_start));
                        n = (nfa.start, nfa.end);
                        dup += 1;
                    }
                    while dup < upper_bound - 1 {
                        nfa.add(&curr_nfa, &mut id);
                        nfa.concat(n, (n.1 + 1, n.1 + 1 + dup_end - dup_start));
                        nfa.add_epsilon_transition(n.1, nfa.end);
                        n = (nfa.start, nfa.end);
                        dup += 1;
                    }
                    // Stop any more occurences
                    nfa.add(&curr_nfa, &mut id);
                    nfa.concat(n, (n.1 + 1, n.1 + 1 + dup_end - dup_start));
                    nfa.all_nodes[nfa.end].is_end = false;
                    let new_end = Node::new(true, id);
                    id += 1;
                    nfa.all_nodes.push(new_end);
                    let length = nfa.all_nodes.len();
                    nfa.end = length - 1;
                    nfa.add_epsilon_transition(n.1, nfa.end);
                    stack.push((nfa.start, nfa.end));
                    continue;
                }
                // For {,m}
                if lower_bound == -1 && upper_bound > -1 {
                    while dup < upper_bound - 1 {
                        nfa.add(&curr_nfa, &mut id);
                        nfa.concat(n, (n.1 + 1, n.1 + 1 + dup_end - dup_start));
                        nfa.add_epsilon_transition(n.1, nfa.end);
                        n = (nfa.start, nfa.end);
                        dup += 1;
                    }
                    // Stop any more occurences
                    nfa.add(&curr_nfa, &mut id);
                    nfa.concat(n, (n.1 + 1, n.1 + 1 + dup_end - dup_start));
                    nfa.all_nodes[nfa.end].is_end = false;
                    let new_end = Node::new(true, id);
                    id += 1;
                    nfa.all_nodes.push(new_end);
                    let length = nfa.all_nodes.len();
                    nfa.end = length - 1;
                    nfa.add_epsilon_transition(n.1, nfa.end);
                    nfa.all_nodes[nfa.start].is_end = true;
                    stack.push((nfa.start, nfa.end));
                    continue;
                }
                println!("Should not reach here");
            } else if token == '[' {
                let mut characters = String::new();
                i += 1;
                while postfixexp.chars().nth(i).unwrap() != ']' {
                    if postfixexp.chars().nth(i).unwrap() == '-' {
                        let add_str: String = (postfixexp.chars().nth(i - 1).unwrap()..=postfixexp.chars().nth(i + 1).unwrap()).collect();
                        characters.push_str(&add_str);
                        i += 2;
                        continue;
                    }
                    if postfixexp.chars().nth(i).unwrap() != '.' {
                        characters.push(postfixexp.chars().nth(i).unwrap());
                    }
                    i +=1;
                }
                nfa.character_class(characters, &mut id);
                stack.push((nfa.start, nfa.end));
            } else if token == '.' {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                nfa.concat(left, right);
                stack.push((nfa.start, nfa.end));
            } else if token == '+' {
                let n = stack.pop().unwrap();
                nfa.plus(n, &mut id);
                stack.push((nfa.start, nfa.end));
            } else if token == '?' {
                let n = stack.pop().unwrap();
                nfa.question(n, &mut id);
                stack.push((nfa.start, nfa.end));
            } else {
                nfa.from_symbol(token.to_string(), &mut id);
                stack.push((nfa.start, nfa.end));
            }
            i += 1;
        }
        let x = stack.pop();
        nfa.start = x.unwrap().0;
        nfa.end = x.unwrap().1;
        return nfa;
    }

    pub fn all_nodes(&self) -> Vec<Node> {
        return self.all_nodes.clone();
    }

    pub fn start(&self) -> usize{
        return self.start;
    }
    
    pub fn end(&self) -> usize {
        return self.end;
    }

    pub fn add(&mut self, original: &Vec<Node>, id: &mut i32) {
        let offset: usize = self.all_nodes.len();
        for i in 0..(original.len()){
            let el = &original[i];
            let mut n = Node::new(el.is_end, *id);
            *id += 1;
            n.transition = el.transition.clone();
            n.epsilon_transitions = el.epsilon_transitions.clone();
            for (_string, index) in &mut n.transition {
                *index += offset;
            }
            for ep_idx in &mut n.epsilon_transitions {
                *ep_idx += offset;
            }
            self.all_nodes.push(n);
        }
    }

    pub fn duplicate(&mut self, start: usize, end: usize, id: &mut i32) {
        let offset: usize = self.all_nodes.len();
        for i in start..(end + 1){
            let el = &self.all_nodes[i];
            let mut n = Node::new(el.is_end, *id);
            n.transition = el.transition.clone();
            for (_string, index) in &mut n.transition {
                *index += offset;
            }
            for ep_idx in &mut n.epsilon_transitions {
                *ep_idx += offset;
            }
            n.epsilon_transitions = el.epsilon_transitions.clone();
            *id += 1;
            self.all_nodes.push(n);
        }
    }

    pub fn character_class(&mut self, characters: String, id: &mut i32) {
        let start = Node::new(false, *id);
        *id += 1;
        let end = Node::new(true, *id);
        *id += 1;
        self.all_nodes.push(start);
        self.all_nodes.push(end);
        let length = self.all_nodes.len();
        let start_ptr = length - 2;
        let end_ptr = length - 1;
        for token in characters.chars() {
            self.add_transition(start_ptr, end_ptr, token.to_string());
        }
        
        self.start = start_ptr;
        self.end = end_ptr;
    }

    pub fn precedence(&mut self, operator: char) -> i32 {
        match operator {
            '|' => 1,
            '.' => 2,
            '*' => 3,
            _ => 0,
        }
    }
    
    pub fn insert_concat(&mut self, regex: String) -> String {
        let mut output = String::new();
        let mut i = 0;
        while i < regex.chars().count() {
            let token = regex.chars().nth(i).unwrap();
            output.push(token);
            if token == '[' || token == '{' {
                i += 1;
                while regex.chars().nth(i).unwrap() != ']' && regex.chars().nth(i).unwrap() != '}' {
                    output.push(regex.chars().nth(i).unwrap());
                    i += 1;
                }
                output.push(regex.chars().nth(i).unwrap());
                i += 1;
                continue;
            }
            if token == '(' || token == '|' || token == '[' || token == ']' || token == '{' || token == '}' {
                i += 1;
                continue;
            }
            if i < regex.chars().count() - 1 {
                let next = regex.chars().nth(i + 1).unwrap();
                if next == '*' || next == '?' || next == '+' || next == '|' || next == ')' || next == ']' || next == '{' {
                    i += 1;
                    continue;
                }
                output.push('.');
            }
            i += 1;
        }
        return output;
    }
    
    pub fn to_postfix_insert_concat(&mut self, raw_regex: String) -> String {
        let regex = self.insert_concat(raw_regex);
        let mut output = String::new();
        let mut stack: Vec<char> = Vec::new();
        let _i = 0;
    
        for token in regex.chars() {
            if token == '(' {
                stack.push('(');
            } else if token == ')' {
                let mut first = stack.pop();
                while first.is_some() {
                    if first.unwrap() == '(' {
                        break;
                    }
                    output.push(first.unwrap());
                    first = stack.pop();
                }
            } else if token == '|' || token == '.' || token == '*' {
                let mut last = stack.last();
                while last.is_some() {
                    if self.precedence(*last.unwrap()) >= self.precedence(token) {
                        output.push(stack.pop().unwrap());
                    } else {
                        break;
                    }
                    last = stack.last();
                }
                stack.push(token);
            } else {
                output.push(token);
            }
        }
    
        let mut start = stack.pop();
        while start.is_some() {
            output.push(start.unwrap());
            start = stack.pop();
        }
    
        return output;
    }

    pub fn from_epsilon(&mut self, id: &mut i32) {
        let start = Node::new(false, *id);
        *id += 1;
        let end = Node::new(true, *id);
        *id += 1;
        self.all_nodes.push(start);
        self.all_nodes.push(end);
        let length = self.all_nodes.len();
        let start_ptr = length - 2;
        let end_ptr = length - 1;

        self.add_epsilon_transition(start_ptr, end_ptr);
        
        
        self.start = start_ptr;
        self.end = end_ptr;
    }

    pub fn from_symbol(&mut self, symbol: String, id: &mut i32) {
        let start = Node::new(false, *id);
        *id += 1;
        let end = Node::new(true, *id);
        *id += 1;
        self.all_nodes.push(start);
        self.all_nodes.push(end);
        let length = self.all_nodes.len();
        let start_ptr = length - 2;
        let end_ptr = length - 1;
        //Start to end
        self.add_transition(start_ptr, end_ptr, symbol);
        
        
        
        self.start = start_ptr;
        self.end = end_ptr;
    }

    pub fn add_epsilon_transition(&mut self, from: usize, to: usize) {
        self.all_nodes[from].epsilon_transitions.push(to);
    }

    //May need to check if multiple transitions
    pub fn add_transition(&mut self, from: usize, to: usize, symbol: String) {
        *self.all_nodes[from].transition.entry(symbol).or_insert(to) = to;
    }

    pub fn asterisk(&mut self, nfa: (usize, usize), id: &mut i32) {
        let start = Node::new(false, *id);
        *id += 1;
        let end = Node::new(true, *id);
        *id += 1;
        self.all_nodes.push(start);
        self.all_nodes.push(end);

        let length = self.all_nodes.len();
        let start_ptr = length - 2;
        let end_ptr = length - 1;  

        self.all_nodes[nfa.1].is_end = false;
        self.add_epsilon_transition(nfa.1, end_ptr);
        self.add_epsilon_transition(nfa.1, nfa.0);
        self.add_epsilon_transition(start_ptr, end_ptr);
        self.add_epsilon_transition(start_ptr, nfa.0);

        self.start = start_ptr;
        self.end = end_ptr;
    }

    pub fn question(&mut self, nfa: (usize, usize), id: &mut i32) {
        let start = Node::new(false, *id);
        *id += 1;
        let end = Node::new(true, *id);
        *id += 1;
        self.all_nodes.push(start);
        self.all_nodes.push(end);

        let length = self.all_nodes.len();
        let start_ptr = length - 2;
        let end_ptr = length - 1;  

        self.all_nodes[nfa.1].is_end = false;
        self.add_epsilon_transition(nfa.1, end_ptr);
        self.add_epsilon_transition(start_ptr, end_ptr);
        self.add_epsilon_transition(start_ptr, nfa.0);

        self.start = start_ptr;
        self.end = end_ptr;
    }

    pub fn plus(&mut self, nfa: (usize, usize), id: &mut i32) {
        let start = Node::new(false, *id);
        *id += 1;
        let end = Node::new(true, *id);
        *id += 1;
        self.all_nodes.push(start);
        self.all_nodes.push(end);

        let length = self.all_nodes.len();
        let start_ptr = length - 2;
        let end_ptr = length - 1;  

        self.all_nodes[nfa.1].is_end = false;
        self.add_epsilon_transition(nfa.1, end_ptr);
        self.add_epsilon_transition(nfa.1, nfa.0);
        self.add_epsilon_transition(start_ptr, nfa.0);

        self.start = start_ptr;
        self.end = end_ptr;
    }

    pub fn concat(&mut self, first: (usize, usize), second: (usize, usize)) {
        self.add_epsilon_transition(first.1, second.0);

        self.all_nodes[first.1].is_end = false;
        
        self.start = first.0;
        self.end = second.1;
    }

    pub fn union(&mut self, first: (usize, usize), second: (usize, usize), id: &mut i32) {
        let start = Node::new(false, *id);
        *id += 1;
        let end = Node::new(true, *id);
        *id += 1;
        self.all_nodes.push(start);
        self.all_nodes.push(end);

        let length = self.all_nodes.len();
        let start_ptr = length - 2;
        let end_ptr = length - 1;  
        
        self.add_epsilon_transition(start_ptr, first.0);
        self.add_epsilon_transition(start_ptr, second.0);
        self.add_epsilon_transition(first.1, end_ptr);
        self.all_nodes[first.1].is_end = false;
        self.add_epsilon_transition(second.1, end_ptr);
        self.all_nodes[second.1].is_end = false;
    
        self.start = start_ptr;
        self.end = end_ptr;
    }

    pub fn add_next_state(&mut self, state: usize, next_states: &mut Vec<usize>, visited: &mut Vec<usize>) {
        if self.all_nodes[state].epsilon_transitions.len() > 0 {
            for st in &(self.all_nodes[state].clone().epsilon_transitions) {
                if !visited.clone().contains(st) {
                    visited.push((st).clone());
                    self.add_next_state(*st, next_states, visited);
                }
            }
        } else {
            next_states.push(state);
        }
    }

    pub fn search(&mut self, nfa: (usize, usize), word:String) -> bool{
        let mut current_states: Vec<usize> = Vec::new();
        let mut visited: Vec<usize> = Vec::new(); 
        self.add_next_state(nfa.0, &mut current_states, &mut visited);

        for symbol in word.chars() {
            let mut next_states: Vec<usize> = Vec::new();
            visited.clear(); 

            for state in &current_states {    
                if self.all_nodes[*state].transition.get(&symbol.to_string()).is_some() {
                    self.add_next_state(*(self.all_nodes[*state].transition.get(&symbol.to_string())).unwrap(), &mut next_states, &mut visited.clone());
                }
            }
            current_states = next_states;
        }

        for state in current_states {
            if self.all_nodes[state].is_end {
                return true;
            }
        }
        return false;
    }
}
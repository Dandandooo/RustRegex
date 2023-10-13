use std::collections::HashMap;
use std::collections::HashSet;
use std::vec::Vec;

pub struct NFA {
    regex: String,
    flags: HashSet<char>,
    data: HashMap<i8, NfaNode>,
    range: HashSet<char>,
}

struct NfaNode {
    id: i8,
    is_terminal: bool,
    paths: Vec<(char, &'static i8)>,
}

impl NFA {
    fn new() -> Self {
        NFA {
            regex: String::new(),
            flags: HashSet::new(),
            data: HashMap::from([(0, NfaNode::new(0))]),
            range: HashSet::new(),
        }
    }

    fn get_front(&self) -> Option<&NfaNode> {
        self.data.get(&0)
    }

    fn add_node(&mut self, node: NfaNode) {
        let id = node.id;
        self.data.insert(id, node.clone());
        for (ch, _) in node.paths {
            if !self.range.contains(&ch) {
                self.range.insert(ch);
            }
        }
    }


}

impl NfaNode {
    fn new(id: i8) -> Self {
        NfaNode {
            id,
            is_terminal: false,
            paths: Vec::new(),
        }
    }

    fn clone(&self) -> Self {
        NfaNode {
            id: self.id.clone(),
            is_terminal: self.is_terminal.clone(),
            paths: self.paths.clone(),
        }
    }

    fn is_dfa(&self) -> bool {
        let mut unique: HashSet<char> = HashSet::new();
        for (ch, _) in &self.paths {
            if unique.contains(ch) { return false; }
            if *ch == '\0' { return false;}
            unique.insert(*ch);
        } true
    }

    fn add_path(&mut self, ch: char, node_id: &'static i8) {
        self.paths.push((ch, node_id));
    }


}

pub struct DFA {
    regex: String,
    flags: HashSet<char>,
    data: HashMap<i8, DfaNode>,
    range: HashSet<char>,
}

struct DfaNode {
    id: i8,
    is_terminal: bool,
    paths: HashMap<char, &'static i8>
}

impl DFA {
    fn new(regex: String, flags: HashSet<char>) -> Self {
        DFA {
            regex,
            flags,
            data: HashMap::new(),
            range: HashSet::new(),
        }
    }

    fn get_front(&self) -> Option<&DfaNode> {
        self.data.get(&0)
    }

    fn add_node(&mut self, node: DfaNode) {
        self.data.insert(node.id, node.clone());
        for (ch, _) in node.paths {
            if !self.range.contains(&ch) {
                self.range.insert(ch);
            }
        }
    }

    fn get_node(&self, id: &i8) -> Option<&DfaNode> {
        self.data.get(id)
    }

    fn get_node_mut(&mut self, id: &i8) -> Option<&mut DfaNode> {
        self.data.get_mut(id)
    }

    // Displaying each node and it's connections. '_' means there is no connection for that character.
    fn display(&self) {
        println!("{}{}", self.regex, self.to_string(true));
    }

    fn to_string(&self, in_color: bool) -> String {
        let mut output = String::new();
        let keys: Vec<char> = self.range.iter().map(|x: &char| *x).collect::<Vec<char>>();
        output += "  ";
        for ch in &self.range {
            output += &format!("  {}", ch);
        }
        for (id, node) in &self.data {
            output += "\n";
            if node.is_terminal && in_color {
                output += &format!("\x1b[0;31m{}\x1b[0m", id);
            } else {
                output += &format!("{}", id);
            }
            for ch in &keys {
                if node.paths.contains_key(ch) {
                    output += &format!("  {}", node.follow(ch));
                } else {
                    output += "  _";
                }
            }

        }
        output
    }

    // Returns true if the input string matches the DFA.
    pub fn matches(&self, input: &str) -> bool {
        let mut current_node = self.get_front().unwrap();
        for ch in input.chars() {
            if current_node.paths.contains_key(&ch) {
                current_node = self.get_node(current_node.follow(&ch)).unwrap();
            } else {
                return false;
            }
        }
        current_node.is_terminal
    }
}

impl DfaNode {
    fn new(id: i8) -> Self {
        DfaNode {
            id,
            is_terminal: false,
            paths: HashMap::new(),
        }
    }

    fn clone(&self) -> Self {
        DfaNode {
            id: self.id.clone(),
            is_terminal: self.is_terminal.clone(),
            paths: self.paths.clone(),
        }
    }

    fn add_path(&mut self, ch: char, node_id: &'static i8) {
        self.paths.insert(ch, node_id);
    }

    fn from(nfa_node: NfaNode) -> Self {
        let mut paths: HashMap<char, &'static i8> = HashMap::new();
        for (ch, node_id) in nfa_node.paths {
            if !paths.contains_key(&ch) {
                paths.insert(ch, node_id);
            }
        }
        DfaNode {
            id: nfa_node.id,
            is_terminal: nfa_node.is_terminal,
            paths,
        }
    }

    fn follow(&self, ch: &char) -> &'static i8 {
        self.paths.get(&ch).unwrap()
    }


}
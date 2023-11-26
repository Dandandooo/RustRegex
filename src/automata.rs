use std::collections::{HashMap, HashSet, VecDeque};
use std::vec::Vec;
use parse_regex::{process_regex, RangeType, Token, TokenQuantifier};
use std::fmt;
pub type Id = u8;

#[derive(Default, PartialEq)]
pub struct NFA {
    pub(crate) regex: String,
    pub(crate) flags: HashSet<char>,
    data: HashMap<Id, NfaNode>,
    cur_last: Id,
}

#[derive(PartialEq, Debug)]
struct NfaNode {
    id: Id,
    is_terminal: bool,
    // Character + Id of the node it points to.
    paths: HashSet<(char, Id)>,
}

impl NFA {
    const EPSILON: char = '\0';
    const WORD_CHAR: char = '🦀';
    const WILD_CARD: char = '🃏';
    const WHITE_SPACE: char = '🫥';
    const NUMBER: char = '💯';

    pub fn from(regex: String) -> Self {
        let (tokens, flags) = process_regex(&regex);
        let mut nfa = NFA::new(regex, flags);
        for token in tokens {
            nfa.add_token(token);
        }
        nfa.data.get_mut(&nfa.cur_last).unwrap().is_terminal = true;
        nfa
    }

    pub fn new(regex: String, flags: HashSet<char>) -> Self {
        NFA {
            regex,
            flags,
            data: HashMap::from([(0, NfaNode::new(0))]),
            cur_last: 0,
        }
    }
    fn next_available_id(&self) -> Id {
        let mut id = 0;
        while self.data.contains_key(&id) {
            id += 1;
        }
        id
    }

    fn add_path(&mut self, from: &Id, to: &Id, ch: char) {
        self.data.get_mut(from).unwrap().add_path(ch, *to);
    }
    fn add_token(&mut self, token: Token) {
        self.add_token_to(token, self.cur_last.clone());
    }

    fn add_token_to(&mut self, token: Token, start_id: Id) {
        match token {
            Token::Normal { token: cur_token, quantifier } => {
                match quantifier {
                    TokenQuantifier::None => {
                        let next_id = self.next_available_id();
                        let next_node = NfaNode::new(next_id);

                        self.add_path(&start_id, &next_id, cur_token);
                        
                        self.data.insert(next_id, next_node);
                        self.cur_last = next_id;
                    },
                    TokenQuantifier::Question => {
                        let next_id = self.next_available_id();
                        let next_node = NfaNode::new(next_id);

                        self.add_path(&start_id, &next_id, cur_token);
                        self.add_path(&start_id, &next_id, NFA::EPSILON);

                        self.data.insert(next_id, next_node);
                        
                        self.cur_last = next_id;
                    },
                    TokenQuantifier::Star => {
                        self.add_path(&start_id, &start_id, cur_token);
                    },
                    TokenQuantifier::Plus => {
                        let next_id = self.next_available_id();
                        let next_node = NfaNode::new(next_id);

                        self.data.insert(next_id, next_node);

                        self.add_path(&start_id, &next_id, cur_token.clone());
                        self.add_path(&next_id, &next_id, cur_token.clone());

                        self.cur_last = next_id;
                    },
                    TokenQuantifier::Range(range_type) => {
                        match range_type {
                            RangeType::Discrete (num_times) => {
                                let mut prev_id = start_id.clone();
                                let mut next_id = self.next_available_id();
                                for _ in 0..num_times {
                                    next_id = self.next_available_id();
                                    let next_node = NfaNode::new(next_id);
                                    self.data.insert(next_id, next_node);
                                    self.add_path(&prev_id, &next_id, cur_token.clone());
                                    prev_id = next_id;
                                }
                                self.cur_last = next_id;
                            },
                            RangeType::LowerBound (lower) => {
                                let mut prev_id = start_id;
                                for _ in 0..lower {
                                    let next_id = self.next_available_id();
                                    let next_node = NfaNode::new(next_id);
                                    self.data.insert(next_id, next_node);
                                    self.add_path(&prev_id, &next_id, cur_token.clone());
                                    prev_id = next_id;
                                }

                                self.add_path(&prev_id, &prev_id, cur_token.clone());
                                self.cur_last = prev_id;
                            },
                            RangeType::UpperBound(upper) => {
                                let mut possible_ids: Vec<Id> = vec![start_id.clone()];

                                let mut prev_id = start_id.clone();
                                for _ in 0..upper {
                                    let next_id = self.next_available_id();
                                    let next_node = NfaNode::new(next_id);
                                    self.add_path(&prev_id, &next_id, cur_token.clone());
                                    self.data.insert(next_id.clone(), next_node);
                                    possible_ids.push(next_id);
                                    prev_id = next_id;
                                }
                                for id in possible_ids {
                                    if id != prev_id {
                                        self.add_path(&id, &prev_id, NFA::EPSILON);
                                    }
                                }

                                self.cur_last = prev_id;
                            },
                            RangeType::Both (lower, upper) => {
                                let mut possible_ids: Vec<Id> = vec![start_id.clone()];

                                let mut prev_id = start_id.clone();
                                let mut next_id = self.next_available_id();
                                for _ in 0..upper {
                                    next_id = self.next_available_id();
                                    let next_node = NfaNode::new(next_id);
                                    self.add_path(&prev_id, &next_id, cur_token.clone());
                                    self.data.insert(next_id.clone(), next_node);
                                    possible_ids.push(next_id);
                                    prev_id = next_id;
                                }
                                for id in possible_ids.iter().skip(lower) {
                                    if id != &prev_id {
                                        self.add_path(&id, &prev_id, NFA::EPSILON);
                                    }
                                }

                                self.cur_last = prev_id;
                            }
                        }
                    }
                } }
            Token::CaptureGroup { sub_groups, quantifier } => {
                for token in &sub_groups {
                    self.add_token(token.clone());
                }
                match quantifier {
                    TokenQuantifier::Plus => {
                        self.add_path(&self.cur_last.clone(), &start_id, NFA::EPSILON);
                    },
                    TokenQuantifier::Star => {
                        let next_id = self.next_available_id();
                        let next_node = NfaNode::new(next_id);
                        self.data.insert(next_id, next_node);
                        self.add_path(&self.cur_last.clone(), &start_id, NFA::EPSILON);
                        self.add_path(&start_id, &next_id, NFA::EPSILON);
                        self.add_path(&self.cur_last.clone(), &next_id, NFA::EPSILON);
                        self.cur_last = next_id;
                    },
                    TokenQuantifier::Question => {
                        self.add_path(&start_id, &self.cur_last.clone(), NFA::EPSILON);
                    },
                    TokenQuantifier::Range (range_type) => match range_type {
                        RangeType::Discrete ( num_times ) => {
                            for _ in 0..num_times {
                                self.add_token(Token::CaptureGroup {
                                    sub_groups: sub_groups.clone(),
                                    quantifier: TokenQuantifier::None,
                                })
                            }
                        },
                        RangeType::LowerBound ( lower_bound) => {
                            for _ in 0..lower_bound {
                                self.add_token(Token::CaptureGroup {
                                    sub_groups: sub_groups.clone(),
                                    quantifier: TokenQuantifier::None,
                                })
                            }
                            self.add_token(Token::CaptureGroup {
                                sub_groups: sub_groups.clone(),
                                quantifier: TokenQuantifier::Plus
                            })
                        },
                        RangeType::UpperBound ( upper_bound) => {
                            for _ in 0..=upper_bound {
                                self.add_token(Token::CaptureGroup {
                                    sub_groups: sub_groups.clone(),
                                    quantifier: TokenQuantifier::Question
                                });
                            }
                        },
                        RangeType::Both ( lower_bound, upper_bound) => {
                            for _ in 0..lower_bound {
                                self.add_token(Token::CaptureGroup {
                                    sub_groups: sub_groups.clone(),
                                    quantifier: TokenQuantifier::None,
                                });
                            }
                            for _ in lower_bound..=upper_bound {
                                self.add_token(Token::CaptureGroup {
                                    sub_groups: sub_groups.clone(),
                                    quantifier: TokenQuantifier::Question,
                                });
                            }
                        }
                    },
                    _ => (),
                }
            },
            Token::CharacterClass { class_options, quantifier } => {
                let next_id = self.next_available_id();
                let next_node = NfaNode::new(next_id);
                self.data.insert(next_id, next_node);
                for char in &class_options {
                    self.add_path(&start_id, &next_id, *char);
                    match quantifier {
                        TokenQuantifier::Plus | TokenQuantifier::Star => self.add_path(&next_id, &next_id, *char),
                        _ => (),
                    }
                }
                match quantifier {
                    TokenQuantifier::Question | TokenQuantifier::Star => {
                        self.add_path(&start_id, &next_id, NFA::EPSILON);
                    },
                    TokenQuantifier::Range (range_type) => match range_type {
                        RangeType::Discrete (num_times) => {
                            for _ in 0..num_times {
                                self.add_token(Token::CharacterClass {
                                    class_options: class_options.clone(),
                                    quantifier: TokenQuantifier::None,
                                });
                            }
                        },
                        RangeType::LowerBound (lower_bound) => {
                            for _ in 0..lower_bound {
                                self.add_token(Token::CharacterClass {
                                    class_options: class_options.clone(),
                                    quantifier: TokenQuantifier::None,
                                });
                            }
                            self.add_token(Token::CharacterClass {
                                class_options: class_options.clone(),
                                quantifier: TokenQuantifier::Star,
                            });
                        },
                        RangeType::UpperBound (upper_bound) => {
                            for _ in 0..=upper_bound {
                                self.add_token(Token::CharacterClass {
                                    class_options: class_options.clone(),
                                    quantifier: TokenQuantifier::Question,
                                });
                            }
                        },
                        RangeType::Both (lower_bound, upper_bound) => {
                            for _ in 0..lower_bound {
                                self.add_token(Token::CharacterClass {
                                    class_options: class_options.clone(),
                                    quantifier: TokenQuantifier::None,
                                });
                            }
                            for _ in lower_bound..=upper_bound {
                                self.add_token(Token::CharacterClass {
                                    class_options: class_options.clone(),
                                    quantifier: TokenQuantifier::Question,
                                });
                            }
                        }
                    },
                    _ => (),
                }
                self.cur_last = next_id;
            },
            Token::Pipe { sub_groups } => {
                let root_id = start_id.clone();
                let mut last_ids : Vec<Id> = sub_groups.iter().map(|_| root_id.clone()).collect();
                for (idx, &ref group) in sub_groups.iter().enumerate().to_owned() {
                    let mut last_id = root_id.clone();
                    for token in group {
                        self.add_token_to(token.clone(), last_id.clone());
                        last_id = self.cur_last.clone();
                    }
                    last_ids[idx] = last_id;
                }
                let next_id = self.next_available_id();
                let next_node = NfaNode::new(next_id);
                self.data.insert(next_id, next_node);
                for id in last_ids {
                    self.add_path(&id, &next_id, NFA::EPSILON);
                }
                self.cur_last = next_id;
            },
        }
    }

    fn get_front(&self) -> &NfaNode {
        self.data.get(&0).unwrap()
    }

    fn has_epsilon(&self) -> bool {
        for (_, node) in &self.data {
            for (ch, _) in &node.paths {
                if *ch == NFA::EPSILON {
                    return true;
                }
            }
        }
        false
    }

    fn remove_epsilons(&mut self) {
        if !self.has_epsilon(){ return }

        let mut lead_map: HashMap<Id, HashSet<Id>> = HashMap::new();
        for (id, node) in self.data.iter() {
            let mut leads = HashSet::<Id>::new();
            self.follow_epsilon(node, &mut leads);
            lead_map.insert(id.clone(), leads);
        }

        for (id, leads) in lead_map.iter() {
            // Close Enclosure here
            for lead in leads {
                let lead_node = self.data[lead].clone();
                if self.data[lead].is_terminal {
                    self.data.entry(id.clone()).or_insert_with(|| lead_node.clone()).is_terminal = true;
                }
                let lead_paths = self.data[lead].paths.clone();
                for (ch, next) in lead_paths {
                    if self.data[id].paths.contains(&(ch, next)) {
                        continue;
                    }
                    if next != *id && ch != NFA::EPSILON{
                        self.data.entry(id.clone()).or_insert_with(|| lead_node.clone()).paths.insert((ch, next));
                    }
                }
            }
        }

        // Actually removing the epsilons from the nodes
        for (_, node) in self.data.iter_mut() {
            for (ch, id) in node.paths.clone().iter().collect::<Vec<&(char, Id)>>() {
                if *ch == NFA::EPSILON { node.paths.remove(&(*ch, *id)); }
            }
        }
    }

    fn follow_epsilon(&self, node: &NfaNode, visited: &mut HashSet<Id>){
        if visited.contains(&node.id) {return}
        visited.insert(node.id);
        for (ch, next) in node.paths.iter() {
            if *ch == NFA::EPSILON {
                self.follow_epsilon(&self.data[next], visited);
            }
        }
    }

    fn has_duplicates(&self) -> bool {
        for (_, node) in self.data.iter() {
            if !node.is_dfa() {return true}
        } false
    }

    fn remove_duplicate_connections(&mut self) {
        while self.has_duplicates() {
            let dupes: HashMap<Id, HashMap<char, HashSet<Id>>> = self.data.iter().map(|(id, node)| (*id, node.find_dupes())).collect();
            let mut to_remove: HashSet<Id> = HashSet::new();

            let cur_id = dupes.keys().min().unwrap().clone();
            let dupe = dupes.get(&cur_id).unwrap();

            for (ch, nexts) in dupe {
                let new_id = self.next_available_id();
                let mut new_node = NfaNode::new(new_id);
                for next in nexts {
                    let next_paths = self.data[next].paths.clone();
                    new_node.paths.extend(next_paths);
                    to_remove.insert(*next);
                }
                self.data.insert(new_id, new_node);

                self.add_path(&cur_id, &new_id, *ch);
            }

            for id in to_remove {
                self.data.remove(&id);
            }
        }
    }

    fn make_dfa(&mut self) {
        self.remove_epsilons();
        self.remove_duplicate_connections();
    }

    fn to_dfa(&self) -> DFA {
        let mut dfa = DFA::new(self.regex.clone(), self.flags.clone());
        let mut new_data : HashMap<Id, DfaNode> = HashMap::new();
        for (id, node) in self.data.iter() {
            new_data.insert(*id, DfaNode::from(node.clone()));
        }
        dfa.data = new_data;

        dfa
    }

}

impl fmt::Debug for NFA {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut nodes_string = String::new();
        let mut available = self.data.keys().collect::<HashSet<_>>();
        while !available.is_empty() {
            let cur_min = available.iter().map(|&x| x).min().unwrap();
            nodes_string.push_str(format!("{cur_min}: {:?}", self.data[cur_min]).as_str());
            available.remove(cur_min);
            nodes_string.push_str(", ");
        }
        write!(f, "NFA {{ regex: {}, flags: {:?}, data: {{ {} }}, cur_last: {} }}", self.regex, self.flags, nodes_string, self.cur_last)
    }
}

impl NfaNode {
    fn new(id: u8) -> Self {
        NfaNode {
            id,
            is_terminal: false,
            paths: HashSet::new(),
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

    fn add_path(&mut self, ch: char, node_id: Id) {
        self.paths.insert((ch, node_id));
    }

    fn find_dupes(&self) -> HashMap<char, HashSet<Id>> {
        let mut dupe_map: HashMap<char, HashSet<Id>> = HashMap::new();
        for (ch, next) in self.paths.iter() {
            if dupe_map.contains_key(ch) {
                dupe_map.get_mut(ch).unwrap().insert(*next);
            } else {
                dupe_map.insert(*ch, HashSet::from([*next]));
            }
        }
        dupe_map.iter().filter(|(_, v)| v.len() > 1).map(|(k, v)| (*k, v.clone())).collect()
    }

}

#[derive(PartialEq, Debug, Clone)]
pub struct DFA {
    regex: String,
    flags: HashSet<char>,
    data: HashMap<Id, DfaNode>,
}

#[derive(PartialEq, Debug, Clone)]
struct DfaNode {
    id: Id,
    is_terminal: bool,
    paths: HashMap<char, Id>
}

impl DFA {
    pub fn from(regex: String) -> Self{
        let mut nfa = NFA::from(regex);
        nfa.make_dfa();
        let mut dfa = nfa.to_dfa();
        dfa.purge();
        dfa.clean();
        dfa
    }

    fn new(regex: String, flags: HashSet<char>) -> Self {
        DFA {
            regex,
            flags,
            data: HashMap::new(),
        }
    }

    fn get_front(&self) -> &DfaNode {
        self.data.get(&0).unwrap()
    }

    fn get_node(&self, id: &u8) -> Option<&DfaNode> {
        self.data.get(id)
    }

    fn get_node_mut(&mut self, id: &u8) -> &mut DfaNode {
        self.data.get_mut(id).unwrap()
    }

    // Returns true if the input string matches the DFA.
    pub fn matches(&self, input: &str) -> bool {
        let mut current_node = self.get_front();
        for ch in input.chars() {
            if current_node.paths.contains_key(&ch) {
                current_node = self.get_node(&current_node.follow(&ch)).unwrap();
            } else {
                return false;
            }
        }
        current_node.is_terminal
    }

    pub fn purge(&mut self) {
        let mut reached: HashSet<Id> = HashSet::new();
        self.spread_through(&0, &mut reached);
        let all_keys : HashSet<Id> = self.data.keys().map(|&x| x.clone()).collect();
        let to_remove: HashSet<Id> = &all_keys - &reached;
        for node_id in to_remove {
            self.data.remove(&node_id);
        }
    }

    fn clean(&mut self) {
        let mut visited: HashSet<Id> = HashSet::new();
        let mut to_visit: VecDeque<Id> = VecDeque::from(vec![0]);
        while !to_visit.is_empty() {
            let cur = to_visit.pop_front().unwrap();
            if visited.contains(&cur) { continue; }
            visited.insert(cur.clone());
            for (_, next) in self.data[&cur].paths.iter() {
                if !visited.contains(next) {
                    to_visit.push_back(*next);
                }
            }
        }

        let to_remove = &self.data.keys().map(|x| x.clone()).collect::<HashSet<Id>>() - &visited;
        for id in to_remove {
            self.data.remove(&id);
        }
    }

    fn spread_through(&self, cur: &Id, visited: &mut HashSet<Id>) {
        if visited.contains(cur) { return; }
        visited.insert(cur.clone());
        for (_, next) in self.data[cur].paths.iter() {
            self.spread_through(next, visited);
        }
    }
}

impl DfaNode {
    fn new(id: u8) -> Self {
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

    fn add_path(&mut self, ch: char, node_id: Id) {
        self.paths.insert(ch, node_id);
        todo!();
    }

    fn from(nfa_node: NfaNode) -> Self {
        let mut paths: HashMap<char, Id> = HashMap::new();
        for (ch, node_id) in nfa_node.paths {
            if !paths.contains_key(&ch) {
                paths.insert(ch, node_id);
            } else {panic!("Duplicate paths in NFA node!");}
        }
        DfaNode {
            id: nfa_node.id,
            is_terminal: nfa_node.is_terminal,
            paths,
        }
    }

    fn follow(&self, ch: &char) -> &Id {
        self.paths.get(ch).unwrap()
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use parse_regex::process_regex;

    macro_rules! set {
        ($($x:expr),*) => {
            {
                #[allow(unused_mut)]
                let mut temp_set = HashSet::new();
                $(
                    temp_set.insert($x);
                )*
                temp_set
            }
        };
    }

    #[test]
    fn nfa_all_normal() {
        let regex = r"ab*c+d?e{2,4}f{,2}g{2}h{2,}";
        let (_, flags) = process_regex(regex);
        let nfa = NFA::from(regex.to_string());
        let correct_nfa = NFA {
            regex: regex.to_string(),
            flags: flags.clone(),
            data: HashMap::from([
                (0, NfaNode {
                    id: 0,
                    is_terminal: false,
                    paths: set![('a', 1)],
                }),
                (1, NfaNode {
                    id: 1,
                    is_terminal: false,
                    paths: set![('b', 1), ('c', 2)],
                }),
                (2, NfaNode {
                    id: 2,
                    is_terminal: false,
                    paths: set![('c', 2), ('d', 3), (NFA::EPSILON, 3)],
                }),
                (3, NfaNode {
                    id: 3,
                    is_terminal: false,
                    paths: set![('e', 4)],
                }),
                (4, NfaNode {
                    id: 4,
                    is_terminal: false,
                    paths: set![('e', 5)],
                }),
                (5, NfaNode {
                    id: 5,
                    is_terminal: false,
                    paths: set![('e', 6), (NFA::EPSILON, 7)]
                }),
                (6, NfaNode {
                    id: 6,
                    is_terminal: false,
                    paths: set![('e', 7), (NFA::EPSILON, 7)]
                }),
                (7, NfaNode {
                    id: 7,
                    is_terminal: false,
                    paths: set![('f', 8), (NFA::EPSILON, 9)]
                }),
                (8, NfaNode {
                    id: 8,
                    is_terminal: false,
                    paths: set![('f', 9), (NFA::EPSILON, 9)]
                }),
                (9, NfaNode {
                    id: 9,
                    is_terminal: false,
                    paths: set![('g', 10)]
                }),
                (10, NfaNode {
                    id: 10,
                    is_terminal: false,
                    paths: set![('g', 11)]
                }),
                (11, NfaNode {
                    id: 11,
                    is_terminal: false,
                    paths: set![('h', 12)]
                }),
                (12, NfaNode {
                    id: 12,
                    is_terminal: false,
                    paths: set![('h', 13)]
                }),
                (13, NfaNode {
                    id: 13,
                    is_terminal: true,
                    paths: set![('h', 13)]
                })
            ]),
            cur_last: 13,

        };
        assert_eq!(nfa, correct_nfa);
    }

    #[test]
    fn nfa_normal() {
        let regex = r"ab";
        let (_, flags) = process_regex(regex);
        let nfa = NFA::from(regex.to_string());
        let correct_nfa = NFA {
            regex: regex.to_string(),
            flags: flags.clone(),
            data: HashMap::from([
                (0, NfaNode {
                id: 0,
                is_terminal: false,
                paths: set![('a', 1)],
            }),
            (1, NfaNode {
                id: 1,
                is_terminal: false,
                paths: set![('b', 2)],
            }),
            (2, NfaNode {
                id: 2,
                is_terminal: true,
                paths: set![],
            }),
            ]),
            cur_last: 2,
        };
        assert_eq!(nfa, correct_nfa);
    }

    #[test]
    fn nfa_normal_star() {
        let regex = r"b*";
        let (_, flags) = process_regex(regex);
        let nfa = NFA::from(regex.to_string());
        let correct_nfa = NFA {
            regex: regex.to_string(),
            flags: flags.clone(),
            data: HashMap::from([
                (0, NfaNode {
                    id: 0,
                    is_terminal: true,
                    paths: set![('b', 0)],
                }),
            ]),
            cur_last: 0,
        };
        assert_eq!(nfa, correct_nfa);
    }

    #[test]
    fn nfa_normal_plus() {
        let regex = r"b+";
        let (_, flags) = process_regex(regex);
        let nfa = NFA::from(regex.to_string());
        let correct_nfa = NFA {
            regex: regex.to_string(),
            flags: flags.clone(),
            data: HashMap::from([
                (0, NfaNode {
                    id: 0,
                    is_terminal: false,
                    paths: set![('b', 1)],
                }),
                (1, NfaNode {
                    id: 1,
                    is_terminal: true,
                    paths: set![('b', 1)],
                }),
            ]),
            cur_last: 1,
        };
        assert_eq!(nfa, correct_nfa);
    }

    #[test]
    fn nfa_normal_question() {
        let regex = r"b?";
        let (_, flags) = process_regex(regex);
        let nfa = NFA::from(regex.to_string());
        let correct_nfa = NFA {
            regex: regex.to_string(),
            flags: flags.clone(),
            data: HashMap::from([
                (0, NfaNode {
                    id: 0,
                    is_terminal: false,
                    paths: set![('b', 1), (NFA::EPSILON, 1)],
                }),
                (1, NfaNode {
                    id: 1,
                    is_terminal: true,
                    paths: set![],
                }),
            ]),
            cur_last: 1,
        };
        assert_eq!(nfa, correct_nfa);
    }

    #[test]
    fn nfa_normal_range_both() {
        let regex = r"b{2,4}";
        let (_, flags) = process_regex(regex);
        let nfa = NFA::from(regex.to_string());
        let correct_nfa = NFA {
            regex: regex.to_string(),
            flags: flags.clone(),
            data: HashMap::from([
                (0, NfaNode {
                    id: 0,
                    is_terminal: false,
                    paths: set![('b', 1)],
                }),
                (1, NfaNode {
                    id: 1,
                    is_terminal: false,
                    paths: set![('b', 2)],
                }),
                (2, NfaNode {
                    id: 2,
                    is_terminal: false,
                    paths: set![('b', 3), (NFA::EPSILON, 4)],
                }),
                (3, NfaNode {
                    id: 3,
                    is_terminal: false,
                    paths: set![('b', 4), (NFA::EPSILON, 4)],
                }),
                (4, NfaNode {
                    id: 4,
                    is_terminal: true,
                    paths: set![],
                }),
            ]),
            cur_last: 4,
        };
        assert_eq!(nfa, correct_nfa);
    }

    #[test]
    fn nfa_normal_range_lower() {
        let regex = r"b{2,}";
        let (_, flags) = process_regex(regex);
        let nfa = NFA::from(regex.to_string());
        let correct_nfa = NFA {
            regex: regex.to_string(),
            flags: flags.clone(),
            data: HashMap::from([
                (0, NfaNode {
                    id: 0,
                    is_terminal: false,
                    paths: set![('b', 1)],
                }),
                (1, NfaNode {
                    id: 1,
                    is_terminal: false,
                    paths: set![('b', 2)],
                }),
                (2, NfaNode {
                    id: 2,
                    is_terminal: true,
                    paths: set![('b', 2)],
                }),
            ]),
            cur_last: 2,
        };
        assert_eq!(nfa, correct_nfa);
    }

    #[test]
    fn nfa_normal_range_upper() {
        let regex = r"b{,4}";
        let (_, flags) = process_regex(regex);
        let nfa = NFA::from(regex.to_string());
        let correct_nfa = NFA {
            regex: regex.to_string(),
            flags,
            data: HashMap::from([
                (0, NfaNode {
                    id: 0,
                    is_terminal: false,
                    paths: set![('b', 1), (NFA::EPSILON, 4)],
                }),
                (1, NfaNode {
                    id: 1,
                    is_terminal: false,
                    paths: set![('b', 2), (NFA::EPSILON, 4)],
                }),
                (2, NfaNode {
                    id: 2,
                    is_terminal: false,
                    paths: set![('b', 3), (NFA::EPSILON, 4)],
                }),
                (3, NfaNode {
                    id: 3,
                    is_terminal: false,
                    paths: set![('b', 4), (NFA::EPSILON, 4)],
                }),
                (4, NfaNode {
                    id: 4,
                    is_terminal: true,
                    paths: set![],
                })
            ]),
            cur_last: 4,
        };
        assert_eq!(nfa, correct_nfa);
    }

    #[test]
    fn nfa_normal_range_discrete() {
        let regex = r"b{2}";
        let (_, flags) = process_regex(regex);
        let nfa = NFA::from(regex.to_string());
        let correct_nfa =  NFA {
            regex: regex.to_string(),
            flags: flags.clone(),
            data: HashMap::from([
                (0, NfaNode {
                id: 0,
                is_terminal: false,
                paths: set![('b', 1)],
            }),
            (1, NfaNode {
                id: 1,
                is_terminal: false,
                paths: set![('b', 2)],
            }),
            (2, NfaNode {
                id: 2,
                is_terminal: true,
                paths: set![],
            }),
            ]),
            cur_last: 2,
        };
        assert_eq!(nfa, correct_nfa);
    }

    #[test]
    fn nfa_basic_pipe () {
        let regex = r"ab|cd";
        let (_, flags) = process_regex(regex);
        let nfa = NFA::from(regex.to_string());
        let correct_nfa = NFA {
            regex: regex.to_string(),
            flags: flags.clone(),
            data: HashMap::from([
                (0, NfaNode {
                    id: 0,
                    is_terminal: false,
                    paths: set![('a', 1), ('c', 3)],
                }),
                (1, NfaNode {
                    id: 1,
                    is_terminal: false,
                    paths: set![('b', 2)],
                }),
                (2, NfaNode {
                    id: 2,
                    is_terminal: false,
                    paths: set![(NFA::EPSILON, 5)],
                }),
                (3, NfaNode {
                    id: 3,
                    is_terminal: false,
                    paths: set![('d', 4)],
                }),
                (4, NfaNode {
                    id: 4,
                    is_terminal: false,
                    paths: set![(NFA::EPSILON, 5)],
                }),
                (5, NfaNode {
                    id: 5,
                    is_terminal: true,
                    paths: set![],
                }),
            ]),
            cur_last: 5,
        };
        assert_eq!(nfa, correct_nfa);
    }

    #[test]
    fn nfa_capture_basic_star() {
        let regex = "(ab)*";
        let (_, flags) = process_regex(regex);
        let nfa = NFA::from(regex.to_string());
        let correct_nfa = NFA {
            regex: regex.to_string(),
            flags: flags.clone(),
            data: HashMap::from([
                (0, NfaNode {
                    id: 0,
                    is_terminal: false,
                    paths: set![('a', 1), (NFA::EPSILON, 3)],
                }),
                (1, NfaNode {
                    id: 1,
                    is_terminal: false,
                    paths: set![('b', 2)],
                }),
                (2, NfaNode {
                    id: 2,
                    is_terminal: false,
                    paths: set![(NFA::EPSILON, 0), (NFA::EPSILON, 3)],
                }),
                (3, NfaNode {
                    id: 3,
                    is_terminal: true,
                    paths: set![],
                }),
            ]),
            cur_last: 3,
        };
        assert_eq!(nfa, correct_nfa);
    }

    #[test]
    fn nfa_comprehensive_rangeless() {
        let regex = r"a+b*c?d(ab|cd)|efgh[j-l]+";
        let nfa = NFA::from(regex.to_string());
        let correct_nfa = NFA {
            regex: regex.to_string(),
            flags: HashSet::new(),
            data: HashMap::from([
                (0, NfaNode {
                    id: 0,
                    is_terminal: false,
                    paths: set![('a', 1), ('e', 9)],
                }),
                (1, NfaNode {
                    id: 1,
                    is_terminal: false,
                    paths: set![('a', 1), ('b', 1), ('c', 2), (NFA::EPSILON, 2)],
                }),
                (2, NfaNode {
                    id: 2,
                    is_terminal: false,
                    paths: set![('d', 3)],
                }),
                (3, NfaNode {
                    id: 3,
                    is_terminal: false,
                    paths: set![('a', 4), ('c', 6)],
                }),
                (4, NfaNode {
                    id: 4,
                    is_terminal: false,
                    paths: set![('b', 5)],
                }),
                (5, NfaNode {
                    id: 5,
                    is_terminal: false,
                    paths: set![(NFA::EPSILON, 8)],
                }),
                (6, NfaNode {
                    id: 6,
                    is_terminal: false,
                    paths: set![('d', 7)],
                }),
                (7, NfaNode {
                    id: 7,
                    is_terminal: false,
                    paths: set![(NFA::EPSILON, 8)],
                }),
                (8, NfaNode {
                    id: 8,
                    is_terminal: false,
                    paths: set![(NFA::EPSILON, 14)],
                }),
                (9, NfaNode {
                    id: 9,
                    is_terminal: false,
                    paths: set![('f', 10)],
                }),
                (10, NfaNode {
                    id: 10,
                    is_terminal: false,
                    paths: set![('g', 11)],
                }),
                (11, NfaNode {
                    id: 11,
                    is_terminal: false,
                    paths: set![('h', 12)],
                }),
                (12, NfaNode {
                    id: 12,
                    is_terminal: false,
                    paths: set![('j', 13),('k', 13),('l', 13)],
                }),
                (13, NfaNode {
                    id: 13,
                    is_terminal: false,
                    paths: set![('j', 13),('k', 13),('l', 13), (NFA::EPSILON, 14)],
                }),
                (14, NfaNode {
                    id: 14,
                    is_terminal: true,
                    paths: set![],
                })
            ]),
            cur_last: 14
        };
        assert_eq!(nfa, correct_nfa);
    }

    #[test]
    fn nfa_remove_epsilon_comprehensive() {
        let regex = r"a+b*c?d(ab|cd)|efgh[j-l]+";
        let mut nfa = NFA::from(regex.to_string());
        nfa.remove_epsilons();
        let correct_nfa = NFA {
            regex: regex.to_string(),
            flags: HashSet::new(),
            data: HashMap::from([
                (0, NfaNode {
                    id: 0,
                    is_terminal: false,
                    paths: set![('a', 1), ('e', 9)],
                }),
                (1, NfaNode {
                    id: 1,
                    is_terminal: false,
                    paths: set![('a', 1), ('b', 1), ('c', 2), ('d', 3)],
                }),
                (2, NfaNode {
                    id: 2,
                    is_terminal: false,
                    paths: set![('d', 3)],
                }),
                (3, NfaNode {
                    id: 3,
                    is_terminal: false,
                    paths: set![('a', 4), ('c', 6)],
                }),
                (4, NfaNode {
                    id: 4,
                    is_terminal: false,
                    paths: set![('b', 5)],
                }),
                (5, NfaNode {
                    id: 5,
                    is_terminal: true,
                    paths: set![],
                }),
                (6, NfaNode {
                    id: 6,
                    is_terminal: false,
                    paths: set![('d', 7)],
                }),
                (7, NfaNode {
                    id: 7,
                    is_terminal: true,
                    paths: set![],
                }),
                (8, NfaNode {
                    id: 8,
                    is_terminal: true,
                    paths: set![],
                }),
                (9, NfaNode {
                    id: 9,
                    is_terminal: false,
                    paths: set![('f', 10)],
                }),
                (10, NfaNode {
                    id: 10,
                    is_terminal: false,
                    paths: set![('g', 11)],
                }),
                (11, NfaNode {
                    id: 11,
                    is_terminal: false,
                    paths: set![('h', 12)],
                }),
                (12, NfaNode {
                    id: 12,
                    is_terminal: false,
                    paths: set![('j', 13),('k', 13),('l', 13)],
                }),
                (13, NfaNode {
                    id: 13,
                    is_terminal: true,
                    paths: set![('j', 13),('k', 13),('l', 13)],
                }),
                (14, NfaNode {
                    id: 14,
                    is_terminal: true,
                    paths: set![],
                })
            ]),
            cur_last: 14
        };
        assert_eq!(nfa, correct_nfa);
    }

    #[test]
    fn dfa_functionality_simple() {
        let regex = r"ab";
        let dfa = DFA::from(regex.to_string());
        assert!(dfa.matches("ab"));
        assert!(!dfa.matches("a"));
        assert!(!dfa.matches("b"));
        assert!(!dfa.matches("ba"));
        assert!(!dfa.matches("abc"));
    }

    #[test]
    fn dfa_functionality_star() {
        let regex = r"ab*";
        let dfa = DFA::from(regex.to_string());
        assert!(dfa.matches("a"));
        assert!(dfa.matches("ab"));
        assert!(dfa.matches("abb"));
        assert!(dfa.matches("abbbbbb"));
    }

    #[test]
    fn dfa_functionality_plus() {
        let regex = r"ab+";
        let dfa = DFA::from(regex.to_string());
        assert!(!dfa.matches("a"));
        assert!(dfa.matches("ab"));
        assert!(dfa.matches("abb"));
        assert!(dfa.matches("abbbbbb"));
        assert!(!dfa.matches("abbba"));
    }

    #[test]
    fn dfa_functionality_question() {
        let regex = r"ab?c";
        let dfa = DFA::from(regex.to_string());
        assert!(dfa.matches("ac"));
        assert!(dfa.matches("abc"));
        assert!(!dfa.matches("abbc"));
        assert!(!dfa.matches("ab"));
    }

    #[test]
    fn dfa_functionality_capture_basic() {
        let regex = r"(abc)";
        let dfa = DFA::from(regex.to_string());
        assert!(dfa.matches("abc"));
        assert!(!dfa.matches("ab"));
        assert!(!dfa.matches("bc"));
        assert!(!dfa.matches("ac"));
        assert!(!dfa.matches("abcd"));
    }

    #[test]
    fn dfa_functionality_capture_inset() {
        let regex = r"a(ab)b";
        let dfa = DFA::from(regex.to_string());
        assert!(dfa.matches("aabb"));
        assert!(!dfa.matches("ab"));
    }

    #[test]
    fn dfa_functionality_capture_nested() {
        let regex = r"(a(bc))";
        let dfa = DFA::from(regex.to_string());
        assert!(dfa.matches("abc"));
        assert!(!dfa.matches("a"));
        assert!(!dfa.matches("bc"));
    }

    #[test]
    fn dfa_functionality_capture_starred() {
        let regex = r"c(ab)*";
        let dfa = DFA::from(regex.to_string());
        assert!(dfa.matches("c"));
        assert!(dfa.matches("cab"));
        assert!(dfa.matches("cabababab"));
        assert!(!dfa.matches("caba"));
    }

    #[test]
    fn dfa_functionality_capture_plussed() {
        let regex = r"c(ab)+";
        let dfa = DFA::from(regex.to_string());
        assert!(!dfa.matches("c"));
        assert!(dfa.matches("cab"));
        assert!(dfa.matches("cabababab"));
        assert!(!dfa.matches("caba"));
    }

    #[test]
    fn dfa_functionality_capture_questioned() {
        let regex = r"c(ab)?";
        let dfa = DFA::from(regex.to_string());
        assert!(dfa.matches("c"));
        assert!(dfa.matches("cab"));
        assert!(!dfa.matches("cabababab"));
        assert!(!dfa.matches("caba"));
    }

    #[test]
    fn dfa_functionality_normal_ranges() {
        let regex_discrete = r"b{2}";
        let regex_lower = r"b{2,}";
        let regex_upper = r"b{,4}";
        let regex_both = r"b{2,4}";
        let dfa_discrete = DFA::from(regex_discrete.to_string());
        let dfa_lower = DFA::from(regex_lower.to_string());
        let dfa_upper = DFA::from(regex_upper.to_string());
        let dfa_both = DFA::from(regex_both.to_string());
        assert!(dfa_discrete.matches("bb"));
        assert!(!dfa_discrete.matches("b"));
        assert!(!dfa_discrete.matches("bbb"));
        assert!(!dfa_discrete.matches("bbbb"));
        assert!(dfa_lower.matches("bb"));
        assert!(dfa_lower.matches("bbb"));
        assert!(!dfa_lower.matches("b"));
        assert!(dfa_upper.matches(""));
        assert!(dfa_upper.matches("b"));
        assert!(dfa_upper.matches("bb"));
        assert!(dfa_upper.matches("bbb"));
        assert!(dfa_upper.matches("bbbb"));
    }
}
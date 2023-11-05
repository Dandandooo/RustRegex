pub mod automata;
pub mod parse_regex;
pub mod steps;

use steps::make_dfa;

fn main() {
    let regex: String = "/[a-z]+/i".to_string();
    let nfa = make_dfa(regex);
}
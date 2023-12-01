pub mod automata;
pub mod parse_regex;

use std::path::Path;
use std::fs::File;
use std::io::BufRead;
use automata::DFA;

pub fn test_regex(file_path: &str) {
    let path = Path::new(file_path);
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => panic!("Could not open file: {}", file_path),
    };

    println!("Testing regex from file: {}", file_path);

    let reader = std::io::BufReader::new(file);
    let mut lines = reader.lines().into_iter();
    let regex = lines.next().unwrap().unwrap();

    println!("Regex: {regex}");

    let dfa = DFA::from(regex);
    dfa.display();
    for line in lines.skip(1).map(|l| l.unwrap()) {
        let mut iter = line.split(' ');
        let string = iter.next().unwrap();
        let expected = iter.next().unwrap() == "true";
        print_test_case(&dfa, string, expected);
    }

}

fn print_test_case(dfa: &DFA, string: &str, expected: bool) {
    let result = dfa.matches(string);
    if result == expected {
        println!("\x1b[32mPASS\x1b[37m:\x1b[0m {} \x1b[37m->\x1b[0m {}", string, result);
    } else {
        println!("\x1b[31mFAIL\x1b[37m:\x1b[0m {} \x1b[37m->\x1b[0m {} \x1b[37m(expected {expected})\x1b[0m", string, result);
    }
}
extern crate rust_regex;

use rust_regex::test_regex;

fn main() {
    let filename = "test_cases.txt";
    test_regex(filename);
}
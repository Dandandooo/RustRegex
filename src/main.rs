use std::io;
use rust_regex::regex::Regex;

fn main() {
    println!("Please note, this program assumes only valid input is given. You can look at test cases for example use in the main file.");
    println!(r"Enter a regex expression (or \exit to exit): ");
    let mut regex: String = String::new();
    io::stdin().read_line(&mut regex).expect("failed to readline");

    while regex.trim() != r"\exit" {
        let mut nfa = Regex::new(regex.trim().to_string());
        println!(r"Enter a string to match against the regex expression (or \back to enter another regex): ");
        let mut match_string: String = String::new();
        io::stdin().read_line(&mut match_string).expect("failed to readline");
        while match_string.trim() != r"\back" {
            println!("Match result - {}", nfa.search((nfa.start(), nfa.end()), match_string.trim().to_string()));
            println!(r"Enter a string to match against the regex expression (or \back to enter another regex): ");
            match_string = "".to_string();
            io::stdin().read_line(&mut match_string).expect("failed to readline");
        }

        println!(r"Enter a regex expression (or \exit to exit): ");
        regex = "".to_string();
        io::stdin().read_line(&mut regex).expect("failed to readline");
    }
    
    print!("Program terminated");
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_asterisk() {
        let mut nfa = Regex::new("a*".to_string());
        assert!(nfa.search((nfa.start(), nfa.end()), "".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "a".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "aa".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "aaaaaaaa".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "b".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "dsfsf".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "abaa".to_string()));
    }


    #[test]
    fn test_concat() {
        let mut nfa = Regex::new("ab".to_string());
        assert!(nfa.search((nfa.start(), nfa.end()), "ab".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "a".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "aba".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "dfhrs".to_string()));
    }

    #[test]
    fn test_union() {
        let mut nfa = Regex::new("a|b".to_string()); // Same as a|b
        assert!(nfa.search((nfa.start(), nfa.end()), "a".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "b".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "ab".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "aba".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "dfhrs".to_string()));
    }

    #[test]
    fn test_asterisk_and_concat() {
        let mut nfa = Regex::new("a*b".to_string());
        assert!(nfa.search((nfa.start(), nfa.end()), "ab".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "b".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "aaaaaaaab".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "aba".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "dfhrs".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "aa".to_string()));
    }

    #[test]
    fn test_all() {
        let mut nfa = Regex::new("(a|b)*c".to_string()); // Same as (a∣b)*c
        assert!(nfa.search((nfa.start(), nfa.end()), "c".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "ac".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "aaaaaac".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "bbbbbbbbbc".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "aaabbbaababac".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "aba".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "dfhrs".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "ca".to_string()));
    }

    #[test]
    fn test_group_union() {
        let mut nfa = Regex::new("(ab)|(cd)".to_string()); // Same as (ab)|(cd)
        assert!(nfa.search((nfa.start(), nfa.end()), "ab".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "cd".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "abcd".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "aba".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "dfhrs".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "a".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "b".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "c".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "d".to_string()));
    }

    #[test]
    fn test_group() {
        let mut nfa = Regex::new("a(b|c)d".to_string()); // Same as a(b|c)d
        assert!(nfa.search((nfa.start(), nfa.end()), "abd".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "acd".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "ad".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "bd".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "cd".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "a".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "b".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "c".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "d".to_string()));
    }

    #[test]
    fn test_class() {
        let mut nfa = Regex::new("[abc]".to_string()); // Same as a(b|c)d
        assert!(nfa.search((nfa.start(), nfa.end()), "a".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "b".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "c".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "ab".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "bc".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "ca".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "abc".to_string()));
    }

    #[test]
    fn test_with_class() {
        let mut nfa = Regex::new("b[abc]*".to_string()); // Same as a(b|c)d
        assert!(nfa.search((nfa.start(), nfa.end()), "b".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "ba".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "bbbbbbbb".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "bacbcbbbacca".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "bcd".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "asfsdafs".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "aa".to_string()));
    }

    #[test]
    fn test_range() {
        let mut nfa = Regex::new("[B-H]".to_string()); // Same as a(b|c)d
        assert!(nfa.search((nfa.start(), nfa.end()), "C".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "B".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "H".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "D".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "E".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "F".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "G".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "b".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "BC".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "A".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "I".to_string()));
    }

    #[test]
    fn test_with_range() {
        let mut nfa = Regex::new("b[a-z]+".to_string()); // Same as a(b|c)d
        assert!(!nfa.search((nfa.start(), nfa.end()), "b".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "ba".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "bbbbbbbb".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "bacbcbbbacca".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "bcasiohfhiaff".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "basjkdkmakcmascnalkefi".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "b*".to_string()));
    }

    #[test]
    fn exact_range() {
        let mut nfa = Regex::new("b{4}".to_string()); // Same as a(b|c)d
        assert!(nfa.search((nfa.start(), nfa.end()), "bbbb".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "bbb".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "bbbbb".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "bb".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "b".to_string()));
    }

    #[test]
    fn range_lower_bound() {
        let mut nfa = Regex::new("b{3,}".to_string()); // Same as a(b|c)d
        assert!(nfa.search((nfa.start(), nfa.end()), "bbb".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "bbbbb".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "bbbb".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "bbbbbbbbbbbbbbbbbbbb".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "bb".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "b".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "a".to_string()));
    }

    #[test]
    fn range_bound() {
        let mut nfa = Regex::new("b{3,6}".to_string()); // Same as a(b|c)d
        assert!(nfa.search((nfa.start(), nfa.end()), "bbb".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "bbbb".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "bbbbb".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "bbbbbb".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "bb".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "b".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "bbbbbbb".to_string()));
    }

    #[test]
    fn range_upper_bound() {
        let mut nfa = Regex::new("b{,5}".to_string()); // Same as a(b|c)d
        assert!(nfa.search((nfa.start(), nfa.end()), "".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "b".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "bb".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "bbb".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "bbbb".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "bbbbb".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "bbbbbb".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "bbbbbbbbbbbbbb".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "a".to_string()));
    }

    #[test]
    fn dfa_functionality_simple() {
        let regex = r"ab";
        let mut nfa = Regex::new(regex.to_string());
        assert!(nfa.search((nfa.start(), nfa.end()), "ab".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "a".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "b".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "ba".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "abc".to_string()));
    }

    #[test]
    fn dfa_functionality_star() {
        let regex = r"ab*";
        let mut nfa = Regex::new(regex.to_string());
        assert!(nfa.search((nfa.start(), nfa.end()), "a".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "ab".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "abb".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "abbbbbb".to_string()));
    }

    #[test]
    fn dfa_functionality_plus() {
        let regex = r"ab+";
        let mut nfa= Regex::new(regex.to_string());
        assert!(!nfa.search((nfa.start(), nfa.end()), "a".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "ab".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "abb".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "abbbbbb".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "abbba".to_string()));
    }

    #[test]
    fn dfa_functionality_question() {
        let regex = r"ab?c";
        let mut nfa= Regex::new(regex.to_string());
        assert!(nfa.search((nfa.start(), nfa.end()), "ac".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "abc".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "abbc".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "ab".to_string()));
    }

    #[test]
    fn dfa_functionality_capture_basic() {
        let regex = r"(abc)";
        let mut nfa= Regex::new(regex.to_string());
        assert!(nfa.search((nfa.start(), nfa.end()), "abc".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "ab".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "bc".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "ac".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "abcd".to_string()));
    }

    #[test]
    fn dfa_functionality_capture_inset() {
        let regex = r"a(ab)b";
        let mut nfa= Regex::new(regex.to_string());
        assert!(nfa.search((nfa.start(), nfa.end()), "aabb".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "ab".to_string()));
    }

    #[test]
    fn dfa_functionality_capture_nested() {
        let regex = r"(a(bc))";
        let mut nfa= Regex::new(regex.to_string());
        assert!(nfa.search((nfa.start(), nfa.end()), "abc".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "a".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "bc".to_string()));
    }

    #[test]
    fn dfa_functionality_capture_starred() {
        let regex = r"c(ab)*";
        let mut nfa= Regex::new(regex.to_string());
        assert!(nfa.search((nfa.start(), nfa.end()), "c".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "cab".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "cabababab".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "caba".to_string()));
    }

    #[test]
    fn dfa_functionality_capture_plussed() {
        let regex = r"c(ab)+";
        let mut nfa= Regex::new(regex.to_string());
        assert!(!nfa.search((nfa.start(), nfa.end()), "c".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "cab".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "cabababab".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "caba".to_string()));
    }

    #[test]
    fn dfa_functionality_capture_questioned() {
        let regex = r"c(ab)?";
        let mut nfa= Regex::new(regex.to_string());
        assert!(nfa.search((nfa.start(), nfa.end()), "c".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "cab".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "cabababab".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "caba".to_string()));
    }

    #[test]
    fn dfa_functionality_normal_ranges() {
        let regex_discrete = r"b{2}";
        let regex_lower = r"b{2,}";
        let regex_upper = r"b{,4}";
        let regex_both = r"b{2,4}";
        let mut dfa_discrete = Regex::new(regex_discrete.to_string());
        let mut dfa_lower = Regex::new(regex_lower.to_string());
        let mut dfa_upper = Regex::new(regex_upper.to_string());
        let mut dfa_both = Regex::new(regex_both.to_string());
        assert!(dfa_discrete.search((dfa_discrete.start(), dfa_discrete.end()), "bb".to_string()));
        assert!(!dfa_discrete.search((dfa_discrete.start(), dfa_discrete.end()), "b".to_string()));
        assert!(!dfa_discrete.search((dfa_discrete.start(), dfa_discrete.end()), "bbb".to_string()));
        assert!(!dfa_discrete.search((dfa_discrete.start(), dfa_discrete.end()), "bbbb".to_string()));
        assert!(dfa_lower.search((dfa_lower.start(), dfa_lower.end()), "bb".to_string()));
        assert!(dfa_lower.search((dfa_lower.start(), dfa_lower.end()), "bbb".to_string()));
        assert!(!dfa_lower.search((dfa_lower.start(), dfa_lower.end()), "b".to_string()));
        assert!(dfa_upper.search((dfa_upper.start(), dfa_upper.end()), "".to_string()));
        assert!(dfa_upper.search((dfa_upper.start(), dfa_upper.end()), "b".to_string()));
        assert!(dfa_upper.search((dfa_upper.start(), dfa_upper.end()), "bb".to_string()));
        assert!(dfa_upper.search((dfa_upper.start(), dfa_upper.end()), "bbb".to_string()));
        assert!(dfa_upper.search((dfa_upper.start(), dfa_upper.end()), "bbbb".to_string()));
        assert!(!dfa_upper.search((dfa_upper.start(), dfa_upper.end()), "bbbbb".to_string()));
        assert!(!dfa_both.search((dfa_both.start(), dfa_both.end()), "".to_string()));
        assert!(!dfa_both.search((dfa_both.start(), dfa_both.end()), "b".to_string()));
        assert!(dfa_both.search((dfa_both.start(), dfa_both.end()), "bb".to_string()));
        assert!(dfa_both.search((dfa_both.start(), dfa_both.end()), "bbb".to_string()));
        assert!(dfa_both.search((dfa_both.start(), dfa_both.end()), "bbbb".to_string()));
        assert!(!dfa_both.search((dfa_both.start(), dfa_both.end()), "bbbbb".to_string()));
    }

    #[test]
    fn dfa_functionality_class_basic() {
        let regex = "[a-d]";
        let mut nfa= Regex::new(regex.to_string());
        assert!(nfa.search((nfa.start(), nfa.end()), "a".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "b".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "c".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "d".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "e".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "aa".to_string()));
    }

    #[test]
    fn dfa_functionality_class_starred() {
        let regex = "[a-c]*";
        let mut nfa= Regex::new(regex.to_string());
        assert!(nfa.search((nfa.start(), nfa.end()), "".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "a".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "b".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "c".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "aa".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "aabbabcaccabbacabacbccabcbbacbca".to_string()));
    }

    #[test]
    fn dfa_functionality_class_plussed() {
        let regex = "[a-c]+";
        let mut nfa= Regex::new(regex.to_string());
        assert!(!nfa.search((nfa.start(), nfa.end()), "".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "a".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "b".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "c".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "aa".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "aabbabcaccabbacabacbccabcbbacbca".to_string()));
    }

    #[test]
    fn dfa_functionality_class_questioned() {
        let regex = "[a-c]?";
        let mut nfa= Regex::new(regex.to_string());
        assert!(nfa.search((nfa.start(), nfa.end()), "".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "a".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "b".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "c".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "aa".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "aabbabcaccabbacabacbccabcbbacbca".to_string()));
    }

    #[test]
    fn dfa_functionality_class_range_discrete() {
        let regex = "[a-c]{2}";
        let mut nfa= Regex::new(regex.to_string());
        assert!(nfa.search((nfa.start(), nfa.end()), "aa".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "ab".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "ac".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "ba".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "bb".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "bc".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "ca".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "cb".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "cc".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "a".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "b".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "c".to_string()));
    }

    #[test]
    fn dfa_functionality_class_range_lower() {
        let regex = "[a-c]{2,}";
        let mut nfa= Regex::new(regex.to_string());
        assert!(nfa.search((nfa.start(), nfa.end()), "aa".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "ab".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "ac".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "ba".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "bb".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "bc".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "ca".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "cb".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "cc".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "acabcabbccaca".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "a".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "b".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "c".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "".to_string()));
    }

    #[test]
    fn dfa_functionality_class_range_upper() {
        let regex = "[a-c]{,2}";
        let mut nfa= Regex::new(regex.to_string());
        assert!(nfa.search((nfa.start(), nfa.end()), "aa".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "ab".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "ac".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "ba".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "bb".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "bc".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "ca".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "cb".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "cc".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "acabcabbccaca".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "a".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "b".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "c".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "".to_string()));
    }

    #[test]
    fn dfa_functionality_class_range_both() {
        let regex = "[a-c]{2,3}";
        let mut nfa= Regex::new(regex.to_string());
        assert!(nfa.search((nfa.start(), nfa.end()), "aa".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "ab".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "ac".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "ba".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "bb".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "bc".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "ca".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "cb".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "cc".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "aaa".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "aba".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "aca".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "baa".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "bba".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "bca".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "caa".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "cba".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "cca".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "aab".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "abb".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "acb".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "bab".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "bbb".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "bcb".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "cab".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "cbb".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "ccb".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "aac".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "abc".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "acc".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "bac".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "bbc".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "bcc".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "cac".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "cbc".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "ccc".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "acabcabbccaca".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "a".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "b".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "c".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "".to_string()));
    }

    #[test]
    fn dfa_functionality_capture_range_discrete() {
        let regex = r"(aa){2}";
        let mut nfa= Regex::new(regex.to_string());
        assert!(!nfa.search((nfa.start(), nfa.end()), "aa".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "aaaa".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "aaaaaa".to_string()));
    }

    #[test]
    fn dfa_functionality_capture_range_lower() {
        let regex = r"(aa){2,}";
        let mut nfa= Regex::new(regex.to_string());
        assert!(nfa.search((nfa.start(), nfa.end()), "aaaa".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "aaaaaa".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "aaaaaaaaa".to_string()));
    }

    #[test]
    fn dfa_functionality_capture_range_upper() {
        let regex = r"(aa){,2}";
        let mut nfa= Regex::new(regex.to_string());
        assert!(nfa.search((nfa.start(), nfa.end()), "".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "aa".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "aaaa".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "aaaaaa".to_string()));
    }

    #[test]
    fn dfa_functionality_capture_range_both() {
        let regex = r"(aa){2,3}";
        let mut nfa= Regex::new(regex.to_string());
        assert!(!nfa.search((nfa.start(), nfa.end()), "aa".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "aaaa".to_string()));
        assert!(nfa.search((nfa.start(), nfa.end()), "aaaaaa".to_string()));
        assert!(!nfa.search((nfa.start(), nfa.end()), "aaaaaaaaa".to_string()));
    }
}
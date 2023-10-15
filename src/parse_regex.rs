use std::vec::Vec;
use std::collections::HashSet;
use std::collections::HashMap;

enum TokenType {
    CaptureGroup,
    CharacterClass,
    Pipe,
    Normal,
}
struct Token {
    token: String,
    token_type: TokenType,
    quantifier: String,
    sub_group: Vec<Token>,
}
pub fn tokenize_regex(regex: &str) -> (Vec<Vec<String>>, HashSet<char>) {
    // index of the right most slash in the regular expression (the one before the flags)
    let r_index = regex.rfind(|c| c == '/').unwrap();

    // the flags come after the right most slash, so we separate them from the regex
    let flags = regex[r_index + 1..].chars().collect::<HashSet<char>>();

    // the regex is everything before the right most slash and after the first slash
    let ex = regex[1..r_index].to_string();
    
    (tokenize(ex), flags)
}

fn split_to_parts(regex: String) -> Vec<String> {
    let mut parts: Vec<String> = Vec::new();
    let mut chars = regex.chars().peekable();

    let mut cur_char = chars.next();
    while cur_char != None {
        let cc = cur_char.unwrap();
        if cc == '\\' {
            let nc = chars.peek();
            if nc == None { panic!("Invalid regex"); }
            let ncc = chars.next().unwrap();
            let mut quant = String::new();
            if chars.peek() != None {
                let quant_char = chars.next().unwrap();
                if quant_char == '?' || quant_char == '*' || quant_char == '+' {
                    quant.push(quant_char);
                    if chars.peek() != None {
                        let quant_char = chars.next().unwrap();
                        quant.push(quant_char);
                    }
                }
            }

        }
    }

    todo!();
}

fn parts_to_token(parts: Vec<String>) -> Vec<Token> {
    todo!();
}

fn tokenize(regex: String) -> Vec<Vec<String>> {
    todo!();
}

fn capture_group(regex: String, starting_index: usize) -> String {
    let end_parentheses = match regex.as_bytes()[starting_index] as char {
        '(' => ')',
        '[' => ']',
        _ => panic!("Invalid capture group"),
    };
    let mut cur_depth = 0;

    let mut parentheses: String = String::new();

    for (idx, char) in regex.char_indices().skip(starting_index + 1) {
        if char == '(' || char == '[' {
            cur_depth += 1;
            parentheses.push(char);
        } else if char == end_parentheses && cur_depth == 0 {
            return regex[starting_index..idx + 1].to_string();
        } else if char == ')' && char == parentheses.as_bytes()[parentheses.len() - 1] as char {
            cur_depth -= 1;
            parentheses.pop();
        } else if char == ']' && char == parentheses.as_bytes()[parentheses.len() - 1] as char {
            cur_depth -= 1;
            parentheses.pop();
        }
    }
    panic!("Invalid capture group");
    
}

fn find_paren_match(regex: &String, starting_index: usize) -> usize {
    let end_parentheses = match regex.as_bytes()[starting_index] as char {
        '(' => ')',
        '[' => ']',
        _ => panic!("Invalid capture group"),
    };
    let mut cur_depth = 0;

    let mut parentheses: String = String::new();

    for (idx, char) in regex.char_indices().skip(starting_index) {
        if char == '(' || char == '[' {
            cur_depth += 1;
            parentheses.push(char);
        } else if char == end_parentheses && cur_depth == 0 {
            return idx;
        } else if char == ')' && char == parentheses.as_bytes()[parentheses.len() - 1] as char {
            cur_depth -= 1;
            parentheses.pop();
        } else if char == ']' && char == parentheses.as_bytes()[parentheses.len() - 1] as char {
            cur_depth -= 1;
            parentheses.pop();
        }
    }
    panic!("Invalid capture group");
}

// Will check if the regex contains a pipe that is not in a capture group or character class
fn check_pipe(regex: &String) -> bool {
    let mut cur_depth = 0;
    let mut parentheses: String = String::new();
    for char in regex.chars() {
        if char == '(' || char == '[' {
            cur_depth += 1;
            parentheses.push(char);
        } else if char == ')' && char == parentheses.as_bytes()[parentheses.len() - 1] as char {
            cur_depth -= 1;
            parentheses.pop();
        } else if char == ']' && char == parentheses.as_bytes()[parentheses.len() - 1] as char {
            cur_depth -= 1;
            parentheses.pop();
        } else if char == '|' && cur_depth == 0 {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_split1() {
        let regex = "a(b|c)d".to_string();
        let tokens = split_to_parts(regex);
        assert_eq!(tokens, vec!["a", "(b|c)", "d"]);
    }

    #[test]
    fn test_split_pipe_capture_star() {
        let regex = "a(b|c)*d|e".to_string();
        let tokens = split_to_parts(regex);
        assert_eq!(tokens, vec!["a", "(b|c)*", "d", "|", "e"]);
    }

    #[test]
    fn test_split_stars() {
        let regex = r"a*b+".to_string();
        let tokens = split_to_parts(regex);
        assert_eq!(tokens, vec![r"a*", r"b+"]);
    }

    #[test]
    fn test_split4() {
        let regex = r"a(b|c)*?".to_string();
        let tokens = split_to_parts(regex);
        assert_eq!(tokens, vec![r"a", r"(b|c)*?"]);
    }

    fn test_split_question() {
        let regex = r"a?b".to_string();
        let tokens = split_to_parts(regex);
        assert_eq!(tokens, vec![r"a?", r"b"]);
    }

    fn test_split_backslash() {
        let regex = r"a\\b".to_string();
        let tokens = split_to_parts(regex);
        assert_eq!(tokens, vec![r"a",r"\w"]);
    }

    fn test_split_backslash_quantifier() {
        let regex = r"a\*b".to_string();
        let tokens = split_to_parts(regex);
        assert_eq!(tokens, vec![r"a",r"\*"]);
    }

    fn test_split_backslash_parentheses() {
        let regex = r"a\(b".to_string();
        let tokens = split_to_parts(regex);
        assert_eq!(tokens, vec![r"a",r"\(","b"]);
    }

    fn test_split_backslash_backslash() {
        let regex = r"a\\b".to_string();
        let tokens = split_to_parts(regex);
        assert_eq!(tokens, vec![r"a",r"\\","b"]);
    }
}
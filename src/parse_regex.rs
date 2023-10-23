use std::vec::Vec;
use std::collections::HashSet;
use std::collections::HashMap;

enum TokenType {
    CaptureGroup,
    CharacterClass,
    Pipe,
    Normal,
}
pub struct Token {
    token: String,
    token_type: TokenType,
    quantifier: String,
    sub_groups: Vec<Token>,
}
pub fn process_regex(regex: &str) -> (Vec<Token>, HashSet<char>) {
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
    let chars: Vec<char> = regex.chars().collect();
    let mut cur_index: usize = 0;

    while cur_index < chars.len() {
        let mut cur_token: String = String::new();

        if chars[cur_index] == '(' || chars[cur_index] == '[' {
            let end_index = find_paren_match(&regex, cur_index);
            cur_token.push_str(&regex[cur_index..end_index + 1]);
            cur_index = end_index + 1;
        } else {
            if chars[cur_index] == '\\' {
                cur_token.push(chars[cur_index]);
                cur_index += 1;
            }
            cur_token.push(chars[cur_index]);
            cur_index += 1;
        }
        // Getting the quantifiers
        if cur_index < chars.len() {
            if chars[cur_index] == '*' || chars[cur_index] == '+' {
                cur_token.push(chars[cur_index]);
                cur_index += 1;
            } else if chars[cur_index] == '{' {
                let end_index = regex[cur_index..].find('}').unwrap() + cur_index;
                cur_token.push_str(&regex[cur_index..end_index + 1]);
                cur_index = end_index + 1;
            }
            if cur_index < chars.len() && chars[cur_index] == '?' {
                cur_token.push(chars[cur_index]);
                cur_index += 1;
            }
        }
        parts.push(cur_token);
    }
    parts
}

fn parts_to_token(parts: Vec<String>) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    for part in parts {
        let mut cur_index: usize = 0;
        let mut cur_token: Token = Token {
            token: String::new(),
            token_type: TokenType::Normal,
            quantifier: String::new(),
            sub_groups: Vec::new(),
        };
        if part.as_bytes()[0] as char == '(' {
            let end_index = find_paren_match(&part, 0);
            let inner_component = part[1..end_index].to_string();
            cur_token.token_type = TokenType::CaptureGroup;
        } else if part.as_bytes()[0] as char == '[' {
            cur_token.token_type = TokenType::CharacterClass;
        } else {
            if part.as_bytes()[0] as char == '\\' {
                cur_token.token.push(part.as_bytes()[0] as char);
                cur_index += 1;
            }
            cur_token.token.push(part.as_bytes()[cur_index] as char);
            cur_index += 1;
        }
        cur_token.quantifier = part[cur_index..].to_string();
        tokens.push(cur_token);
    }
    todo!();
}

fn tokenize(regex: String) -> Vec<Token> {
    todo!();
}

fn find_paren_match(regex: &String, starting_index: usize) -> usize {
    let end_parentheses = match regex.as_bytes()[starting_index] as char {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        _ => panic!("Invalid capture group"),
    };
    let mut cur_depth = 0;

    let mut parentheses: String = String::new();

    for (idx, char) in regex.char_indices().skip(starting_index) {
        if char == '(' || char == '[' {
            cur_depth += 1;
            parentheses.push(char);
        } else if char == end_parentheses && cur_depth == 1 {
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
        } else if char == '|' && cur_depth <= 0 {
            return true;
        }
    }
    false
}

fn find_pipes(regex: &String) -> Vec<usize> {
    let mut cur_depth = 0;
    let mut parentheses: String = String::new();
    let mut pipes: Vec<usize> = Vec::new();
    for (idx, char) in regex.char_indices() {
        if char == '(' || char == '[' {
            cur_depth += 1;
            parentheses.push(char);
        } else if char == ')' && char == parentheses.as_bytes()[parentheses.len() - 1] as char {
            cur_depth -= 1;
            parentheses.pop();
        } else if char == ']' && char == parentheses.as_bytes()[parentheses.len() - 1] as char {
            cur_depth -= 1;
            parentheses.pop();
        } else if char == '|' && cur_depth <= 0 {
            pipes.push(idx);
        }
    }
    pipes
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
        println!("{:?}", tokens);
        assert_eq!(tokens, vec![r"a", r"(b|c)*?"]);
    }

    #[test]
    fn test_split_question() {
        let regex = r"a?b".to_string();
        let tokens = split_to_parts(regex);
        assert_eq!(tokens, vec![r"a?", r"b"]);
    }

    #[test]
    fn test_split_backslash() {
        let regex = r"a\w".to_string();
        let tokens = split_to_parts(regex);
        assert_eq!(tokens, vec![r"a",r"\w"]);
    }

    #[test]
    fn test_split_backslash_quantifier() {
        let regex = r"a\*b".to_string();
        let tokens = split_to_parts(regex);
        assert_eq!(tokens, vec![r"a",r"\*", "b"]);
    }

    #[test]
    fn test_split_backslash_parentheses() {
        let regex = r"a\(b".to_string();
        let tokens = split_to_parts(regex);
        assert_eq!(tokens, vec![r"a",r"\(","b"]);
    }

    #[test]
    fn test_split_backslash_backslash() {
        let regex = r"a\\b".to_string();
        let tokens = split_to_parts(regex);
        assert_eq!(tokens, vec![r"a",r"\\","b"]);
    }

    #[test]
    fn test_split_quantifier() {
        let regex = r"a{1,2}b".to_string();
        let tokens = split_to_parts(regex);
        assert_eq!(tokens, vec![r"a{1,2}", "b"]);
    }

}
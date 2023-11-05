use std::vec::Vec;
use std::collections::HashSet;

#[derive(PartialEq, Debug)]
enum TokenType {
    CaptureGroup,
    CharacterClass,
    Pipe,
    Normal,
}

/*
    * Token is a struct that represents a single token in a regular expression.
    * A Normal token (not capture group) should have one character in its token field, and no sub_groups.
    * A Pipe token should have no characters in its token field, and at least one sub_group.
    * A Character class should have the character class in its class_options field, and no sub_groups.
    * A Capture group should have its contents split into tokens stored in the sub_groups field, and its quantifier in the quantifier field.
 */
#[derive(PartialEq, Debug)]
pub struct Token {
    pub(crate) token: String,
    token_type: TokenType,
    pub(crate) quantifier: String,
    sub_groups: Vec<Token>,
    class_options: HashSet<char>,
}



impl Token {
    fn blank() -> Self {
        Token {
            token: String::new(),
            token_type: TokenType::Normal,
            quantifier: String::new(),
            sub_groups: Vec::<Token>::new(),
            class_options: HashSet::<char>::new(),
        }
    }
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

fn split_pipes(regex: &String) -> Vec<String> {
    let mut cur_string = String::new();
    let mut parentheses: String = String::new();
    let mut options: Vec<String> = Vec::new();

    let chars = regex.chars();

    for char in chars {
        if char == '|' && parentheses.is_empty() {
            options.push(cur_string.clone());
            cur_string.clear();
        } else {
            if char == '(' || char == '[' {
                parentheses.push(char);
            } else if char == ')' && '(' == parentheses.as_bytes()[parentheses.len() - 1] as char {
                parentheses.pop();
            } else if char == ']' && '[' == parentheses.as_bytes()[parentheses.len() - 1] as char {
                parentheses.pop();
            }
            cur_string.push(char);
        }
    }
    options.push(cur_string);

    options
}
fn split_pipes_vec(parts: &Vec<String>) -> Vec<Vec<String>> {
    let mut options: Vec<Vec<String>> = Vec::new();
    let mut cur_vec: Vec<String> = Vec::new();
    for part in parts {
        if part == "|" {
            options.push(cur_vec.clone());
            cur_vec.clear();
        } else {
            cur_vec.push(part.clone());
        }
    }
    options.push(cur_vec);

    options
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
        let mut cur_token: Token = Token::blank();
        if part.as_bytes()[0] as char == '(' {
            let end_index = find_paren_match(&part, 0);
            let inner_component = part[1..end_index].to_string();
            cur_token.token_type = TokenType::CaptureGroup;
            cur_token.sub_groups = parts_to_token(split_to_parts(inner_component));
            cur_index = end_index + 1;
        } else if part.as_bytes()[0] as char == '[' {
            let end_index = find_paren_match(&part, 0);
            if part.as_bytes()[1] as char == '^' {
                cur_token.token.push(part.as_bytes()[1] as char);
                cur_index += 1;
            }
            let inner_component = part[cur_index + 1..end_index].to_string();
            cur_token.token_type = TokenType::CharacterClass;
            cur_token.class_options = process_character_class(inner_component);
            cur_index = end_index + 1;
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
    tokens
}

fn tokenize(regex: String) -> Vec<Token> {
    parts_to_token(split_to_parts(regex))
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

fn process_character_class(character_class: String) -> HashSet<char> {
    let chars: Vec<char> = character_class.chars().collect::<Vec<char>>();
    let length: usize = chars.len();
    let mut options = HashSet::<char>::new();

    for (idx, &char) in chars.iter().enumerate() {
        if length - idx > 3 && chars[idx + 1] == '-' {
            for c in char..=chars[idx + 2] {
                options.insert(c);
            }
            continue;
        }
        if char == '-' && idx != 0 && idx != length - 1 {
            options.insert(char);
        } else if char != ']' {
            options.push(char);
        }
    }

    options
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

    #[test]
    fn test_split_pipes_none() {
        let regex = r"a(b|c)d".to_string();
        let tokens = split_pipes(&regex);
        assert_eq!(tokens, vec!["a(b|c)d"]);
    }

    #[test]
    fn test_split_pipes_normal() {
        let regex = r"a(b|c)d|e".to_string();
        let tokens = split_pipes(&regex);
        assert_eq!(tokens, vec!["a(b|c)d", "e"]);
    }

    #[test]
    fn test_split_pipes_empty() {
        let regex = r"a(b|c)d||e".to_string();
        let tokens = split_pipes(&regex);
        assert_eq!(tokens, vec!["a(b|c)d", "", "e"]);
    }

    #[test]
    fn test_parts_to_token_uninteresting() {
        let regex: String = r"abc*de".to_string();
        let tokens: Vec<Token> = parts_to_token(split_to_parts(regex));
        let correct_tokens: Vec<Token> = vec![Token {token: "a".to_string(), ..Token::blank()},
                                              Token {token: "b".to_string(), ..Token::blank()},
                                              Token {token: "c".to_string(), quantifier: "*".to_string(), ..Token::blank()},
                                              Token {token: "d".to_string(), ..Token::blank()},
                                              Token {token: "e".to_string(), ..Token::blank()}];
        assert_eq!(tokens, correct_tokens);
    }

    #[test]
    fn test_parts_to_token_capture() {
        let regex: String = r"a(bc)*d".to_string();
        let tokens: Vec<Token> = parts_to_token(split_to_parts(regex));
        let correct_tokens: Vec<Token> = vec![
            Token {token: "a".to_string(), ..Token::blank()},
            Token {quantifier: "*".to_string(), token_type: TokenType::CaptureGroup, sub_groups: vec![Token {token: "b".to_string(), ..Token::blank()}, Token {token: "c".to_string(), ..Token::blank()}], ..Token::blank()},

            Token {token: "d".to_string(), ..Token::blank()}];
        assert_eq!(tokens, correct_tokens);
    }

    #[test]
    fn test_parts_to_token_class() {
        let regex: String = r"\w[bc]d".to_string();
        let tokens: Vec<Token> = parts_to_token(split_to_parts(regex));
        let correct_tokens: Vec<Token> = vec![
            Token {token: r"\w".to_string(), ..Token::blank()},
            Token {class_options: HashSet::<String>::from(["b".to_string(), "c".to_string()]), token_type: TokenType::CharacterClass, ..Token::blank()},
            Token {token: r"a".to_string(), ..Token::blank()}
        ];
        assert_eq!(tokens, correct_tokens);
    }

}
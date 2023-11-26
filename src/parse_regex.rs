use std::vec::Vec;
use std::collections::HashSet;

#[derive(PartialEq, Debug, Default, Copy, Clone)]
pub enum TokenQuantifier {
    Star,
    Plus,
    Question,
    Range(RangeType),
    #[default]
    None,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum RangeType {
    Discrete(usize),
    UpperBound(usize),
    LowerBound(usize),
    Both(usize, usize),
}

impl TokenQuantifier {
    fn from_string(quantifier: String) -> Self {
        if quantifier.len() == 0 {
            return TokenQuantifier::None;
        }
        match quantifier.as_bytes()[0] as char {
            '*' => TokenQuantifier::Star,
            '+' => TokenQuantifier::Plus,
            '{' => TokenQuantifier::range_from_string(quantifier),
            '?' => TokenQuantifier::Question,
            _ => TokenQuantifier::None,
        }
    }

    fn range_from_string(quantifier: String) -> Self {
        let end_index = find_paren_match(&quantifier, 0);
        let inner_component = quantifier[1..end_index].to_string();
        if inner_component.contains(',') {
            let bounds: Vec<&str> = inner_component.split(',').collect();
            return if bounds[1].len() == 0 {
                TokenQuantifier::Range(RangeType::LowerBound(str::parse::<usize>(bounds[0]).unwrap()))
            } else if bounds[0].len() == 0 {
                TokenQuantifier::Range(RangeType::UpperBound(str::parse::<usize>(bounds[1]).unwrap()))
            } else {
                TokenQuantifier::Range(RangeType::Both(str::parse::<usize>(bounds[0]).unwrap(), str::parse::<usize>(bounds[1]).unwrap()))
            };
        }
        TokenQuantifier::Range(RangeType::Discrete(str::parse::<usize>(inner_component.as_str()).unwrap()))
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Normal { token: char, quantifier: TokenQuantifier },
    CaptureGroup { sub_groups: Vec<Token>, quantifier: TokenQuantifier },
    CharacterClass { class_options: HashSet<char>, quantifier: TokenQuantifier, /* exclude: bool */},
    Pipe { sub_groups: Vec<Vec<Token>> },
}

impl Token {
    const WORD_CHAR: char = '🦀';
    const WILD_CARD: char = '🃏';
    const WHITE_SPACE: char = '🫥';
    const NUMBER: char = '💯';

    fn from(part: String) -> Self {
        // Capture Group
        return if part.as_bytes()[0] as char == '(' {
            let end_index = find_paren_match(&part, 0);
            let inner_component = part[1..end_index].to_string();

            let sub_groups: Vec<Token> = tokenize(inner_component);
            let quantifier = part[end_index + 1..].to_string();

            Token::CaptureGroup { sub_groups, quantifier: TokenQuantifier::from_string(quantifier) }
        }

        // Character Class
        else if part.as_bytes()[0] as char == '[' {
            // let exclude: bool = part.as_bytes()[1] as char == '^';
            let end_index = find_paren_match(&part, 0);

            let inner_component = part[1 /* + (exclude as usize) */..end_index].to_string();
            let class_options = process_character_class(inner_component);

            let quantifier = part[end_index + 1..].to_string();

            Token::CharacterClass { class_options, quantifier: TokenQuantifier::from_string(quantifier) }
        }

        // Normal Token
        else {
            let mut cur_string = String::new();
            let mut cur_index: usize = 0;
            if part.as_bytes()[0] as char == '\\' {
                cur_string.push(part.as_bytes()[0] as char);
                cur_index += 1;
            }
            cur_string.push(part.as_bytes()[cur_index] as char);
            cur_index += 1;

            let cur_token = match cur_string.as_str() {
                r"\w" => Token::WORD_CHAR,
                r"\s" => Token::WHITE_SPACE,
                r"\d" => Token::NUMBER,
                "." => Token::WILD_CARD,
                r"\t" => '\t',
                r"\n" => '\n',
                r"\r" => '\r',
                _ => cur_string.as_bytes()[cur_string.len()-1] as char,
            };

            let quantifier = part[cur_index..].to_string();

            Token::Normal { token: cur_token, quantifier: TokenQuantifier::from_string(quantifier) }
        }
    }
}


pub fn process_regex(regex: &str) -> (Vec<Token>, HashSet<char>) {
    if regex.as_bytes()[0] as char != '/' {
        return (tokenize(regex.to_string()), HashSet::new());
    }
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
    if parts.contains(&"|".to_string()) {
        let split: Vec<Vec<String>> = split_pipes_vec(&parts);
        let sub_groups: Vec<Vec<Token>> = split.iter().map(|vec: &Vec<String>| parts_to_token(vec.to_owned())).collect();
        return vec![Token::Pipe { sub_groups }]
    }
    parts.iter().map(
        |x: &String | Token::from(x.clone())
    ).collect::<Vec<Token>>()
}

fn tokenize(regex: String) -> Vec<Token> {
    if check_pipe(&regex) {
        let options: Vec<String> = split_pipes(&regex);
        let split: Vec<Vec<String>> = options.iter().map(|option: &String| split_to_parts(option.to_owned())).collect();
        let sub_groups: Vec<Vec<Token>> = split.iter().map(|vec: &Vec<String>| parts_to_token(vec.to_owned())).collect();
        return vec![Token::Pipe { sub_groups }]
    }
    parts_to_token(split_to_parts(regex))
}

fn find_paren_match(regex: &String, starting_index: usize) -> usize {
    let end_parentheses = match regex.as_bytes()[starting_index] as char {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        _ => panic!("Invalid capture group"),
    };
    let mut parentheses: String = String::new();

    for (idx, char) in regex.char_indices().skip(starting_index) {
        if char == '(' || char == '[' || char == '{' {
            match char {
                '(' => parentheses.push(')'),
                '[' => parentheses.push(']'),
                '{' => parentheses.push('}'),
                _ => panic!("Invalid capture group"),
            }
        } else if char == end_parentheses && parentheses.len() == 1 {
            return idx;
        } else if char == ')' && char == parentheses.as_bytes()[parentheses.len() - 1] as char {
            parentheses.pop();
        } else if char == ']' && char == parentheses.as_bytes()[parentheses.len() - 1] as char {
            parentheses.pop();
        }
    }
    panic!("Invalid capture group");
}

// Will check if the regex contains a pipe that is not in a capture group or character class
fn check_pipe(regex: &String) -> bool {
    let mut cur_depth = 0;
    let mut parentheses: String = String::new();
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
        } else if char == '|' && cur_depth <= 0 && idx > 0 && regex.as_bytes()[idx-1] as char != '\\' {
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
        } else if char == '|' && cur_depth <= 0 && idx > 0 && regex.as_bytes()[idx-1] as char != '\\' {
            pipes.push(idx);
        }
    }
    pipes
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

fn process_character_class(character_class: String) -> HashSet<char> {
    let chars: Vec<char> = character_class.chars().collect::<Vec<char>>();
    let length: usize = chars.len();
    let mut options = HashSet::<char>::new();

    for (idx, &char) in chars.iter().enumerate() {
        if length - idx >= 2 && chars[idx + 1] == '-' {
            options.extend(char..=chars[idx + 2]);
        }
        else if (char == '-' && idx != 0 && idx != length - 1) || (idx >= 2 && chars[idx - 1] == '-') {
            continue;
        } else if char != ']' {
            options.insert(char);
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
        let regex = r"a(b|c)*".to_string();
        let tokens = split_to_parts(regex);
        println!("{:?}", tokens);
        assert_eq!(tokens, vec![r"a", r"(b|c)*"]);
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
    fn test_split_pipes_vec() {
        let parts: Vec<String> = vec!["a(b|c)d".to_string(), "|".to_string(), "e".to_string(), "|".to_string()];
        let tokens = split_pipes_vec(&parts);
        assert_eq!(tokens, vec![vec!["a(b|c)d".to_string()], vec!["e".to_string()], vec![]]);
    }

    #[test]
    fn test_parts_to_token_uninteresting() {
        let regex: String = r"abc*de".to_string();
        let tokens: Vec<Token> = parts_to_token(split_to_parts(regex));
        let correct_tokens: Vec<Token> = vec![Token::Normal { token: 'a', quantifier: TokenQuantifier::None },
                                              Token::Normal { token: 'b', quantifier: TokenQuantifier::None },
                                              Token::Normal { token: 'c', quantifier: TokenQuantifier::Star },
                                              Token::Normal { token: 'd', quantifier: TokenQuantifier::None },
                                              Token::Normal { token: 'e', quantifier: TokenQuantifier::None }];
        assert_eq!(tokens, correct_tokens);
    }

    #[test]
    fn test_parts_to_token_capture() {
        let regex: String = r"a(bc)*d".to_string();
        let tokens: Vec<Token> = parts_to_token(split_to_parts(regex));
        let correct_tokens: Vec<Token> = vec![
            Token::Normal {token: 'a', quantifier: TokenQuantifier::None},
            Token::CaptureGroup { sub_groups: vec![Token::Normal { token: 'b', quantifier: TokenQuantifier::None },
                                                   Token::Normal { token: 'c', quantifier: TokenQuantifier::None }],
                                  quantifier: TokenQuantifier::Star},
            Token::Normal { token: 'd', quantifier: TokenQuantifier::None }];
        assert_eq!(tokens, correct_tokens);
    }

    #[test]
    fn test_parts_to_token_class() {
        let regex: String = r"\w[bc]d".to_string();
        let tokens: Vec<Token> = parts_to_token(split_to_parts(regex));
        let correct_tokens: Vec<Token> = vec![
            Token::Normal { token: Token::WORD_CHAR, quantifier: TokenQuantifier::None },
            Token::CharacterClass { class_options: HashSet::<char>::from(['b', 'c']), quantifier: TokenQuantifier::None, /* exclude: false */},
            Token::Normal { token: 'd', quantifier: TokenQuantifier::None }];
        assert_eq!(tokens, correct_tokens);
    }

    #[test]
    fn test_process_character_class() {
        let character_class: String = r"abc".to_string();
        let options: HashSet<char> = process_character_class(character_class);
        let correct_options: HashSet<char> = HashSet::from(['a', 'b', 'c']);
        assert_eq!(options, correct_options);
    }

    #[test]
    fn test_process_character_class_range() {
        let character_class: String = r"a-c".to_string();
        let options: HashSet<char> = process_character_class(character_class);
        let correct_options: HashSet<char> = HashSet::from(['a', 'b', 'c']);
        assert_eq!(options, correct_options);
    }

    #[test]
    fn test_parts_to_token_class_range() {
        // I'll be using a whale for \w
        let regex: String = r"\w[a-d]d".to_string();
        let tokens: Vec<Token> = parts_to_token(split_to_parts(regex));
        let correct_tokens: Vec<Token> = vec![
            Token::Normal { token: Token::WORD_CHAR, quantifier: TokenQuantifier::None },
            Token::CharacterClass { class_options: HashSet::<char>::from(['a', 'b', 'c', 'd']), quantifier: TokenQuantifier::None, /* exclude: false */ },
            Token::Normal { token: 'd', quantifier: TokenQuantifier::None }];
        assert_eq!(tokens, correct_tokens);
    }

    // #[test]
    #[allow(unused)]
    fn test_parts_to_token_class_exclude() {
        let parts: Vec<String> = vec!["[^ab]".to_string()];
        let tokens: Vec<Token> = parts_to_token(parts);
        let correct_tokens: Vec<Token> = vec![
            Token::CharacterClass { class_options: HashSet::<char>::from(['a', 'b']), quantifier: TokenQuantifier::None, /* exclude: true */}];
        assert_eq!(tokens, correct_tokens);
    }

    #[test]
    fn test_tokenize_nested() {
        let regex: String = r"a\w(c|[a-b]+)*d?".to_string();
        let tokens: Vec<Token> = tokenize(regex);
        let correct_tokens: Vec<Token> = vec![
            Token::Normal { token: 'a', quantifier: TokenQuantifier::None },
            Token::Normal { token: Token::WORD_CHAR, quantifier: TokenQuantifier::None },
            Token::CaptureGroup { sub_groups: vec![
                Token::Pipe { sub_groups: vec![
                    vec![Token::Normal { token: 'c', quantifier: TokenQuantifier::None }],
                    vec![Token::CharacterClass { class_options: HashSet::<char>::from(['a', 'b']), quantifier: TokenQuantifier::Plus, /* exclude: false */}]
                ] } ], quantifier: TokenQuantifier::Star},
            Token::Normal { token: 'd', quantifier: TokenQuantifier::Question }];
        assert_eq!(tokens, correct_tokens);
    }

    #[test]
    fn test_tokenize_pipe() {
        let regex: String = r"a(b|f)*d|e".to_string();
        let tokens: Vec<Token> = tokenize(regex);
        let correct_tokens: Vec<Token> = vec![
            Token::Pipe {
                sub_groups: vec![
                    vec![
                        Token::Normal { token: 'a', quantifier: TokenQuantifier::None },
                        Token::CaptureGroup {
                            sub_groups: vec![Token::Pipe {sub_groups: vec![
                                vec![Token::Normal { token: 'b', quantifier: TokenQuantifier::None }],
                                vec![Token::Normal { token: 'f', quantifier: TokenQuantifier::None }],
                                ]}], quantifier: TokenQuantifier::Star
                        },
                        Token::Normal { token: 'd', quantifier: TokenQuantifier::None }
                    ],
                    vec![
                        Token::Normal { token: 'e', quantifier: TokenQuantifier::None }
                    ]
                ]
            }
        ];

        assert_eq!(tokens, correct_tokens);
    }

}
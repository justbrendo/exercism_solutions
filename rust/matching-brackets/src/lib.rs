fn reverse_bracket(c: char) -> char {
    match c {
        '(' => ')',
        '{' => '}',
        '[' => ']',
        _ => c,
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for c in string.chars() {
        match c {
            '[' | '{' | '(' => stack.push(c), 
            ']' | '}' | ')' => {match stack.pop() {
                Some(p) => {if c != reverse_bracket(p) {return false}},
                None => {return false}
            }},
            _=> {}
        }
    }

    stack.is_empty()
}

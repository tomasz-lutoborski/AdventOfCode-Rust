use std::fs::read_to_string;

fn react(input: &str) -> usize {
    let mut stack: Vec<char> = Vec::new();
    for c in input.chars() {
        if let Some(&last) = stack.last() {
            if last != c && last.eq_ignore_ascii_case(&c) {
                stack.pop();
                continue;
            }
        }
        stack.push(c);
        println!("{:?}", stack);
    }
    stack.len()
}

fn react2(input: &str, ignore: char) -> usize {
    let mut stack: Vec<char> = Vec::new();
    for c in input.chars() {
        if c.eq_ignore_ascii_case(&ignore) {
            continue;
        }
        if let Some(&last) = stack.last() {
            if last != c && last.eq_ignore_ascii_case(&c) {
                stack.pop();
                continue;
            }
        }
        stack.push(c);
    }
    stack.len()
}

fn check_ignores(input: &str) {
    for c in b'a'..=b'z' {
        let c = c as char;
        let len = react2(input, c);
        println!("{}: {}", c, len);
    }
}

pub fn solve() {
    let content = "dabAcCaCBAcCcaDA";
    check_ignores(&content);
}

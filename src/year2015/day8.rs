use std::fs::read_to_string;

pub fn solve() {
    let content = read_to_string("inputs/Year2015/Day8.txt").unwrap();
    println!("Part 1: {}", &content);
}

#[cfg(test)]
mod test {
    use super::*;
    static INPUT: &str = r#"""
"abc"
"aaa\"aaa"
"\x27""#;

    // #[test]
}

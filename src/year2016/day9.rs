use std::fs::read_to_string;

fn decompress(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '(' {
            let mut marker = String::new();
            while let Some(c) = chars.next() {
                if c == ')' {
                    break;
                }
                marker.push(c);
            }
            let parts: Vec<usize> = marker.split('x').map(|s| s.parse().unwrap()).collect();
            let mut repeat = String::new();
            for _ in 0..parts[0] {
                repeat.push(chars.next().unwrap());
            }
            result.push_str(&repeat.repeat(parts[1]));
        } else {
            result.push(c);
        }
    }
    result
}

fn decompress_v2(s: &str) -> usize {
    let mut result = 0;
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '(' {
            let mut marker = String::new();
            while let Some(c) = chars.next() {
                if c == ')' {
                    break;
                }
                marker.push(c);
            }
            let parts: Vec<usize> = marker.split('x').map(|s| s.parse().unwrap()).collect();
            let mut repeat = String::new();
            for _ in 0..parts[0] {
                repeat.push(chars.next().unwrap());
            }
            result += parts[1] * decompress_v2(&repeat);
        } else {
            result += 1;
        }
    }
    result
}

pub fn solve() {
    let content = read_to_string("inputs/Year2016/Day9.txt").unwrap();
    println!("{}", decompress_v2(&content))
}

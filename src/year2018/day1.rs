use std::{collections::HashSet, fs::read_to_string};

fn find_frequencty(s: &str) -> i32 {
    s.lines()
        .map(|l| i32::from_str_radix(l, 10).unwrap())
        .fold(0, |acc, f| acc + f)
}

fn find_repeat(s: &str) -> i32 {
    let mut seen = HashSet::new();
    let mut curr = 0;

    for freq in s
        .lines()
        .map(|l| i32::from_str_radix(l, 10).unwrap())
        .cycle()
    {
        // println!("{}, {}, {:?}", curr, freq, seen);
        if let Some(&f) = seen.get(&(freq + curr)) {
            return f;
        } else {
            curr += freq;
            seen.insert(curr);
        }
    }

    curr
}

pub fn solve() {
    let content = read_to_string("inputs/Year2018/Day1.txt").unwrap();
    // let content = "+7\n+7\n-2\n-7\n-4";
    // let content = "+1\n-1";
    println!("{}", find_repeat(&content));
}

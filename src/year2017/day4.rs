use std::fs::read_to_string;

use itertools::Itertools;

fn is_anagram(s1: &str, s2: &str) -> bool {
    let mut chars1 = s1.chars().collect_vec();
    let mut chars2 = s2.chars().collect_vec();

    chars1.sort_unstable();
    chars2.sort_unstable();

    chars1 == chars2
}

fn count_non_repeating(s: &str) -> usize {
    let mut correct = 0;

    for line in s.lines() {
        let mut words = line.split_ascii_whitespace().collect_vec();
        words.sort_unstable();
        let mut sorted = words.clone();
        sorted.dedup();

        if sorted == words {
            correct += 1;
        };
    }

    correct
}

fn has_line_anagram(line: &str) -> bool {
    let words = line.split_ascii_whitespace().collect_vec();

    for (i, word1) in words.iter().enumerate() {
        for (j, word2) in words.iter().enumerate() {
            if i != j && is_anagram(&word1, &word2) {
                return true;
            }
        }
    }

    false
}

fn count_non_anagrams(s: &str) -> usize {
    let mut correct = 0;

    for line in s.lines() {
        if !has_line_anagram(line) {
            correct += 1;
        }
    }

    correct
}
pub fn solve() {
    let content = read_to_string("inputs/Year2017/Day4.txt").unwrap();
    println!("{}", count_non_anagrams(&content));
}

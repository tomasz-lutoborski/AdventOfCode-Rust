use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    iter::zip,
};

use itertools::Itertools;

fn has_three_vowels(input: &str) -> bool {
    input
        .chars()
        .filter(|c| match c {
            'a' | 'e' | 'o' | 'i' | 'u' => true,
            _ => false,
        })
        .count()
        >= 3
}

fn has_two_in_row(input: &str) -> bool {
    let zipped = zip(input.chars(), input.chars().skip(1));
    zipped.filter(|(c1, c2)| c1 == c2).count() > 0
}

fn has_not_forbidden(input: &str) -> bool {
    let forbidden = vec!["ab", "cd", "pq", "xy"];

    for pat in forbidden {
        if input.contains(pat) {
            return false;
        }
    }

    true
}

fn has_two_separated(input: &str) -> bool {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(3)
        .any(|w| w[0] == w[2])
}

fn has_pair_not_overlapping(input: &str) -> bool {
    let mut seen = HashSet::new();
    let char_pairs: Vec<_> = input
        .char_indices()
        .zip(input.char_indices().skip(1))
        .map(|((i, a), (_, b))| ((a, b), i))
        .collect();

    let mut seen_indices = HashMap::new();

    for ((a, b), i) in char_pairs {
        let pair = format!("{}{}", a, b);
        if let Some(&last_index) = seen_indices.get(&pair) {
            if i > last_index + 1 && seen.contains(&pair) {
                return true;
            }
        } else {
            seen.insert(pair.clone());
        }
        seen_indices.insert(pair, i);
    }

    false
}

pub fn solve() {
    let content = read_to_string("inputs/Year2015/Day5.txt").unwrap();
    let res = content
        .lines()
        .filter(|l| has_two_separated(l) && has_pair_not_overlapping(l))
        .count();
    println!("{:?}", res);
}

#[cfg(test)]
mod test {
    use crate::year2015::day5::{
        has_not_forbidden, has_pair_not_overlapping, has_three_vowels, has_two_in_row,
        has_two_separated,
    };

    #[test]
    fn test_has_three_vowels() {
        assert!(has_three_vowels("ugknbfddgicrmopn"));
        assert!(has_three_vowels("haegwjzuvuyypxyu"));
        assert!(!has_three_vowels("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_has_two_in_row() {
        assert!(has_two_in_row("ugknbfddgicrmopn"));
        assert!(has_two_in_row("haegwjzuvuyypxyu"));
        assert!(!has_two_in_row("jchzalrnumimnmhp"));
    }

    #[test]
    fn test_has_not_forbidden() {
        assert!(has_not_forbidden("ugknbfddgicrmopn"));
        assert!(!has_not_forbidden("haegwjzuvuyypxyu"));
        assert!(has_not_forbidden("jchzalrnumimnmhp"));
    }

    #[test]
    fn test_has_two_separated() {
        assert!(has_two_separated("qjhvhtzxzqqjkmpb"));
        assert!(has_two_separated("xxyxx"));
        assert!(!has_two_separated("uurcxstgmygtbstg"));
    }

    #[test]
    fn test_has_pair_not_overlapping() {
        assert!(has_pair_not_overlapping("qjhvhtzxzqqjkmpb"));
        assert!(has_pair_not_overlapping("xxyxx"));
        assert!(!has_pair_not_overlapping("ieodomkazucvgmuy"));
        assert!(!has_pair_not_overlapping("aaa"));
        assert!(has_pair_not_overlapping("uurcxstgmygtbstg"));
        assert!(has_pair_not_overlapping("xyxy"));
    }
}

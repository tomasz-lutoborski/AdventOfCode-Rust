use std::{collections::VecDeque, fs::read_to_string};

use itertools::Itertools;

fn count_two_three(s: &str) -> u32 {
    fn folding(acc: (u32, u32), el: Vec<String>) -> (u32, u32) {
        let twos = el.clone().into_iter().any(|s| s.len() == 2);
        let threes = el.into_iter().any(|s| s.len() == 3);

        match (twos, threes) {
            (true, true) => (acc.0 + 1, acc.1 + 1),
            (true, false) => (acc.0 + 1, acc.1),
            (false, true) => (acc.0, acc.1 + 1),
            _ => acc,
        }
    }
    let (twos, threes) = s
        .lines()
        .map(|l| {
            l.chars()
                .sorted()
                .group_by(|&x| x)
                .into_iter()
                .map(|(_, group)| group.collect::<String>())
                .collect_vec()
        })
        .fold((0, 0), |acc, el| folding(acc, el));

    twos * threes
}

fn differs_by_char(s1: &str, s2: &str) -> bool {
    s1.chars()
        .zip(s2.chars())
        .filter(|(c1, c2)| c1 != c2)
        // .inspect(|x| println!("{:?}", x))
        .count()
        == 1
}

fn find_similar(s: &str) -> String {
    let ids: Vec<&str> = s.lines().collect();

    for i in 0..ids.len() {
        for j in i + 1..ids.len() {
            // println!("{}\n{}", ids[i], ids[j]);
            if differs_by_char(ids[i], ids[j]) {
                return format!("{}\n{}", ids[i], ids[j]);
            }
        }
    }

    "failed".to_string()
}

pub fn solve() {
    let content = read_to_string("inputs/Year2018/Day2.txt").unwrap();
    // let content = "abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz";
    println!("{}", find_similar(&content));
}

#[cfg(test)]
mod test {
    use crate::year2018::day2::differs_by_char;

    #[test]
    fn test_differs_by_char() {
        assert!(differs_by_char("abcd", "abce"));
    }
}

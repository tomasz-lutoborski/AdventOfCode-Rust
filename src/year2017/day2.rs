use std::fs::read_to_string;

use itertools::Itertools;

fn get_row_min_max(s: &str) -> (u32, u32) {
    s.split_whitespace()
        .map(|s| u32::from_str_radix(&s, 10).unwrap())
        .minmax()
        .into_option()
        .unwrap()
}

fn find_divisible_pair<I>(iter: I) -> Option<(u32, u32)>
where
    I: IntoIterator<Item = u32>,
    I::IntoIter: Clone,
{
    iter.into_iter().tuple_combinations().find_map(|(a, b)| {
        if a % b == 0 {
            Some((a, b))
        } else if b % a == 0 {
            Some((b, a))
        } else {
            None
        }
    })
}

fn get_row_divisible(s: &str) -> u32 {
    let nums = s
        .split_whitespace()
        .map(|s| u32::from_str_radix(&s, 10).unwrap())
        .collect::<Vec<u32>>();

    find_divisible_pair(nums).map(|(a, b)| a / b).unwrap()
}

fn get_hash(s: &str) -> u32 {
    s.lines()
        .map(|l| get_row_min_max(l))
        .map(|(min, max)| max - min)
        .fold(0, |acc, diff| acc + diff)
}

fn get_hash_div(s: &str) -> u32 {
    s.lines()
        .map(|l| get_row_divisible(l))
        .fold(0, |acc, div| acc + div)
}

pub fn solve() {
    let content = read_to_string("inputs/Year2017/Day2.txt").unwrap();
    println!("{}", get_hash_div(&content));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_row_divisible() {
        assert_eq!(get_row_divisible("5 9 2 8"), 4);
    }

    #[test]
    fn test_find_divisible_pair() {
        assert_eq!(find_divisible_pair(vec![5, 9, 2, 8]), Some((8, 2)));
        assert_eq!(find_divisible_pair(vec![9, 4, 7, 3]), Some((9, 3)));
    }
}

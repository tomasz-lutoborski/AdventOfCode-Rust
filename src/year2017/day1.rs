use std::fs::read_to_string;

fn sum_matching(s: String) -> u32 {
    let (input, _): (Vec<_>, Vec<_>) = s
        .chars()
        .zip(s.chars().cycle().skip(1))
        .filter(|(c1, c2)| c1 == c2)
        .unzip();

    input
        .into_iter()
        .map(|c| c.to_digit(10).unwrap())
        .fold(0, |acc, x| x + acc)
}

fn sum_matching_halfway(s: String) -> u32 {
    let (input, _): (Vec<_>, Vec<_>) = s
        .chars()
        .zip(s.chars().cycle().skip(s.len() / 2))
        .filter(|(c1, c2)| c1 == c2)
        .unzip();

    input
        .into_iter()
        .map(|c| c.to_digit(10).unwrap())
        .fold(0, |acc, x| x + acc)
}

pub fn solve() {
    let content = read_to_string("inputs/Year2017/Day1.txt").unwrap();
    // println!("{}", content);
    println!("{}", sum_matching_halfway(content));
}

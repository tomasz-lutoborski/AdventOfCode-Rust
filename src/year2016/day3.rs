use std::{fs::read_to_string, str::FromStr};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
struct Triangle {
    sides: Vec<usize>,
}

impl Triangle {
    fn is_correct(&self) -> bool {
        self.sides[0] + self.sides[1] > self.sides[2]
    }
}

impl FromStr for Triangle {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sides = s
            .split_whitespace()
            .map(|side| side.parse().expect("Bad side"))
            .collect_vec();
        sides.sort();
        Ok(Triangle { sides })
    }
}

fn count_correct(s: &str) -> usize {
    s.lines()
        .map(|l| Triangle::from_str(l).unwrap())
        .filter(|t| t.is_correct())
        .count()
}

fn count_correct_horizontal(s: &str) -> usize {
    let mut transformed = String::new();

    s.lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
        .array_chunks::<3>()
        .for_each(|chunk| {
            transformed
                .push_str(format!("{} {} {}\n", chunk[0][0], chunk[1][0], chunk[2][0]).as_str());
            transformed
                .push_str(format!("{} {} {}\n", chunk[0][1], chunk[1][1], chunk[2][1]).as_str());
            transformed
                .push_str(format!("{} {} {}\n", chunk[0][2], chunk[1][2], chunk[2][2]).as_str());
        });

    count_correct(transformed.as_str())
}

pub fn solve() {
    let content = read_to_string("inputs/Year2016/Day3.txt").unwrap();
    println!("{:?}", count_correct_horizontal(content.as_str()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_triangle() {
        assert_eq!(
            Triangle::from_str("  785  516  744").unwrap(),
            Triangle {
                sides: vec![785, 516, 744]
            }
        );
    }

    #[test]
    fn test_triangle_is_correct() {
        assert!(Triangle::from_str("  785  516  744").unwrap().is_correct());
        assert!(!Triangle::from_str("  85  56  744").unwrap().is_correct());
        assert!(Triangle::from_str("  700  500  201").unwrap().is_correct());
    }
}

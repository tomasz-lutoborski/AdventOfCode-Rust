use std::{fs::read_to_string, str::FromStr};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Room {
    name: Vec<String>,
    id: usize,
    checksum: String,
}

fn group_by_equality<T: Eq + Clone + Ord>(mut vec: Vec<T>) -> Vec<Vec<T>> {
    vec.sort_unstable();
    let mut current_group: Vec<T> = Vec::new();
    let mut result: Vec<Vec<T>> = Vec::new();

    for item in vec {
        if current_group.is_empty() || current_group.last().unwrap() == &item {
            current_group.push(item);
        } else {
            result.push(current_group);
            current_group = vec![item];
        }
    }

    result
}

impl Room {
    fn is_correct(&self) -> bool {
        let letters: Vec<char> = self
            .name
            .clone()
            .into_iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect_vec()
            .concat();

        let mut grouped = group_by_equality(letters);

        grouped.sort_unstable_by(|a, b| b.len().cmp(&a.len()));

        grouped
            .into_iter()
            .take(5)
            .map(|cs| cs[0])
            .collect::<Vec<char>>()
            == self.checksum.chars().collect_vec()
    }

    fn decrypt_name(&self) -> String {
        self.name
            .iter()
            .map(|n| decrypt(n, self.id))
            .collect_vec()
            .join(" ")
    }
}

impl FromStr for Room {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s_iter = s.split("[");

        let name_and_id = s_iter.next().unwrap();
        let checksum: String = s_iter
            .next()
            .ok_or("Invalid format")?
            .trim_end_matches("]")
            .to_string();

        let ni_iter = name_and_id.rsplitn(2, "-");
        let id_str = ni_iter.clone().next().ok_or("Invalid id format")?;
        let id: usize = id_str.parse().map_err(|_| "ID is not a valid number")?;
        let name: Vec<String> = name_and_id[..name_and_id.len() - id_str.len() - 1]
            .split('-')
            .map(|s| s.to_string())
            .collect();

        Ok(Self { name, id, checksum })
    }
}

fn decrypt(s: &str, n: usize) -> String {
    let alphabet = "abcdefghijklmnopqrstuvwxyz".chars().collect_vec();
    s.chars()
        .map(|c| alphabet[(alphabet.iter().position(|&l| l == c).unwrap() + n) % alphabet.len()])
        .collect()
}

fn get_ids_sum(input: &str) -> usize {
    input
        .lines()
        .map(|l| Room::from_str(l).unwrap())
        .filter(|r| r.is_correct())
        .fold(0, |acc, r| acc + r.id)
}

pub fn solve() {
    let content = read_to_string("inputs/Year2016/Day4.txt").unwrap();
    println!(
        "{:?}",
        content
            .lines()
            .map(|r| Room::from_str(r).unwrap())
            .map(|r| { r.decrypt_name() })
            .collect_vec()
    );
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn test_room_from_str() {
        assert_eq!(
            Room::from_str("hqcfqwydw-fbqijys-whqii-huiuqhsx-660[qhiwf]").unwrap(),
            Room {
                name: vec![
                    "hqcfqwydw".to_string(),
                    "fbqijys".to_string(),
                    "whqii".to_string(),
                    "huiuqhsx".to_string()
                ],
                id: 660,
                checksum: "qhiwf".to_string()
            }
        );
    }

    #[test]
    fn test_group_by_equality() {
        assert_eq!(
            group_by_equality("abcbbbcddbabasvvabbd".chars().collect_vec()),
            vec![
                vec!['a', 'a', 'a', 'a'],
                vec!['b', 'b', 'b', 'b', 'b', 'b', 'b', 'b'],
                vec!['c', 'c'],
                vec!['d', 'd', 'd'],
                vec!['s']
            ]
        )
    }

    #[test]
    fn test_room_is_correct() {
        let mut room = Room {
            name: vec![
                "hqcfqwydw".to_string(),
                "fbqijys".to_string(),
                "whqii".to_string(),
                "huiuqhsx".to_string(),
            ],
            id: 660,
            checksum: "qhiwf".to_string(),
        };
        assert!(room.is_correct());

        room = Room::from_str("not-a-real-room-404[oarel]").unwrap();
        assert!(room.is_correct());

        room = Room::from_str("totally-real-room-200[decoy]").unwrap();
        assert!(!room.is_correct());

        room = Room::from_str("a-b-c-d-e-f-g-h-987[abcde]").unwrap();
        assert!(room.is_correct());
    }

    #[test]
    fn test_decrypt() {
        assert_eq!(decrypt("qzmt", 343), "very".to_string());
        assert_eq!(decrypt("zixmtkozy", 343), "encrypted".to_string());
    }

    #[test]
    fn test_decrypt_name() {
        let room = Room::from_str("qzmt-zixmtkozy-ivhz-343[oarel]").unwrap();
        assert_eq!(room.decrypt_name(), "very encrypted name".to_string())
    }
}

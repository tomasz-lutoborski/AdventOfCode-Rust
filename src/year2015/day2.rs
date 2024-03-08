use std::{fs::read_to_string, num::ParseIntError, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
struct Present {
    length: i32,
    width: i32,
    height: i32,
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePresentError;

impl FromStr for Present {
    type Err = ParsePresentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('x').collect();
        let length = parts[0].parse::<i32>().map_err(|_| ParsePresentError)?;
        let width = parts[1].parse::<i32>().map_err(|_| ParsePresentError)?;
        let height = parts[2].parse::<i32>().map_err(|_| ParsePresentError)?;

        Ok(Present {
            length,
            width,
            height,
        })
    }
}

impl Present {
    fn get_total_area(&self) -> i32 {
        2 * (self.width * self.length + self.height * self.length + self.width + self.height)
    }

    fn get_smallest_area(&self) -> i32 {
        let mut sides = vec![self.length, self.width, self.height];

        sides.sort_unstable();

        sides.into_iter().take(2).fold(1, |acc, side| acc * side)
    }

    fn get_volume(&self) -> i32 {
        self.width * self.height * self.length
    }

    fn get_smallest_perimeter(&self) -> i32 {
        let mut sides = vec![self.length, self.width, self.height];

        sides.sort_unstable();

        sides
            .into_iter()
            .take(2)
            .fold(0, |acc, side| acc + side * 2)
    }
}

fn parse_presents(input: String) -> Vec<Present> {
    input
        .lines()
        .map(|l| Present::from_str(l).unwrap())
        .collect()
}

fn calculate_area(presents: Vec<Present>) -> i32 {
    presents.into_iter().fold(0, |acc, present| {
        acc + present.get_total_area() + present.get_smallest_area()
    })
}

fn calculate_ribbon(presents: Vec<Present>) -> i32 {
    presents.into_iter().fold(0, |acc, present| {
        acc + present.get_smallest_perimeter() + present.get_volume()
    })
}

pub fn solve() {
    let file_content = read_to_string("inputs/Year2015/Day2.txt").unwrap();
    let presents = parse_presents(file_content);
    println!("{:?}", calculate_ribbon(presents))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_present() {
        assert_eq!(
            Present::from_str("3x11x24").unwrap(),
            Present {
                length: 3,
                width: 11,
                height: 24
            }
        )
    }

    #[test]
    fn test_parse_multiple_presents() {
        let mut pres_vec = Vec::new();
        pres_vec.push(Present {
            length: 3,
            width: 11,
            height: 24,
        });
        pres_vec.push(Present {
            length: 13,
            width: 5,
            height: 19,
        });
        pres_vec.push(Present {
            length: 1,
            width: 9,
            height: 27,
        });
        assert_eq!(
            parse_presents("3x11x24\n13x5x19\n1x9x27".to_string()),
            pres_vec
        );
    }

    #[test]
    fn test_get_smallest_area() {
        let present = Present {
            length: 13,
            width: 5,
            height: 19,
        };
        assert_eq!(present.get_smallest_area(), 65);
    }

    #[test]
    fn test_get_smallest_perimeter() {
        let present = Present {
            length: 13,
            width: 5,
            height: 19,
        };
        assert_eq!(present.get_smallest_perimeter(), 36);
    }
}

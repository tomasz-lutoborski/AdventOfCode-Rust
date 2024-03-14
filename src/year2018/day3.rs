use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    str::FromStr,
};

use itertools::Itertools;

type Canvas = HashMap<(u16, u16), Vec<u16>>;

fn put_claims_on_canvas(s: &str) -> Canvas {
    let mut canvas = Canvas::new();

    for claim in s.lines().map(|l| Claim::from_str(l).unwrap()) {
        for cell in claim.covered_cells() {
            canvas
                .entry(cell)
                .and_modify(|ids| ids.push(claim.id))
                .or_insert(vec![claim.id]);
        }
    }

    canvas
}

fn num_of_covered_by_more_then_one(canvas: Canvas) -> usize {
    canvas.values().filter(|v| v.len() > 1).count()
}

fn get_ids_of_overlapping(canvas: Canvas) -> Vec<u16> {
    let mut ids = canvas
        .values()
        .filter(|v| v.len() > 1)
        .flat_map(|v| v.clone())
        .collect_vec();

    ids.sort_unstable();
    ids.dedup();

    ids
}

#[derive(Debug, PartialEq)]
struct Claim {
    width: u16,
    height: u16,
    top_offset: u16,
    left_offset: u16,
    id: u16,
}

impl Claim {
    fn covered_cells(&self) -> Vec<(u16, u16)> {
        let mut cells = Vec::new();

        for i in self.left_offset..self.left_offset + self.width {
            for j in self.top_offset..self.top_offset + self.height {
                cells.push((i, j))
            }
        }

        cells
    }
}

impl FromStr for Claim {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s
            .split(&['#', '@', ':'])
            .skip(1)
            .map(|s| s.trim())
            .collect();

        let id: u16 = parts[0].parse().unwrap();

        let offsets: Vec<_> = parts[1]
            .split(',')
            .map(|s| s.parse::<u16>().unwrap())
            .collect();

        let size: Vec<_> = parts[2]
            .split('x')
            .map(|s| s.parse::<u16>().unwrap())
            .collect();

        Ok(Claim {
            width: size[0],
            height: size[1],
            top_offset: offsets[1],
            left_offset: offsets[0],
            id,
        })
    }
}

pub fn solve() {
    let content = read_to_string("inputs/Year2018/Day3.txt").unwrap();
    let all_ids: HashSet<u16> = content
        .lines()
        .map(|l| Claim::from_str(l).unwrap())
        .map(|claim| claim.id)
        .collect();
    let ids_overlapping =
        HashSet::from_iter(get_ids_of_overlapping(put_claims_on_canvas(&content)));

    println!("{:?}", all_ids.difference(&ids_overlapping));
}

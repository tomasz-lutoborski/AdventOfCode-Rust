use std::fs::read_to_string;

use itertools::Itertools;

type Maze = Vec<i32>;

fn escape_maze(s: &str) -> usize {
    let mut steps = 0;
    let mut index = 0;

    let mut maze: Maze = s
        .lines()
        .map(|l| i32::from_str_radix(l, 10).unwrap())
        .collect_vec();

    while let Some(&n) = maze.get(index) {
        let next_index = (index as i32 + n) as usize;

        if n >= 3 {
            maze[index] -= 1;
        } else {
            maze[index] += 1;
        };

        index = next_index;
        steps += 1;

        if index >= maze.len() {
            break;
        }
    }

    steps
}

pub fn solve() {
    let content = read_to_string("inputs/Year2017/Day5.txt").unwrap();
    // let content = "0\n3\n0\n1\n-3";
    println!("{}", escape_maze(&content));
}

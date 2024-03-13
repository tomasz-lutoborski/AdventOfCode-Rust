use std::{collections::HashMap, fs::read_to_string, os::unix::raw::gid_t};

fn find_distance_from_center(n: u32) -> u32 {
    let mut level: u32 = 1;

    while (level * 2 - 1).pow(2) < n {
        level += 1;
    }

    let level_diff = (level * 2 - 1).pow(2) - ((level - 1) * 2 - 1).pow(2);
    let step_to_corner = level_diff / 4;
    let dist_from_last_level = n - ((level - 1) * 2 - 1).pow(2);
    let mod_dist = dist_from_last_level % step_to_corner;
    let dist_from_corner = if mod_dist > step_to_corner / 2 {
        step_to_corner - mod_dist
    } else {
        mod_dist
    };

    (level - 1) + (step_to_corner / 2) - dist_from_corner
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Location(i32, i32);

impl Location {
    fn update(&self, dir: Direction) -> Self {
        match dir {
            Direction::Down => Location(self.0, self.1 - 1),
            Direction::Up => Location(self.0, self.1 + 1),
            Direction::Right => Location(self.0 + 1, self.1),
            Direction::Left => Location(self.0 - 1, self.1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn update(&self) -> Direction {
        match self {
            Self::Down => Self::Right,
            Self::Right => Self::Up,
            Self::Up => Self::Left,
            Self::Left => Self::Down,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Position {
    location: Location,
    direction: Direction,
}

impl Position {
    fn update(&self) -> Position {
        let new_dir = match self.location {
            Location(x, y)
                if x.abs() == y.abs() && !(x > 0 && y < 0)
                    || ((x > 0 && y < 0) && x.abs() == y.abs() + 1) =>
            {
                self.direction.update()
            }
            _ => self.direction,
        };

        Self {
            location: self.location.update(new_dir),
            direction: new_dir,
        }
    }

    fn get_adjacent_locs(&self) -> Vec<Location> {
        let mut adjacent_locs = Vec::new();
        let Location(x, y) = self.location;

        for i in x - 1..=x + 1 {
            for j in y - 1..=y + 1 {
                if (i, j) != (x, y) {
                    adjacent_locs.push(Location(i, j));
                }
            }
        }

        adjacent_locs
    }
}

#[derive(Debug)]
struct Grid {
    grid: HashMap<Location, u32>,
    position: Position,
    biggest: u32,
}

impl Grid {
    fn new() -> Self {
        let grid: HashMap<Location, u32> = HashMap::from([
            (Location(0, 0), 1),
            (Location(1, 0), 1),
            (Location(1, 1), 2),
            (Location(0, 1), 4),
            (Location(-1, 1), 5),
            (Location(-1, 0), 10),
            (Location(-1, -1), 11),
            (Location(0, -1), 23),
            (Location(1, -1), 25),
        ]);
        Self {
            grid,
            position: Position {
                location: Location(1, -1),
                direction: Direction::Right,
            },
            biggest: 25,
        }
    }

    fn sum_adjacent(&self) -> u32 {
        let adjs = self.position.get_adjacent_locs();

        adjs.iter()
            .fold(0, |acc, loc| acc + self.grid.get(loc).unwrap_or(&0))
    }

    fn generate_up_to(n: u32) -> Self {
        let mut grid = Self::new();
        while grid.biggest < n {
            grid.position = grid.position.update();
            let adj_sum = grid.sum_adjacent();
            grid.grid.insert(grid.position.location, adj_sum);
            grid.biggest = adj_sum;
        }

        grid
    }
}

pub fn solve() {
    let content = "347991";
    println!("{:#?}", Grid::generate_up_to(347992));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_update_direction() {
        assert_eq!(Direction::Right.update(), Direction::Up);
        assert_eq!(Direction::Left.update(), Direction::Down);
    }

    #[test]
    fn test_update_location() {
        assert_eq!(Location(1, 1).update(Direction::Right), Location(2, 1));
        assert_eq!(Location(1, 10).update(Direction::Down), Location(1, 9));
    }

    #[test]
    fn test_update_position() {
        assert_eq!(
            Position {
                location: Location(1, 1),
                direction: Direction::Up
            }
            .update(),
            Position {
                location: Location(0, 1),
                direction: Direction::Left
            }
        );
        assert_eq!(
            Position {
                location: Location(2, -2),
                direction: Direction::Right
            }
            .update(),
            Position {
                location: Location(3, -2),
                direction: Direction::Right
            }
        );
        assert_eq!(
            Position {
                location: Location(3, -2),
                direction: Direction::Right
            }
            .update(),
            Position {
                location: Location(3, -1),
                direction: Direction::Up
            }
        );
        assert_eq!(
            Position {
                location: Location(3, 3),
                direction: Direction::Up
            }
            .update(),
            Position {
                location: Location(2, 3),
                direction: Direction::Left
            }
        );
    }

    #[test]
    fn test_get_adjacent_locs() {
        assert_eq!(
            Position {
                location: Location(5, 5),
                direction: Direction::Left
            }
            .get_adjacent_locs(),
            vec![
                Location(4, 4),
                Location(4, 5),
                Location(4, 6),
                Location(5, 4),
                Location(5, 6),
                Location(6, 4),
                Location(6, 5),
                Location(6, 6)
            ]
        );
    }

    #[test]
    fn test_sum_adjacents() {
        let grid = Grid::new();
        assert_eq!(grid.sum_adjacent(), 25)
    }
}

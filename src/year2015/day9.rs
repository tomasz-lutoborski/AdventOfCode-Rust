use std::{collections::HashMap, fs::read_to_string, str::FromStr};

use itertools::Itertools;

type City = String;

#[derive(Debug)]
struct Route {
    from: City,
    to: City,
    distance: usize,
}

type RouteMap = HashMap<(City, City), usize>;

impl FromStr for Route {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" = ").collect();
        let cities: Vec<&str> = parts[0].split(" to ").collect();
        let from = cities[0].to_string();
        let to = cities[1].to_string();
        let distance = parts[1].parse().unwrap();
        Ok(Route { from, to, distance })
    }
}

fn build_route_map(s: &str) -> RouteMap {
    let routes = s
        .lines()
        .map(|line| line.parse::<Route>().unwrap())
        .collect_vec();
    let mut route_map = RouteMap::new();
    for route in routes {
        route_map.insert((route.from.clone(), route.to.clone()), route.distance);
        route_map.insert((route.to, route.from), route.distance);
    }
    route_map
}

fn find_shortest(route_map: &RouteMap) -> usize {
    let cities: Vec<City> = route_map
        .keys()
        .map(|(from, to)| vec![from.clone(), to.clone()])
        .flatten()
        .unique()
        .collect();
    let mut shortest = usize::MAX;
    for permutation in cities.iter().permutations(cities.len()) {
        let mut distance = 0;
        for i in 0..permutation.len() - 1 {
            distance += route_map[&(permutation[i].clone(), permutation[i + 1].clone())];
        }
        if distance < shortest {
            shortest = distance;
        }
    }
    shortest
}

fn find_longest(route_map: &RouteMap) -> usize {
    let cities: Vec<City> = route_map
        .keys()
        .map(|(from, to)| vec![from.clone(), to.clone()])
        .flatten()
        .unique()
        .collect();
    let mut longest = usize::MIN;
    for permutation in cities.iter().permutations(cities.len()) {
        let mut distance = 0;
        for i in 0..permutation.len() - 1 {
            distance += route_map[&(permutation[i].clone(), permutation[i + 1].clone())];
        }
        if distance > longest {
            longest = distance;
        }
    }
    longest
}

pub fn solve() {
    let content = read_to_string("inputs/Year2015/Day9.txt").unwrap();
    println!("{:?}", find_longest(&build_route_map(&content)));
}

#[cfg(test)]
mod test {
    #[test]
    fn test_parse_route() {
        let input = "London to Dublin = 464";
        let route = input.parse::<super::Route>().unwrap();
        assert_eq!(route.from, "London");
        assert_eq!(route.to, "Dublin");
        assert_eq!(route.distance, 464);
    }
}

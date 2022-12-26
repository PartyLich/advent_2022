//! Solutions to 2020 day 16 problems
//! --- Day 16: Proboscidea Volcanium ---
use std::collections::{HashMap, HashSet, VecDeque};

use lazy_static::lazy_static;
use regex::Regex;

use crate::read_file;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Valve<'a> {
    rate: u32,
    neighbors: HashMap<&'a str, u32>,
}

fn parse_line(line: &str) -> (&str, Valve) {
    lazy_static! {
        static ref VALVE_RE: Regex =
            Regex::new(r#"Valve (?P<key>\w+) .* rate=(?P<rate>-?\d+); .* valves? (?P<valves>.*)$"#)
                .unwrap();
    }

    VALVE_RE
        .captures(line)
        .and_then(|captures| {
            let key = captures.name("key")?.as_str();
            let rate: u32 = captures.name("rate")?.as_str().parse().ok()?;
            let mut neighbors: HashMap<_, _> = captures
                .name("valves")?
                .as_str()
                .split(", ")
                .map(|key| (key, 1))
                .collect();
            neighbors.insert(key, 0);

            Some((key, Valve { rate, neighbors }))
        })
        .unwrap()
}

fn find_distances<'a>(mut map: HashMap<&'a str, Valve<'a>>) -> HashMap<&'a str, Valve<'a>> {
    let keys: Vec<_> = map.keys().cloned().collect();

    for key in keys {
        let mut queue = VecDeque::with_capacity(map.len());
        let mut visited = HashSet::with_capacity(map.len());

        queue.push_back(key);

        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            if !visited.insert(current) {
                continue;
            }

            let neighbors: Vec<_> = map
                .get(current)
                .unwrap()
                .neighbors
                .iter()
                .filter_map(|(&key, val)| if *val == 1 { Some(key) } else { None })
                .collect();
            for neighbor in neighbors {
                if visited.contains(neighbor) {
                    continue;
                }
                queue.push_back(neighbor);

                let distance = map
                    .get(key)
                    .map(|valve| {
                        let prev = valve.neighbors.get(neighbor).unwrap_or(&u32::MAX);
                        let distance = valve
                            .neighbors
                            .get(current)
                            .map(|d| d + 1)
                            .unwrap_or(u32::MAX);

                        distance.min(*prev)
                    })
                    .unwrap();

                map.entry(key).and_modify(|valve| {
                    valve.neighbors.insert(neighbor, distance);
                });
                map.entry(neighbor).and_modify(|valve| {
                    valve.neighbors.insert(key, distance);
                });
            }
        }
    }

    map
}

fn solve_one<'a>(map: &HashMap<&'a str, Valve<'a>>, start: &str, minutes: u32) -> u32 {
    // evaluate each target branch
    fn helper(
        map: &HashMap<&str, Valve>,
        mut minutes: u32,
        mut total: u32,
        mut remain: HashSet<&str>,
        start: &str,
        target: &str,
    ) -> u32 {
        let rate = map.get(target).unwrap().rate;
        let distance = map.get(start).unwrap().neighbors.get(target).unwrap();

        if (distance + 1) > minutes {
            // node is unreachable within our time limit
            return total;
        }

        minutes -= distance + 1;
        total += minutes * rate;
        remain.remove(target);

        remain
            .iter()
            .map(|&next| helper(map, minutes, total, remain.clone(), target, next))
            .max()
            .unwrap_or(total)
    }

    let remain: HashSet<_> = map
        .iter()
        .filter_map(|(&key, valve)| if valve.rate > 0 { Some(key) } else { None })
        .collect();

    let a = map.get(start).expect("Start value does not exist");
    let max = remain
        .iter()
        .filter_map(|&valve| {
            let rate = map.get(valve).unwrap().rate;
            let dist = a.neighbors.get(valve).unwrap();

            rate.checked_sub(*dist)
        })
        .max()
        .unwrap();

    remain
        .iter()
        .filter_map(|&valve| {
            let rate = map.get(valve).unwrap().rate;
            let dist = a.neighbors.get(valve).unwrap();

            rate.checked_sub(*dist)
                .and_then(|value| if value == max { Some(valve) } else { None })
        })
        .map(|target| helper(map, minutes, 0, remain.clone(), start, target))
        .max()
        .unwrap()
}

/// returns the max pressure releasable in 30 minutes.
pub fn one(file_path: &str) -> u32 {
    const MINUTES: u32 = 30;
    const START: &str = "AA";
    let input = read_file(file_path);
    let mut valves: HashMap<_, _> = input.lines().map(parse_line).collect();

    valves = find_distances(valves);
    solve_one(&valves, START, MINUTES)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return the max pressure releasable in 30 minutes";
        let expected = 1651;
        let actual = one("input/16-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}

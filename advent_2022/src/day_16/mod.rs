//! Solutions to 2020 day 16 problems
//! --- Day 16: Proboscidea Volcanium ---
use std::collections::{HashMap, HashSet, VecDeque};

use lazy_static::lazy_static;
use regex::Regex;

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

        queue.push_back((key, key));

        while !queue.is_empty() {
            let (current, parent) = queue.pop_front().unwrap();
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
                queue.push_back((neighbor, current));

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

/// returns the max pressure releasable in 30 minutes.
pub fn one(file_path: &str) -> usize {
    todo!();
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

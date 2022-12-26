//! Solutions to 2020 day 16 problems
//! --- Day 16: Proboscidea Volcanium ---
use std::collections::HashMap;

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

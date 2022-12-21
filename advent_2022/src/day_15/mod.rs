//! Solutions to 2020 day 15 problems
//! --- Day 15: Beacon Exclusion Zone ---
use std::collections::HashSet;

use lazy_static::lazy_static;
use regex::Regex;

use crate::{day_09::Direction, read_file};

fn parse_line(line: &str) -> (Direction, Direction) {
    lazy_static! {
        static ref DIRECTION_RE: Regex = Regex::new(r#"x=(?P<x>-?\d+), y=(?P<y>-?\d+)$"#).unwrap();
    }
    let (sensor_str, beacon_str) = line.split_once(':').unwrap();
    let sensor = DIRECTION_RE
        .captures(sensor_str)
        .and_then(|captures| {
            let x = captures.name("x")?.as_str().parse().ok()?;
            let y = captures.name("y")?.as_str().parse().ok()?;
            Some(Direction(x, y))
        })
        .unwrap();
    let beacon = DIRECTION_RE
        .captures(beacon_str)
        .and_then(|captures| {
            let x = captures.name("x")?.as_str().parse().ok()?;
            let y = captures.name("y")?.as_str().parse().ok()?;
            Some(Direction(x, y))
        })
        .unwrap();

    (sensor, beacon)
}

impl Direction {
    /// Returns the manhattan distance between two points
    pub fn manhattan(&self, other: &Direction) -> usize {
        other.0.abs_diff(self.0) + other.1.abs_diff(self.1)
    }
}

fn solve_one(readings: &[(Direction, Direction)], row: isize) -> usize {
    let mut map: HashSet<Direction> = HashSet::new();
    for (sensor, beacon) in readings {
        let distance = sensor.manhattan(beacon) as isize;

        if (sensor.1 + distance) < row || (sensor.1 - distance) > row {
            continue;
        }

        for y in (0..=distance).rev() {
            if (sensor.1 + y) != row && (sensor.1 - y) != row {
                continue;
            }

            let width = distance - y;
            for x in 0..=width {
                let bottom = sensor.1 + y;
                let top = sensor.1 - y;

                if top == row {
                    map.insert(Direction(sensor.0 - x, top));
                    map.insert(Direction(sensor.0 + x, top));
                }

                if bottom == row {
                    map.insert(Direction(sensor.0 - x, bottom));
                    map.insert(Direction(sensor.0 + x, bottom));
                }
            }
        }
    }
    for (sensor, beacon) in readings {
        map.remove(sensor);
        map.remove(beacon);
    }

    map.len()
}

/// returns the number of positions a beacon cannot be present in a given row
pub fn one(file_path: &str) -> usize {
    const ROW: isize = 2_000_000;
    let readings: Vec<_> = read_file(file_path).lines().map(parse_line).collect();

    solve_one(&readings, ROW)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return the number of positions a beacon cannot be present in a given row";
        let expected = 26;
        let readings: Vec<_> = read_file("input/15-t.txt")
            .lines()
            .map(parse_line)
            .collect();
        let actual = solve_one(&readings, 10);
        assert_eq!(actual, expected, "{}", msg);
    }
}

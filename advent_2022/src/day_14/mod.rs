//! Solutions to 2020 day 14 problems
//! --- Day 14: Regolith Reservoir ---
use std::collections::HashSet;

use crate::{day_09::Direction, read_file};

fn parse_line(line: &str) -> HashSet<Direction> {
    let points: Vec<_> = line
        .split(" -> ")
        .map(|pair| {
            pair.split_once(',')
                .map(|(x_str, y_str)| Direction(x_str.parse().unwrap(), y_str.parse().unwrap()))
                .unwrap()
        })
        .collect();
    let mut result = HashSet::new();

    for idx in 1..points.len() {
        let start = points[idx - 1];
        let end = points[idx];

        if start.0 == end.0 {
            // vertical
            let x = start.0;
            let min = start.1.min(end.1);
            let max = start.1.max(end.1);
            for y in min..=max {
                result.insert(Direction(x, y));
            }
        } else {
            // horizontal
            let y = start.1;
            let min = start.0.min(end.0);
            let max = start.0.max(end.0);
            for x in min..=max {
                result.insert(Direction(x, y));
            }
        }
    }

    result
}

fn step(mut map: HashSet<Direction>, max_y: isize) -> (HashSet<Direction>, isize) {
    const START: Direction = Direction(500, 0);
    let can_move = |position| {
        if !map.contains(&(position + Direction(0, 1))) {
            // down
            return Some(position + Direction(0, 1));
        }
        if !map.contains(&(position + Direction(-1, 1))) {
            // down left
            return Some(position + Direction(-1, 1));
        }
        if !map.contains(&(position + Direction(1, 1))) {
            // down right
            return Some(position + Direction(1, 1));
        }

        None
    };

    let mut current_pos = START;
    while let Some(new_pos) = can_move(current_pos) {
        current_pos = new_pos;
        if current_pos.1 >= max_y {
            return (map, current_pos.1);
        }
    }
    map.insert(current_pos);

    (map, current_pos.1)
}

/// returns the number of units of sand come to rest before sand starts flowing into the abyss below
pub fn one(file_path: &str) -> u32 {
    let mut map: HashSet<Direction> = read_file(file_path).lines().flat_map(parse_line).collect();
    let max_y = map.iter().fold(0, |acc, next| next.1.max(acc));
    let mut count = 0;

    let mut sand_height = 0;
    while sand_height < max_y {
        count += 1;
        (map, sand_height) = step(map, max_y);
    }

    count - 1
}

fn step_two(mut map: HashSet<Direction>, max_y: isize) -> (HashSet<Direction>, bool) {
    const START: Direction = Direction(500, 0);
    let can_move = |position: Direction| {
        let next = position + Direction(0, 1);
        if next.1 >= max_y {
            return None;
        }

        if !map.contains(&next) {
            // down
            return Some(next);
        }

        let next = position + Direction(-1, 1);
        if !map.contains(&next) {
            // down left
            return Some(next);
        }

        let next = position + Direction(1, 1);
        if !map.contains(&next) {
            // down right
            return Some(next);
        }

        None
    };

    let mut current_pos = START;
    while let Some(new_pos) = can_move(current_pos) {
        current_pos = new_pos;
    }
    let is_new = map.insert(current_pos);

    (map, is_new)
}

/// returns the number of units of sand that come to rest before one stops at 500, 0
pub fn two(file_path: &str) -> u32 {
    let mut map: HashSet<Direction> = read_file(file_path).lines().flat_map(parse_line).collect();
    let max_y = map.iter().fold(0, |acc, next| next.1.max(acc)) + 2;
    let mut count = 0;

    let mut flowing = true;
    while flowing {
        count += 1;
        (map, flowing) = step_two(map, max_y);
    }

    count - 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return the number of units of sand come to rest before sand starts flowing into the abyss below";
        let expected = 24;
        let actual = one("input/14-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn part_two() {
        let msg = "should return the number of units of sand that come to rest before one stops at 500, 0";
        let expected = 93;
        let actual = two("input/14-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}

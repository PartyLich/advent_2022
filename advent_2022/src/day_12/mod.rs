//! Solutions to 2020 day 12 problems
//! --- Day 12: Hill Climbing Algorithm ---
use std::collections::{HashMap, HashSet, VecDeque};

use crate::{day_03::LOWERCASE_OFFSET, day_09::Direction, read_file};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Height(u32);

fn parse_char(character: char) -> (Height, bool, bool) {
    let mut start = false;
    let mut end = false;
    let height = Height(match character {
        'S' => {
            start = true;
            ('a' as u32).checked_sub(LOWERCASE_OFFSET).unwrap()
        }
        'E' => {
            end = true;
            ('z' as u32).checked_sub(LOWERCASE_OFFSET).unwrap()
        }
        _ => (character as u32)
            .checked_sub(LOWERCASE_OFFSET)
            .unwrap_or_else(|| panic!("Invalid character in map data: '{}'", character)),
    });

    (height, start, end)
}

fn parse_map(input: &str) -> Result<(HashMap<Direction, Height>, Direction, Direction), String> {
    let mut result = HashMap::new();
    let mut start = Default::default();
    let mut end = Default::default();

    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let (height, is_start, is_end) = parse_char(ch);
            let direction = Direction(col as isize, row as isize);

            result.insert(direction, height);

            if is_start {
                start = direction;
            }
            if is_end {
                end = direction;
            }
        }
    }

    Ok((result, start, end))
}

const WEST: Direction = Direction(-1, 0);
const EAST: Direction = Direction(1, 0);
const NORTH: Direction = Direction(0, 1);
const SOUTH: Direction = Direction(0, -1);

fn get_neighbors(current: Direction) -> Vec<Direction> {
    vec![
        current + NORTH,
        current + SOUTH,
        current + EAST,
        current + WEST,
    ]
}

fn dijkstra(map: &HashMap<Direction, Height>, target: Direction) -> HashMap<Direction, u32> {
    let mut result: HashMap<Direction, u32> = Default::default();
    let mut visited: HashSet<Direction> = Default::default();
    let mut queue = VecDeque::new();

    queue.push_back(target);
    result.insert(target, 0);

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        if !visited.insert(current) {
            continue;
        }

        let distance = result.get(&current).unwrap() + 1;
        for neighbor in get_neighbors(current) {
            if map.get(&neighbor).is_none() || visited.contains(&neighbor) {
                continue;
            }

            // if the height makes it non-traversable, continue
            let current_height = map.get(&current).unwrap().0;
            let neighbor_height = map.get(&neighbor).unwrap().0;
            if current_height > neighbor_height && (current_height - neighbor_height) > 1 {
                continue;
            }

            result.insert(neighbor, distance);
            queue.push_back(neighbor);
        }
    }

    result
}

/// returns the shortest path length from start to end position
pub fn one(file_path: &str) -> u32 {
    let input = read_file(file_path);
    let (map, start, end) = parse_map(&input).unwrap();
    let distance_map = dijkstra(&map, end);

    *distance_map.get(&start).unwrap()
}

/// returns the shortest path length from any square with elevation a to the end
/// position
pub fn two(file_path: &str) -> u32 {
    let input = read_file(file_path);
    let (map, _, end) = parse_map(&input).unwrap();
    let distance_map = dijkstra(&map, end);

    *map.iter()
        .filter_map(|(position, &height)| {
            if height.0 == 1 {
                // some of the map points may be non-traversable
                Some(distance_map.get(position).unwrap_or(&u32::MAX))
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return shortest path length from start to end location";
        let expected = 31;
        let actual = one("input/12-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn part_two() {
        let msg = "should return shortest path from any valid start point";
        let expected = 29;
        let actual = two("input/12-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}

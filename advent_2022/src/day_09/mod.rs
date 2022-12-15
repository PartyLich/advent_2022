//! Solutions to 2020 day 9 problems
//! --- Day 9: Rope Bridge ---
use std::collections::HashSet;
use std::ops::{Add, Sub};

use parser::three::lib::{choice, keep_first, p_char, p_int};

use crate::read_file;

/// Grid direction
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Direction(pub isize, pub isize);

impl Add for Direction {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Direction {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

fn parse_line(input: &str) -> Result<(Direction, isize), String> {
    let w = p_char('L').map(|_| Direction(-1, 0));
    let e = p_char('R').map(|_| Direction(1, 0));
    let n = p_char('U').map(|_| Direction(0, 1));
    let s = p_char('D').map(|_| Direction(0, -1));
    let all = choice([n, e, s, w]);

    let space = p_char(' ');
    let digit = p_int(10);

    let parser = keep_first(all, space)
        .and_then(digit)
        .with_label("Motion list".to_string());

    match parser.parse(input) {
        Ok((_input, value)) => Ok(value),
        Err(err) => Err(format!("{}", err)),
    }
}

fn follow(head: Direction, tail: Direction) -> Direction {
    let dist = head - tail;
    if (dist.0).abs() < 2 && (dist.1).abs() < 2 {
        return tail;
    }

    let mut tail = tail;
    let x_step = dist.0.signum();
    let y_step = dist.1.signum();

    if (dist.1).abs() == 0 {
        // horizontal
        tail.0 += x_step;
    } else if (dist.0).abs() == 0 {
        // vertical
        tail.1 += y_step;
    } else {
        // diagonal
        tail.0 += x_step;
        tail.1 += y_step;
    }

    tail
}

/// returns the number of positions the tail visited at least once
pub fn one(file_path: &str) -> usize {
    let input = read_file(file_path);
    let instructions = input
        .lines()
        .map(parse_line)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let mut visited: HashSet<Direction> = Default::default();
    let mut head = Direction(0, 0);
    let mut tail = Direction(0, 0);

    visited.insert(tail);

    for (dir, count) in instructions {
        for _ in 0..count {
            head = head + dir;
            tail = follow(head, tail);

            visited.insert(tail);
        }
    }

    visited.len()
}

/// returns tail positions with a longer rope
pub fn two(file_path: &str) -> usize {
    let input = read_file(file_path);
    let instructions = input
        .lines()
        .map(parse_line)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let mut visited: HashSet<Direction> = Default::default();
    let mut head = Direction(0, 0);
    let mut tail = Vec::with_capacity(9);
    for _ in 0..9 {
        tail.push(Direction(0, 0));
    }

    visited.insert(tail[8]);

    for (dir, count) in instructions {
        for _ in 0..count {
            head = head + dir;
            tail = tail
                .iter()
                .enumerate()
                .fold(Vec::with_capacity(9), |mut acc, (idx, &next)| {
                    let lead = if idx == 0 { head } else { acc[idx - 1] };
                    let result = follow(lead, next);
                    acc.push(result);

                    acc
                });

            visited.insert(tail[8]);
        }
    }

    visited.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return the number of positions the tail visited at least once";
        let expected = 13;
        let actual = one("input/09-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn part_two() {
        let msg = "should return tail positions with a longer rope";
        let expected = 1;
        let actual = two("input/09-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn test_follow() {
        let msg = "should return next position according to rules";
        let expected = Direction(4, 1);
        let actual = follow(Direction(5, 1), Direction(3, 0));
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn part_two_larger() {
        let msg = "should return tail positions with a longer rope";
        let expected = 36;
        let actual = two("input/09-t2.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}

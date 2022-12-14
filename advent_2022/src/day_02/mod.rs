//! Solutions to 2022 day 2 problems
//! --- Day 2: Rock Paper Scissors ---
use crate::read_file;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Shape {
    Rock = 1,
    Paper,
    Scissor,
}

impl From<&str> for Shape {
    fn from(character: &str) -> Self {
        match character {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissor,
            _ => panic!("Invalid character in map data: '{}'", character),
        }
    }
}

fn parse_line(line: &str) -> Vec<Shape> {
    return line.split_whitespace().map(From::from).collect::<Vec<_>>();
}

fn score(pair: Vec<Shape>) -> u32 {
    //  score for the outcome of the round
    let outcome = match pair[1] as isize - pair[0] as isize {
        //  3 if the round was a draw
        0 => 3,
        //  0 if you lost,
        -1 | 2 => 0,
        //  6 if you won).
        _ => 6,
    };

    pair[1] as u32 + outcome
}

/// returns the total score according to the strategy guide
pub fn one(file_path: &str) -> u32 {
    return read_file(file_path)
        .lines()
        .map(parse_line)
        .map(score)
        .sum();
}

fn score_two(pair: Vec<Shape>) -> u32 {
    match pair[1] {
        // X (Rock) means you need to lose
        Shape::Rock => 1 + (((pair[0] as u32) + 1) % 3),
        // Y (Paper) means you need to draw
        Shape::Paper => 3 + pair[0] as u32,
        // Z (Scissors) means you need to win
        Shape::Scissor => {
            let val = 1 + ((pair[0] as u32) % 3);
            6 + val
        }
    }
}

/// returns the total score according to the strategy guide
pub fn two(file_path: &str) -> u32 {
    read_file(file_path)
        .lines()
        .map(parse_line)
        .map(score_two)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return the total score according to the strategy guide";
        let expected = 15;
        let actual = one("input/02-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn part_two() {
        let msg = "should return the total score according to the strategy guide";
        let expected = 12;
        let actual = two("input/02-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}

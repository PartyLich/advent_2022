//! Solutions to 2022 day 5 problems
//!
use crate::read_file;

type Towers = Vec<Vec<char>>;

#[derive(Clone, Copy, Debug)]
/// count, from, to
struct Instruction(usize, usize, usize);

fn parse_towers(input: &str) -> Towers {
    let iter = input.lines().rev();
    let tower_count: usize = iter
        .take(1)
        .map(|line| line.split_whitespace().last().unwrap())
        .map(|max| max.parse().unwrap())
        .last()
        .unwrap();

    let mut res: Towers = Vec::with_capacity(tower_count);
    for _ in 0..tower_count {
        res.push(Vec::new());
    }

    for line in input.lines().rev().skip(1) {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .for_each(|(idx, c)| {
                if c != ' ' {
                    res[idx].push(c);
                }
            });
    }

    res
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<_> = line
                .split_whitespace()
                .skip(1)
                .step_by(2)
                .map(|num_str| num_str.parse::<usize>().expect("Failed to parse number"))
                .collect();

            Instruction(parts[0], parts[1] - 1, parts[2] - 1)
        })
        .collect()
}

fn process(mut tower: Towers, instruction: Instruction) -> Towers {
    for _ in 1..=instruction.0 {
        if let Some(c) = tower[instruction.1].pop() {
            tower[instruction.2].push(c);
        }
    }

    tower
}

/// returns the crates on top of each stack
pub fn one(file_path: &str) -> String {
    let input = read_file(file_path);
    let (tower_str, instruction_str) = input
        .split_once("\n\n")
        .expect("Failed to parse input file.");
    let instructions: Vec<_> = parse_instructions(instruction_str);
    let mut towers = parse_towers(tower_str);

    for instruction in instructions {
        towers = process(towers, instruction);
    }

    towers
        .iter()
        .filter_map(|tower| tower.last())
        .collect::<String>()
        .trim()
        .into()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return the crates on top of each stack";
        let expected = "CMZ";
        let actual = one("input/05-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}

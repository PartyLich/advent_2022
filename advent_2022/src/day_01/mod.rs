//! Solutions to 2022 day 1 problems
//! --- Day 1: Calorie Counting ---
use crate::read_file;

fn parse_elf(elf: &str) -> usize {
    return elf
        .lines()
        .map(|line| line.parse::<usize>().unwrap_or_default())
        .sum();
}

/// returns the largest quantity of calories carried
pub fn one(file_path: &str) -> usize {
    return read_file(file_path)
        .split("\n\n")
        .map(parse_elf)
        .max()
        .unwrap();
}

/// returns the sum of the largest 3 calories carried
pub fn two(file_path: &str) -> usize {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return the largest quantity of calories carried";
        let expected = 24000;
        let actual = one("input/01-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn part_two() {
        let msg = "should return the sum of the largest 3 calories carried";
        let expected = 45000;
        let actual = two("input/01-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}

//! Solutions to 2022 day 3 problems
//! --- Day 3: Rucksack Reorganization ---
use crate::read_file;
use std::collections::HashSet;

// A given rucksack always has the same number of items in each of its two compartments, so the
// first half of the characters represent items in the first compartment, while the second half of
// the characters represent items in the second compartment.
type Rucksack = (Vec<char>, Vec<char>);

fn parse_line(line: &str) -> Rucksack {
    let half = line.len() / 2;

    (
        line.chars().take(half).collect(),
        line.chars().skip(half).collect(),
    )
}

/// Ascii offset for lowercase alph chars
pub const LOWERCASE_OFFSET: u32 = 96;
const UPPERCASE_OFFSET: u32 = 64;

fn item_priority(item: char) -> u32 {
    if item as u32 > 96 {
        item as u32 - LOWERCASE_OFFSET
    } else {
        item as u32 - UPPERCASE_OFFSET + 26
    }
}

fn find_common(ruck: Rucksack) -> u32 {
    let set: HashSet<_> = ruck.0.iter().collect();
    let common = *ruck.1.iter().find(|&c| set.contains(c)).unwrap();

    item_priority(common)
}

/// returns the sum of the priorities of item type that appears in both compartments of each
/// rucksack
pub fn one(file_path: &str) -> u32 {
    read_file(file_path)
        .lines()
        .map(parse_line)
        .map(find_common)
        .sum()
}

/// find the one item type that is common between all three rucksacks in each group.
fn find_badge(rucks: &[Vec<char>]) -> u32 {
    let set: HashSet<_> = rucks[0].iter().collect();
    let set: HashSet<_> = rucks[1].iter().filter(|&c| set.contains(c)).collect();
    let common = *rucks[2].iter().find(|&c| set.contains(c)).unwrap();

    item_priority(common)
}

/// returns the sum of the badge priorities
pub fn two(file_path: &str) -> u32 {
    read_file(file_path)
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .chunks(3)
        .map(find_badge)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return the the sum of the priorities of item type that appears in both compartments of each rucksack";
        let expected = 157;
        let actual = one("input/03-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn part_two() {
        let msg = "should return the sum of the badge priorities";
        let expected = 70;
        let actual = two("input/03-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}

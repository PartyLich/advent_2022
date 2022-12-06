//! Solutions to 2020 day 6 problems
//! --- Day 6: Tuning Trouble ---
use std::collections::HashMap;

use crate::read_file;

fn parse_line(line: &str) -> Vec<char> {
    line.chars().collect()
}

fn find_start(size: usize) -> impl Fn(Vec<char>) -> usize {
    move |signal: Vec<char>| {
        let mut map: HashMap<char, u32> =
            signal
                .iter()
                .take(size)
                .fold(HashMap::new(), |mut map, ch| {
                    let count = map.entry(*ch).or_insert(0);
                    *count += 1;

                    map
                });
        let offset = size - 1;
        let mut end = offset;

        while *map.values().max().unwrap_or(&0) != 1 {
            map.entry(signal[end - offset])
                .and_modify(|count| *count -= 1);

            end += 1;

            map.entry(signal[end])
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        end + 1
    }
}

fn solve(file_path: &str, size: usize) -> usize {
    read_file(file_path)
        .lines()
        .map(parse_line)
        .map(find_start(size))
        .sum()
}

/// returns the number of characters processed before the first start-of-packet marker
pub fn one(file_path: &str) -> usize {
    solve(file_path, 4)
}

/// returns the number of characters processed before the first start-of-message marker
pub fn two(file_path: &str) -> usize {
    solve(file_path, 14)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return the number of characters processed before the first start-of-packet marker";
        let cases = [
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];
        for (input, expected) in cases {
            let actual = find_start(4)(parse_line(input));
            assert_eq!(actual, expected, "{}", msg);
        }
    }

    #[test]
    fn part_two() {
        let msg = "should return the number of characters processed before the first start-of-message marker";
        let cases = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];
        for (input, expected) in cases {
            let actual = find_start(14)(parse_line(input));
            assert_eq!(actual, expected, "{}", msg);
        }
    }
}

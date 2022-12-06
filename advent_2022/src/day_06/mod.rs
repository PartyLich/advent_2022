//! Solutions to 2018 day 6 problems
//!
use std::collections::HashMap;

use crate::read_file;

/// 
fn parse_line(line: &str) -> Vec<char> {
    line.chars().collect()
}

fn find_start(signal: Vec<char>) -> usize {
    let mut map: HashMap<char, u32> = signal.iter().take(4).fold(HashMap::new(), |mut map, ch| {
        let count = map.entry(*ch).or_insert(0);
        *count += 1;

        map
    });
    let mut end = 3;

    while *map.values().max().unwrap_or(&0) != 1 {
        map.entry(signal[end - 3]).and_modify(|count| *count -= 1);

        end += 1;

        map.entry(signal[end])
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    end + 1
}

/// returns the number of characters processed before the first start-of-packet marker
pub fn one(file_path: &str) -> usize {
    read_file(file_path)
        .lines()
        .map(parse_line)
        .map(find_start)
        .sum()
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
            let actual = find_start(parse_line(input));
            assert_eq!(actual, expected, "{}", msg);
        }
    }
}

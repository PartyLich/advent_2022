//! Solutions to 2018 day 6 problems
//!

/// 
fn parse_line(line: &str) -> Vec<char> {
    todo!()
}

fn find_start(signal: Vec<char>) -> usize {
    todo!()
}

/// returns the number of characters processed before the first start-of-packet marker
pub fn one(file_path: &str) -> usize {
    todo!()
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

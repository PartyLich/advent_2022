//! Solutions to 2022 day 4 problems
//! --- Day 4: Camp Cleanup ---
use crate::read_file;

#[derive(Clone, Copy, Debug)]
struct Range(u32, u32);

fn parse_line(line: &str) -> Vec<Range> {
    line.split(',')
        .map(|elf| {
            let parts: Vec<_> = elf
                .split('-')
                .map(|num| num.parse().expect("Failed to parse input"))
                .collect();
            Range(parts[0], parts[1])
        })
        .collect()
}

/// returns true if r0 is fully contained by r1
fn is_contained(r0: Range, r1: Range) -> bool {
    r0.0 >= r1.0 && r0.1 <= r1.1
}

/// returns the number of ranges totally contained by their partner.
pub fn one(file_path: &str) -> usize {
    read_file(file_path)
        .lines()
        .map(parse_line)
        .filter_map(|ranges| {
            if is_contained(ranges[0], ranges[1]) || is_contained(ranges[1], ranges[0]) {
                Some(())
            } else {
                None
            }
        })
        .count()
}

/// returns true if r0 overlaps r1
fn is_overlapping(r0: Range, r1: Range) -> bool {
    r0.1 >= r1.0 && r0.1 <= r1.1
}

/// returns the number of ranges that overlap their partner.
pub fn two(file_path: &str) -> usize {
    read_file(file_path)
        .lines()
        .map(parse_line)
        .filter_map(|ranges| {
            if is_overlapping(ranges[0], ranges[1]) || is_overlapping(ranges[1], ranges[0]) {
                Some(())
            } else {
                None
            }
        })
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn contained() {
        let msg = "should return true if r0 is fully contained by r1";
        assert!(is_contained(Range(3, 7), Range(2, 8)), "{}", msg);
        assert!(is_contained(Range(6, 6), Range(4, 6)), "{}", msg);
        assert!(!is_contained(Range(4, 6), Range(6, 6)), "{}", msg);
        assert!(!is_contained(Range(1, 7), Range(4, 6)), "{}", msg);
    }

    #[test]
    fn part_one() {
        let msg = "should return the number of ranges totally contained by their partner";
        let expected = 2;
        let actual = one("input/04-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn overlap() {
        let msg = "should return true if r0 overlaps r1";
        assert!(is_overlapping(Range(5, 7), Range(7, 9)), "{}", msg);
        assert!(is_overlapping(Range(3, 7), Range(2, 8)), "{}", msg);
        assert!(is_overlapping(Range(6, 6), Range(4, 6)), "{}", msg);
        assert!(!is_overlapping(Range(2, 4), Range(5, 6)), "{}", msg);
    }

    #[test]
    fn part_two() {
        let msg = "should return the number of ranges that overlap their partner";
        let expected = 4;
        let actual = two("input/04-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}

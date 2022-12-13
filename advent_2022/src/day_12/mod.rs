//! Solutions to 2020 day 12 problems
//! --- Day 12: Hill Climbing Algorithm ---

/// returns the shortest path length from start to end position
pub fn one(file_path: &str) -> usize {
    todo!()
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
}

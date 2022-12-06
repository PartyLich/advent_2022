//! Solutions to 2020 day 7 problems
//! --- Day 7: No Space Left On Device ---

/// returns the sum of all directories that are of size <= 100000.
pub fn one(file_path: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return the sum of all directories that are of size <= 100000.";
        let expected = 95437;
        let actual = one("input/07-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}

//! Solutions to 2020 day 16 problems
//! --- Day 16: Proboscidea Volcanium ---

/// returns the max pressure releasable in 30 minutes.
pub fn one(file_path: &str) -> usize {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return the max pressure releasable in 30 minutes";
        let expected = 1651;
        let actual = one("input/16-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}

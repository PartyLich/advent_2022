//! Solutions to 2020 day 9 problems
//! --- Day 9: Rope Bridge ---

/// returns the number of positions the tail visited at least once.
pub fn one(file_path: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return the number of positions the tail visited at least once";
        let expected = 13;
        let actual = one("input/09-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}

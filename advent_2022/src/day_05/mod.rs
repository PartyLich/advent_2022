//! Solutions to 2022 day 5 problems
//! --- Day 5: Supply Stacks ---

/// returns the crates on top of each stack
pub fn one(file_path: &str) -> String {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return the crates on top of each stack";
        let expected = "CMZ";
        let actual = one("input/05-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}

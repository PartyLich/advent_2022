//! Solutions to 2022 day 2 problems
//! --- Day 2: Rock Paper Scissors ---

/// returns
pub fn one(file_path: &str) -> usize {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return the total score according to the strategy guide";
        let expected = 15;
        let actual = one("input/02-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}

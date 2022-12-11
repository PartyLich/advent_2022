//! Solutions to 2020 day 11 problems
//! --- Day 11: Monkey in the Middle ---

/// returns the product of the two largest inspection counts after 20 rounds
pub fn one(file_path: &str) -> isize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return the product of the two largest inspection counts after 20 rounds";
        let expected = 10605;
        let actual = one("input/11-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}

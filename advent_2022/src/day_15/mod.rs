//! Solutions to 2020 day 15 problems
//! --- Day 15: Beacon Exclusion Zone ---


/// returns the number of positions a beacon cannot be present in a given row
pub fn one(file_path: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return the number of positions a beacon cannot be present in a given row";
        let expected = 26;
        let actual = one("input/15-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}

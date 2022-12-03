//! Solutions to 2022 day 4 problems
//! --- Day 4: Camp Cleanup ---


/// returns
pub fn one(file_path: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return the number of ranges totally contained by their partner";
        let expected = 2;
        let actual = one("input/04-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}

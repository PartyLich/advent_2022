//! Solutions to 2020 day 8 problems
//! --- Day 8: Treetop Tree House ---

/// returns the number of trees visible from outside the grid
pub fn one(file_path: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return the number of trees visible from outside the grid";
        let expected = 21;
        let actual = one("input/08-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}

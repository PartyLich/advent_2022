//! Solutions to 2020 day 14 problems
//! --- Day 14: Regolith Reservoir ---

/// returns the number of units of sand come to rest before sand starts flowing into the abyss below
pub fn one(file_path: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return the number of units of sand come to rest before sand starts flowing into the abyss below";
        let expected = 24;
        let actual = one("input/14-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}

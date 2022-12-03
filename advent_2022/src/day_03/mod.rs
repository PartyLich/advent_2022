//! Solutions to 2022 day 3 problems
//! --- Day 3: Rucksack Reorganization ---

/// returns the sum of the priorities of item type that appears in both compartments of each rucksack
pub fn one(file_path: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return the the sum of the priorities of item type that appears in both compartments of each rucksack";
        let expected = 157;
        let actual = one("input/03-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}

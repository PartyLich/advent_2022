//! Solutions to 2020 day 10 problems
//! --- Day 10: Cathode-Ray Tube ---

/// returns the sum of the signal strength during the 20th, 60th, 100th, 140th,
/// 180th, and 220th cycles
pub fn one(file_path: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return the sum of the signal strength during the 20th, 60th, 100th, 140th, 180th, and 220th cycles";
        let expected = 13140;
        let actual = one("input/10-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}

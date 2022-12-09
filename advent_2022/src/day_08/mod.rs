//! Solutions to 2020 day 8 problems
//! --- Day 8: Treetop Tree House ---
use crate::load_terrain;

#[derive(Clone, Copy, Debug)]
struct Height(u32);

impl From<char> for Height {
    fn from(character: char) -> Self {
        Self(
            character
                .to_digit(10)
                .unwrap_or_else(|| panic!("Invalid character in map data: '{}'", character)),
        )
    }
}

fn is_visible(map: &[Vec<Height>], map_row: usize) -> impl Fn((usize, &Height)) -> Option<()> + '_ {
    move |(map_col, &Height(height))| {
        // check up
        if map.iter().take(map_row).all(|row| row[map_col].0 < height) {
            return Some(());
        }

        // check down
        if map
            .iter()
            .skip(map_row + 1)
            .all(|row| row[map_col].0 < height)
        {
            return Some(());
        }

        let row = &map[map_row];
        // check right
        if row
            .iter()
            .skip(map_col + 1)
            .all(|&Height(tree)| tree < height)
        {
            return Some(());
        }

        // check left
        if row.iter().take(map_col).all(|&Height(tree)| tree < height) {
            return Some(());
        }

        None
    }
}

/// returns the number of trees visible from outside the grid
pub fn one(file_path: &str) -> u32 {
    let map = load_terrain::<Height>(file_path);
    let rows = map.len();
    let cols = map[0].len();
    let edge_count = 2 * (rows as u32 + cols as u32) - 4;

    let interior_count: usize = map
        .iter()
        .enumerate()
        .skip(1)
        .take(rows - 2)
        .map(|(map_row, col)| {
            col.iter()
                .enumerate()
                .skip(1)
                .take(cols - 2)
                .filter_map(is_visible(&map, map_row))
                .count()
        })
        .sum();

    edge_count + interior_count as u32
}

fn score(map: &[Vec<Height>], map_row: usize) -> impl Fn((usize, &Height)) -> usize + '_ {
    move |(map_col, Height(height))| {
        // check up
        let mut up = 0;
        for row in map.iter().take(map_row).rev() {
            up += 1;
            if row[map_col].0 >= *height {
                break;
            }
        }

        // check down
        let mut down = 0;
        for row in map.iter().skip(map_row + 1) {
            down += 1;
            if row[map_col].0 >= *height {
                break;
            }
        }

        let row = &map[map_row];
        // check right
        let mut right = 0;
        for Height(tree) in row.iter().skip(map_col + 1) {
            right += 1;
            if *tree >= *height {
                break;
            }
        }

        // check left
        let mut left = 0;
        for Height(tree) in row.iter().take(map_col).rev() {
            left += 1;
            if *tree >= *height {
                break;
            }
        }

        left * right * up * down
    }
}

/// returns the largest number of trees visible from a tree to the exterior in cardinal directions
pub fn two(file_path: &str) -> usize {
    let map = load_terrain::<Height>(file_path);
    let rows = map.len();
    let cols = map[0].len();

    map.iter()
        .enumerate()
        .skip(1)
        .take(rows - 2)
        .map(|(map_row, row)| {
            row.iter()
                .enumerate()
                .skip(1)
                .take(cols - 2)
                .map(score(&map, map_row))
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
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

    #[test]
    fn part_two() {
        let msg = "should return the largest number of trees visible from a tree to the exterior in cardinal directions";
        let expected = 8;
        let actual = two("input/08-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}

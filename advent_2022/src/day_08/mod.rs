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
                .filter_map(|(map_col, &height)| {
                    // check up
                    let mut max = 0;
                    for r in (0..(map_row)).rev() {
                        max = max.max(map[r][map_col].0);
                    }
                    if max < height.0 {
                        return Some(());
                    }

                    // check down
                    let mut max = 0;
                    for r in (map_row + 1)..rows {
                        max = max.max(map[r][map_col].0);
                    }
                    if max < height.0 {
                        return Some(());
                    }

                    // check right
                    let mut max = 0;
                    for c in (map_col + 1)..cols {
                        max = max.max(map[map_row][c].0);
                    }
                    if max < height.0 {
                        return Some(());
                    }

                    // check left
                    let mut max = 0;
                    for c in (0..(map_col)).rev() {
                        max = max.max(map[map_row][c].0);
                    }
                    if max < height.0 {
                        return Some(());
                    }

                    None
                })
                .count()
        })
        .sum();

    edge_count + interior_count as u32
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
                .map(|(map_col, Height(height))| {
                    // check up
                    let mut up = 0;
                    for r in (0..map_row).rev() {
                        up += 1;
                        if map[r][map_col].0 >= *height {
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

                    // check right
                    let mut right = 0;
                    for Height(tree) in row[(map_col + 1)..].iter() {
                        right += 1;
                        if *tree >= *height {
                            break;
                        }
                    }

                    // check left
                    let mut left = 0;
                    for Height(tree) in row[..(map_col)].iter().rev() {
                        left += 1;
                        if *tree >= *height {
                            break;
                        }
                    }

                    left * right * up * down
                })
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

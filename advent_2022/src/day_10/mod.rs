//! Solutions to 2020 day 10 problems
//! --- Day 10: Cathode-Ray Tube ---
use parser::three::lib::{choice, keep_first, p_char, p_int, p_string};

use crate::read_file;

fn parse_line(input: &str) -> Result<Option<isize>, String> {
    let space = p_char(' ');
    let digit = p_int(10);

    let addx = p_string("addx");
    let op_addx = keep_first(addx, space)
        .and_then(digit)
        .map(|(_, value)| Some(value));
    let noop = p_string("noop").map(|_| None);

    let parser = choice([op_addx, noop]).with_label("CPU instruction".to_string());

    match parser.parse(input) {
        Ok((_input, value)) => Ok(value),
        Err(err) => Err(format!("{}", err)),
    }
}

/// returns the sum of the signal strength during the 20th, 60th, 100th, 140th,
/// 180th, and 220th cycles
pub fn one(file_path: &str) -> isize {
    let ops: Vec<_> = read_file(file_path)
        .lines()
        .map(parse_line)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let mut reg_x = 1;
    let mut cycle = 1;
    let mut result: [isize; 221] = [0; 221];

    let exec = |op: &Option<isize>| {
        match op {
            Some(value) => {
                for _ in 0..2 {
                    if cycle < result.len() {
                        result[cycle - 1] = reg_x;
                    }
                    cycle += 1;
                }
                reg_x += value;
            }
            None => {
                if cycle < result.len() {
                    result[cycle - 1] = reg_x;
                }
                cycle += 1;
            }
        };
    };

    ops.iter().for_each(exec);

    result
        .iter()
        .enumerate()
        .skip(19)
        .step_by(40)
        .map(|(idx, value)| (idx + 1) as isize * value)
        .sum()
}

/// returns the rendered screen
pub fn two(file_path: &str) -> String {
    let ops: Vec<_> = read_file(file_path)
        .lines()
        .map(parse_line)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let mut reg_x = 1;
    let mut cycle = 1;
    let mut result: [isize; 240] = [0; 240];

    let exec = |op: &Option<isize>| {
        match op {
            Some(value) => {
                for _ in 0..2 {
                    if cycle < result.len() {
                        result[cycle - 1] = reg_x;
                    }
                    cycle += 1;
                }
                reg_x += value;
            }
            None => {
                if cycle < result.len() {
                    result[cycle - 1] = reg_x;
                }
                cycle += 1;
            }
        };
    };

    ops.iter().for_each(exec);

    const WIDTH: usize = 40;

    result
        .iter()
        .enumerate()
        .map(|(idx, &value)| {
            let col = (idx % WIDTH) as isize;
            if (value - 1..=value + 1).contains(&col) {
                '#'
            } else {
                '.'
            }
        })
        .collect::<Vec<_>>()
        .chunks(WIDTH)
        .map(|line| line.iter().collect())
        .collect::<Vec<String>>()
        .join("\n")
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

    #[test]
    fn part_two() {
        let msg = "should return rendered screen";
        let expected = r"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        let actual = two("input/10-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}

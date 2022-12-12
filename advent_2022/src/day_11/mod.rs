//! Solutions to 2020 day 11 problems
//! --- Day 11: Monkey in the Middle ---
use std::ops::{Add, Mul};
use std::str::FromStr;

use parser::three::lib::{
    between, choice, digit_char, keep_second, one_or_more, p_char, p_int, p_string, sep_by, spaces,
};

type Op = Box<dyn Fn(usize) -> usize>;

fn add(op_value: usize) -> Op {
    Box::new(move |i| op_value.add(i))
}

fn multiply(op_value: usize) -> Op {
    Box::new(move |i| op_value.mul(i))
}

struct Monkey {
    pub items: Vec<usize>,
    operation: Op,
    test: usize,
    target: (usize, usize),
}

impl Monkey {
    pub fn new(items: Vec<usize>, operation: Op, test: usize, target: (usize, usize)) -> Self {
        Self {
            items,
            operation,
            test,
            target,
        }
    }
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let digit = p_int(10).map(|i| i as usize);
        let digits = one_or_more(digit_char(10)).map::<String>(|s| s.into_iter().collect());
        let comma = p_char(',').and_then(spaces());

        let spaced = |s| between(spaces(), p_string(s), spaces());

        let int_list = sep_by(digit.clone(), comma.clone());
        let spaced_int = between(spaces(), digit.clone(), spaces());
        let mult = p_char('*').map::<fn(_) -> _>(|_| multiply);
        let plus = p_char('+').map::<fn(_) -> _>(|_| add);
        let ops = choice::<fn(_) -> _>([plus, mult])
            .and_then(between(
                spaces(),
                choice([p_string("old"), digits.clone()]),
                spaces(),
            ))
            .map(|(op, value)| {
                if let Ok(value) = value.parse::<_>() {
                    // if value is number
                    op(value)
                } else {
                    // otherwise it's "old"
                    Box::new(move |i| op(i)(i))
                }
            });

        let p_items = keep_second(spaced("Starting items:"), int_list);
        let p_op = keep_second(spaced("Operation: new = old "), ops.clone());

        let string_then_int = |s| keep_second(spaced(s), spaced_int.clone()).map(|i| i as usize);
        let p_test = string_then_int("Test: divisible by");
        let p_true = string_then_int("If true: throw to monkey");
        let p_false = string_then_int("If false: throw to monkey");

        let lines: Vec<_> = value.lines().collect();

        let items = p_items.parse(lines[1]).map_err(|err| format!("{}", err))?.1;
        let op_fn = p_op.parse(lines[2]).map_err(|err| format!("{}", err))?.1;
        let test = p_test.parse(lines[3]).map_err(|err| format!("{}", err))?.1;

        let on_true = p_true.parse(lines[4]).map_err(|err| format!("{}", err))?.1;
        let on_false = p_false.parse(lines[5]).map_err(|err| format!("{}", err))?.1;
        let target = (on_true, on_false);

        Ok(Monkey::new(items, op_fn, test, target))
    }
}


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

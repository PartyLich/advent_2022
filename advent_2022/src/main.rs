use std::{
    io::{self, Write},
    ops::RangeInclusive,
    path::PathBuf,
    time::{Duration, Instant},
};

use advent_2022::*;

#[cfg(debug_assertions)]
fn get_root_dir() -> PathBuf {
    env!("CARGO_MANIFEST_DIR").into()
}

#[cfg(not(debug_assertions))]
pub fn get_root_dir() -> PathBuf {
    if let Ok(mut exe_path) = std::env::current_exe() {
        exe_path.pop();
        exe_path
    } else {
        PathBuf::new()
    }
}

/// run a problem solver and return its output and run time
fn solve_day<T>(file: &'static str) -> impl Fn(Solver<T>) -> (T, Duration) {
    move |f| {
        let input_path = format!("{}/input/{}.txt", get_root_dir().display(), file);
        let start = Instant::now();
        let result = f(&input_path);
        let dur = start.elapsed();

        (result, dur)
    }
}

static SOLUTIONS: &[Option<Solution<String>>] = &[
    Some(to_solution!(
        "01-1",
        (day_01::one, "Calorie count"),
        (day_01::two, "Top 3 Calories")
    )),
    Some(to_solution!(
        "02-1",
        (day_02::one, "RPS"),
        (day_02::two, "RPS new rules")
    )),
    Some(to_solution!(
        "03-1",
        (day_03::one, "Common character sum"),
        (day_03::two, "Common char in 3 lists")
    )),
    Some(to_solution!(
        "04-1",
        (day_04::one, "Contained ranges"),
        (day_04::two, "Overlapping ranges")
    )),
    Some(to_solution!(
        "05-1",
        (day_05::one, "CrateMover 9000"),
        (day_05::two, "CrateMover 9001")
    )),
    Some(to_solution!(
        "06-1",
        (day_06::one, "signal window"),
        (day_06::two, "bigger signal window")
    )),
    Some(to_solution!(
        "07-1",
        (day_07::one, "recursive directory sizes"),
        (day_07::two, "free up space")
    )),
    Some(to_solution!(
        "08-1",
        (day_08::one, "tree visibility"),
        (day_08::two, "alternate tree visibility")
    )),
    Some(to_solution!(
        "09-1",
        (day_09::one, "unique tail positions"),
        (day_09::two, "long tail positions")
    )),
];

/// run a single day
// i disagree about this readability concern.
#[allow(clippy::option_map_unit_fn)]
fn run_day(day: usize) -> Result<(), String> {
    let idx = day.checked_sub(1).ok_or(format!("Invalid day {}", day))?;
    let entry = SOLUTIONS
        .get(idx)
        .ok_or_else(|| format!("Day {:02} solution not found.", day))?;

    entry
        .as_ref()
        .map(|solution| {
            println!("Day {:02}:", day);
            let run = solve_day(solution.input);
            solution.one.map(|(text, solver)| {
                let (result, dur) = run(solver);
                println!("\tPart 1 - {}: {} ({:?})", text, result, dur);
            });
            solution.two.map(|(text, solver)| {
                let (result, dur) = run(solver);
                println!("\tPart 2 - {}: {} ({:?})", text, result, dur);
            });
        })
        .ok_or_else(|| format!("Day {:02} solution not found.", day))
}

/// run every day in range
fn run_range(range: RangeInclusive<usize>) {
    for day in range {
        // run single day
        if let Err(msg) = run_day(day) {
            println!("{}", msg);
        }
    }
    println!();
}

/// clear the terminal
fn clear_screen() {
    const ESC: char = 0x1B as char;
    const SOFT_CLEAR: &str = "\x1B[2J\x1B[1;1H";
    print!("{}", SOFT_CLEAR);
}

fn main() {
    let mut input = String::new();

    clear_screen();
    loop {
        println!("Which day would you like to run?");
        println!("  a      for all days");
        println!("  #      or enter a day number (eg 17)");
        println!("  # - #  or enter a day range separated by a dash (eg 2-10)");
        println!("  q      to quit");
        print!("-> ");
        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        clear_screen();

        match input.parse() {
            Ok(Command::Quit) => {
                break;
            }
            Ok(Command::All) => {
                println!("Running all");
                run_range(1..=25);
            }
            Ok(Command::Range(range)) => {
                // run every day in range
                println!("Running days {:?}", range);
                run_range(range);
            }
            Ok(Command::Day(day)) => {
                // run single day
                println!("Running day {}", day);
                run_range(day..=day);
            }
            _ => {
                println!("Unrecognized command: '{}'", input.trim());
                println!("  len {}", input.len());
                println!("  buffer {}", input);
            }
        }
        input.clear();
    }
}

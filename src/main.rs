use adventofcode_24::{
    day01::solve_day01, day02::solve_day02, day03::solve_day03, day04::solve_day04,
    day05::solve_day05,
};
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("aoc")
        .version("1.0")
        .author("itsscb <dev@itsscb.de>")
        .about("Solves Advent of Code 2024 puzzles")
        .arg(
            Arg::new("day01")
                .short('1')
                .help("Path to Day 01 Input file"),
        )
        .arg(
            Arg::new("day02")
                .short('2')
                .help("Path to Day 02 Input file"),
        )
        .arg(
            Arg::new("day03")
                .short('3')
                .help("Path to Day 03 Input file"),
        )
        .arg(
            Arg::new("day04")
                .short('4')
                .help("Path to Day 04 Input file"),
        )
        .arg(
            Arg::new("day05")
                .short('5')
                .help("Path to Day 05 Input file"),
        )
        .get_matches();

    if let Some(file) = matches.get_one::<String>("day01") {
        match solve_day01(file) {
            Ok((d, s)) => println!("Result of Day 01:\nDistance: {d}\nScore: {s}"),
            Err(e) => eprintln!("{e}"),
        }
    }

    if let Some(file) = matches.get_one::<String>("day02") {
        match solve_day02(file) {
            Ok((r1, r2)) => {
                println!(
                    "Result of Day 02:\nSafe Reports: {r1}\nSafe Reports with tolerance: {r2}"
                );
            }
            Err(e) => eprintln!("{e}"),
        }
    }

    if let Some(file) = matches.get_one::<String>("day03") {
        match solve_day03(file) {
            Ok((r1, r2)) => {
                println!(
                    "Result of Day 03:\nSum of uncorrupted memory multiplications: {r1}\nSum of uncorrupted and enabled memory multiplications: {r2}"
                );
            }
            Err(e) => eprintln!("{e}"),
        }
    }

    if let Some(file) = matches.get_one::<String>("day04") {
        match solve_day04(file) {
            Ok((r1, r2)) => {
                println!("Result of Day 04:\nXMAS count: {r1}\nX-MAS count: {r2}");
            }
            Err(e) => eprintln!("{e}"),
        }
    }

    if let Some(file) = matches.get_one::<String>("day05") {
        match solve_day05(file) {
            Ok((r1, r2)) => {
                println!("Result of Day 05:\nOrdered middle page count: {r1}\nPart two: {r2}");
            }
            Err(e) => eprintln!("{e}"),
        }
    }
}

use adventofcode_24::{day01::solve_day01, day02::solve_day02};
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
}

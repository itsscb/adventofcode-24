use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::sequence::delimited;
use std::{error::Error, fs};

use regex::Regex;

/// Solves the problem for day 03.
///
/// # Errors
///
/// This function will return an error if the file cannot be read or if the input is invalid.
#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub fn solve_day03(path: &str) -> Result<(i32, i32), Box<dyn Error>> {
    let content = fs::read_to_string(path)?;

    let multiplications = extract_multiplications(&content);
    let part_one = multiplications.iter().map(|x| multiply(*x)).sum::<i32>();
    let multiplications = parse_multiplications_with_rules(&content);
    let part_two = multiplications.iter().map(|x| multiply(*x)).sum::<i32>();
    Ok((part_one, part_two))
}

fn parse_i32_pair(input: &str) -> IResult<&str, (i32, i32)> {
    let (input, first) = nom::character::complete::digit1(input)?;
    let (input, _) = char(',')(input)?;
    let (input, second) = nom::character::complete::digit1(input)?;
    Ok((input, (first.parse().unwrap(), second.parse().unwrap())))
}

fn parse_mul(input: &str) -> IResult<&str, (i32, i32)> {
    delimited(tag("mul("), parse_i32_pair, char(')'))(input)
}

fn ignore_until_mul(input: &str) -> (&str, &str) {
    let pos = input.find("mul(").unwrap_or(input.len());
    (&input[pos..], &input[..pos])
}

fn parse_mul_ignore_prefix(input: &str) -> IResult<&str, (i32, i32)> {
    let (input, _) = ignore_until_mul(input);
    parse_mul(input)
}

fn get_index_mul_and_stop(input: &str) -> (Option<usize>, Option<usize>) {
    let pos_mul = input.find("mul(");
    let pos_stop = input.find("don't()");
    (pos_mul, pos_stop)
}

fn get_index_start(input: &str) -> Option<usize> {
    input.find("do()")
}

fn parse_multiplications_with_rules(input: &str) -> Vec<(i32, i32)> {
    let mut results = Vec::new();
    let mut remaining = input;

    let mut parse = true;

    loop {
        if parse {
            let (pos_mul, pos_stop) = get_index_mul_and_stop(remaining);
            match (pos_mul, pos_stop) {
                (Some(pos_mul), Some(pos_stop)) => {
                    if pos_mul < pos_stop {
                        match parse_mul_ignore_prefix(remaining) {
                            Ok((next_input, result)) => {
                                results.push(result);
                                remaining = next_input;
                            }
                            Err(e) => match e {
                                nom::Err::Error(_) => {
                                    remaining = &remaining[1..];
                                    continue;
                                }
                                e => {
                                    dbg!("Error: {:?}", e);
                                    break;
                                }
                            },
                        }
                    } else {
                        parse = false;
                    }
                }
                (Some(_), None) => match parse_mul_ignore_prefix(remaining) {
                    Ok((next_input, result)) => {
                        results.push(result);
                        remaining = next_input;
                    }
                    Err(e) => match e {
                        nom::Err::Error(_) => {
                            remaining = &remaining[1..];
                            continue;
                        }
                        e => {
                            dbg!("Error: {:?}", e);
                            break;
                        }
                    },
                },
                (None, _) => {
                    break;
                }
            }
        } else {
            let pos_start = get_index_start(remaining);
            match pos_start {
                Some(pos_start) => {
                    remaining = &remaining[pos_start..];
                    parse = true;
                }
                None => {
                    break;
                }
            }
        }
    }

    results
}

#[allow(dead_code)]
fn parse_multiplications(input: &str) -> Vec<(i32, i32)> {
    let mut results = Vec::new();
    let mut remaining = input;

    loop {
        match parse_mul_ignore_prefix(remaining) {
            Ok((next_input, result)) => {
                results.push(result);
                remaining = next_input;
            }
            Err(e) => match e {
                nom::Err::Error(_) => {
                    remaining = &remaining[1..];
                    continue;
                }
                e => {
                    dbg!("Error: {:?}", e);
                    break;
                }
            },
        }

        // If no 'mul(' is found in the remaining input, break the loop
        if !remaining.contains("mul(") {
            break;
        }
    }

    results
}

#[allow(dead_code)]
fn extract_multiplications(input: &str) -> Vec<(i32, i32)> {
    let re = Regex::new("mul\\(\\d{1,3},\\d{1,3}\\)").unwrap();

    re.captures_iter(input)
        .filter_map(|cap| cap.get(0))
        .map(|cap| {
            // dbg!(&cap.as_str());
            let m = cap.as_str();
            let m = m.trim_start_matches("mul(");
            let m = m.trim_end_matches(')');
            let (a, b) = m.split_once(',').unwrap();
            let a: i32 = a.parse().unwrap();
            let b: i32 = b.parse().unwrap();
            (a, b)
        })
        .collect()
}

#[allow(dead_code)]
const fn multiply(input: (i32, i32)) -> i32 {
    input.0 * input.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day03_part_one() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result_regex = extract_multiplications(input);

        assert_eq!(4, result_regex.len());

        let want = 161;
        let got = result_regex.iter().map(|x| multiply(*x)).sum::<i32>();
        assert_eq!(want, got);

        let result = parse_multiplications(input);
        assert_eq!(4, result.len());
        assert_eq!(result_regex, result);
    }

    #[test]
    fn test_day03_part_two() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let want = 48;
        let result = parse_multiplications_with_rules(input);

        dbg!(&result);
        let got = result.iter().map(|x| multiply(*x)).sum::<i32>();

        assert_eq!(want, got);
    }
}

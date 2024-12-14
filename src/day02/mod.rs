use std::{error::Error, fs};

/// Solves the problem for day 02.
///
/// # Errors
///
/// This function will return an error if the file cannot be read or if the input is invalid.
#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub fn solve_day02(path: &str) -> Result<(i32, i32), Box<dyn Error>> {
    let content = fs::read_to_string(path)?;

    let data: Vec<Vec<i32>> = content
        .split_terminator('\n')
        .map(|line| {
            line.split_whitespace()
                .map(str::parse)
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    let result = verify_all_reports(&data);
    let result1 = result
        .iter()
        .filter(|safety| *safety != &Safety::Unsafe)
        .count() as i32;

    let result = verify_tolerance(&data);
    let result2 = result
        .iter()
        .filter(|safety| *safety != &Safety::Unsafe)
        .count() as i32;

    Ok((result1, result2))
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
enum Safety {
    Safe,
    Unsafe,
}

#[allow(dead_code)]
fn verify_all_reports(reports: &[Vec<i32>]) -> Vec<Safety> {
    reports
        .iter()
        .map(|report| verify_reports(report))
        .collect()
}

#[allow(dead_code)]
fn verify_tolerance(reports: &[Vec<i32>]) -> Vec<Safety> {
    reports
        .iter()
        // .filter(|report| verify_reports(report) == Safety::Unsafe)
        .map(|report| {
            let rep = verify_reports(report);

            if rep == Safety::Safe {
                return Safety::Safe;
            }

            if report
                .iter()
                .enumerate()
                .map(|(index, _)| {
                    let temp: Vec<i32> = report
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| *i != index)
                        .map(|(_, r)| *r)
                        .collect();
                    // dbg!(&report, index, &temp);
                    verify_reports(&temp)
                })
                .any(|r| r == Safety::Safe)
            {
                Safety::Safe
            } else {
                Safety::Unsafe
            }
        })
        .collect()
}

#[allow(dead_code)]
fn verify_reports(reports: &[i32]) -> Safety {
    const MAX: i32 = 3;
    const MIN: i32 = 1;

    for (index, report) in reports.iter().enumerate() {
        if index == 0 {
            continue;
        }

        let previous_report = reports[index - 1];
        if index == reports.len() - 1 {
            match (*report, previous_report) {
                (r, p) if r > p => {
                    let level = r - p;
                    if !(MIN..=MAX).contains(&level) {
                        return Safety::Unsafe;
                    }
                }
                (r, p) if r < p => {
                    let level = p - r;
                    if !(MIN..=MAX).contains(&level) {
                        return Safety::Unsafe;
                    }
                }
                (r, p) if r == p => return Safety::Unsafe,
                _ => (),
            }
            return Safety::Safe;
        }
        let next_report = reports[index + 1];

        match (*report, previous_report, next_report) {
            (r, p, n) if r > p && r < n => {
                let level = r - p;
                if !(MIN..=MAX).contains(&level) {
                    return Safety::Unsafe;
                }
            }
            (r, p, n) if r < p && r > n => {
                let level = p - r;
                if !(MIN..=MAX).contains(&level) {
                    return Safety::Unsafe;
                }
            }
            (r, p, n) if (r < p && r < n) || (r > p && r > n) => {
                return Safety::Unsafe;
            }
            (r, p, n) if r == p || r == n => return Safety::Unsafe,
            _ => (),
        }
    }

    Safety::Safe
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day02_part_one() {
        let input = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        let want = Safety::Safe;
        let got = verify_reports(&input[0]);

        assert_eq!(want, got);

        let want = Safety::Unsafe;

        let got = verify_reports(&input[1]);
        assert_eq!(want, got);
        let got = verify_reports(&input[2]);
        assert_eq!(want, got);
        let got = verify_reports(&input[3]);
        assert_eq!(want, got);
        let got = verify_reports(&input[4]);
        assert_eq!(want, got);

        let want = 2;
        let got = verify_all_reports(&input);
        assert_eq!(
            want,
            got.iter()
                .filter(|safety| *safety != &Safety::Unsafe)
                .count()
        );

        let got = verify_all_reports(&input);

        assert_eq!(
            want,
            got.iter()
                .filter(|safety| *safety != &Safety::Unsafe)
                .count()
        );
    }

    #[test]
    fn test_day02_part_two() {
        let input = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        let want = 4;

        let got = verify_tolerance(&input);
        assert_eq!(input.len(), got.len());

        assert_eq!(
            want,
            got.iter()
                .filter(|safety| *safety != &Safety::Unsafe)
                .count()
        );
    }
}

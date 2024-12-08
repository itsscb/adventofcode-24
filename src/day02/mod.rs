use std::{error::Error, fs};

/// Solves the problem for day 02.
///
/// # Errors
///
/// This function will return an error if the file cannot be read or if the input is invalid.
pub fn solve_day02(path: &str) -> Result<i32, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;

    let data: Vec<Vec<i32>> = content
        .split_terminator('\n')
        .map(|line| {
            line.split_whitespace()
                .map(str::parse)
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(0)
}

fn calculate_report_safety(data: &Vec<Vec<i32>>) -> i32 {
    data.iter()
        .map(|reports| {
            for report in reports {
                todo!("Implement this")
            }
            0
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day02() {
        let input = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        let want = 2;

        assert_eq!(calculate_report_safety(&input), want);
    }
}

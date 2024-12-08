use std::{error::Error, fs};

fn calculate_distance(array_one: &mut [i32], array_two: &mut [i32]) -> i32 {
    array_one.sort_unstable();
    array_two.sort_unstable();

    array_one
        .iter()
        .zip(array_two.iter())
        .map(|(a, b)| if a > b { a - b } else { b - a })
        .sum()
}

#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
fn calculate_score(array_one: &mut [i32], array_two: &mut [i32]) -> i32 {
    array_one.sort_unstable();
    array_two.sort_unstable();

    array_one
        .iter()
        .map(|i| i * (array_two.iter().filter(|&j| i == j).count() as i32))
        .sum()
}

/// Solves the problem for day 01.
///
/// # Errors
///
/// This function will return an error if the file cannot be read or if the input is invalid.
pub fn solve_day01(path: &str) -> Result<(i32, i32), Box<dyn Error>> {
    let content = fs::read_to_string(path)?;

    let mut array_one = Vec::new();
    let mut array_two = Vec::new();
    for line in content.split('\n') {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let a = parts[0];
            let b = parts[1];
            array_one.push(a.trim().parse()?);
            array_two.push(b.trim().parse()?);
        }
    }

    if array_one.is_empty() || array_one.len() != array_two.len() {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid input. Arrays must have a length > 0 and have an identical length",
        )))
    } else {
        let distance = calculate_distance(&mut array_one[..], &mut array_two[..]);
        let score = calculate_score(&mut array_one[..], &mut array_two[..]);
        Ok((distance, score))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day01() {
        let mut array_one = [3, 4, 2, 1, 3, 3];
        let mut array_two = [4, 3, 5, 3, 9, 3];

        let want = 11;

        assert_eq!(
            calculate_distance(&mut array_one[..], &mut array_two[..]),
            want
        );

        let want = 31;
        assert_eq!(
            calculate_score(&mut array_one[..], &mut array_two[..]),
            want
        );
    }
}

use std::{error::Error, fs};

/// Solves the problem for day 04.
///
/// # Errors
///
/// This function will return an error if the file cannot be read or if the input is invalid.
#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub fn solve_day04(path: &str) -> Result<(i32, i32), Box<dyn Error>> {
    let content = fs::read_to_string(path)?;

    let data = string_to_arrays(&content);

    let part_one = count_word(&data);
    let part_two = count_cross(&data);
    Ok((part_one, part_two))
}

#[allow(dead_code)]
fn string_to_arrays(input: &str) -> Vec<Vec<char>> {
    input
        .split_terminator('\n')
        .map(|x| x.chars().collect())
        .collect()
}

#[allow(dead_code)]
fn count_cross(input: &[Vec<char>]) -> i32 {
    const CROSS: [char; 3] = ['M', 'A', 'S'];
    let mut count = 0;

    for (li, line) in input.iter().enumerate() {
        for (ci, c) in line.iter().enumerate() {
            if c != &CROSS[1] {
                continue;
            }

            if ci > 0 && ci + 1 < line.len() && li > 0 && li + 1 < input.len() {
                let chars = [
                    input[li - 1][ci - 1],
                    input[li - 1][ci + 1],
                    input[li + 1][ci + 1],
                    input[li + 1][ci - 1],
                ];

                if chars == ['M', 'M', 'S', 'S']
                    || chars == ['M', 'S', 'S', 'M']
                    || chars == ['S', 'S', 'M', 'M']
                    || chars == ['S', 'M', 'M', 'S']
                {
                    count += 1;
                }
            }
        }
    }

    count
}

#[allow(dead_code, clippy::too_many_lines)]
fn count_word(input: &[Vec<char>]) -> i32 {
    const WORD: [char; 4] = ['X', 'M', 'A', 'S'];
    let mut count = 0;

    for (li, line) in input.iter().enumerate() {
        for (ci, c) in line.iter().enumerate() {
            if c != &WORD[0] {
                continue;
            }

            if ci >= 3 {
                if line[ci - 1] == WORD[1] && line[ci - 2] == WORD[2] && line[ci - 3] == WORD[3] {
                    count += 1;
                }
                if li >= 3
                    && input[li - 1][ci - 1] == WORD[1]
                    && input[li - 2][ci - 2] == WORD[2]
                    && input[li - 3][ci - 3] == WORD[3]
                {
                    count += 1;
                }
                if li + 3 < input.len()
                    && input[li + 1][ci - 1] == WORD[1]
                    && input[li + 2][ci - 2] == WORD[2]
                    && input[li + 3][ci - 3] == WORD[3]
                {
                    count += 1;
                }
            }
            if ci + 3 < line.len() {
                if line[ci + 1] == WORD[1] && line[ci + 2] == WORD[2] && line[ci + 3] == WORD[3] {
                    count += 1;
                }
                if li >= 3
                    && input[li - 1][ci + 1] == WORD[1]
                    && input[li - 2][ci + 2] == WORD[2]
                    && input[li - 3][ci + 3] == WORD[3]
                {
                    count += 1;
                }
                if li + 3 < input.len()
                    && input[li + 1][ci + 1] == WORD[1]
                    && input[li + 2][ci + 2] == WORD[2]
                    && input[li + 3][ci + 3] == WORD[3]
                {
                    count += 1;
                }
            }
            if li >= 3
                && input[li - 1][ci] == WORD[1]
                && input[li - 2][ci] == WORD[2]
                && input[li - 3][ci] == WORD[3]
            {
                count += 1;
            }
            if li + 3 < input.len()
                && input[li + 1][ci] == WORD[1]
                && input[li + 2][ci] == WORD[2]
                && input[li + 3][ci] == WORD[3]
            {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day04_part_one() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let data = string_to_arrays(input);
        assert_eq!(10, data.len());
        assert_eq!(10, data[0].len());

        let want = 18;
        let got = count_word(&data);

        assert_eq!(want, got);
    }

    #[test]
    fn test_day04_part_two() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let data = string_to_arrays(input);
        assert_eq!(10, data.len());
        assert_eq!(10, data[0].len());

        let want = 9;
        let got = count_cross(&data);
        assert_eq!(want, got);
    }
}

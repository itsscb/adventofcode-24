use std::{error::Error, fs};

use guard::Guard;

mod guard;

/// Solves the problem for day 06.
///
/// # Errors
///
/// This function will return an error if the file cannot be read or if the input is invalid.
#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub fn solve_day06(path: &str) -> Result<(i32, i32), Box<dyn Error>> {
    let content = fs::read_to_string(path)?;

    let (mut guard, map) = parse_input(&content);
    let part_two = find_possible_obstacles(guard.start(), &map);
    calc_guard_path(&mut guard, &map);
    let part_one = guard.visited();

    Ok((part_one, part_two))
}

#[allow(
    unused_mut,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap
)]
fn find_possible_obstacles(_start: (usize, usize), _map: &[Vec<char>]) -> i32 {
    let mut obstacles: Vec<(usize, usize)> = vec![];

    obstacles.len() as i32
}

fn calc_guard_path(guard: &mut Guard, map: &[Vec<char>]) {
    loop {
        let (x, y) = guard.next_move();
        if x >= map.len() || y >= map[0].len() {
            break;
        }
        // dbg!(x, y);
        let c = map[x][y];
        if c == '#' {
            let new_direction = match guard.direction() {
                guard::Direction::Up => &guard::Direction::Right,
                guard::Direction::Down => &guard::Direction::Left,
                guard::Direction::Left => &guard::Direction::Up,
                guard::Direction::Right => &guard::Direction::Down,
                guard::Direction::Unknown => &guard::Direction::Unknown,
            };
            guard.move_guard(new_direction);
        } else {
            let direction = guard.direction().clone();
            guard.move_guard(&direction);
        }
    }
}

fn parse_input(input: &str) -> (guard::Guard, Vec<Vec<char>>) {
    let mut guard = guard::Guard::new(0, 0, guard::Direction::Unknown);
    let map = input
        .split('\n')
        .enumerate()
        .flat_map(|(i, r)| {
            if r.contains('^') || r.contains('v') || r.contains('<') || r.contains('>') {
                let () = r
                    .chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '^' || *c == 'v' || *c == '<' || *c == '>')
                    .map(|(j, c)| {
                        let d = guard::Direction::from(c);
                        guard = guard::Guard::new(i, j, d);
                    })
                    .collect::<()>();
            }
            vec![r.chars().collect()]
        })
        .collect();
    (guard, map)
}

#[cfg(test)]
mod tests {
    use guard::Guard;

    use super::*;

    #[test]
    fn test_solve_day06() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#........
........#.
#.........
......#^..";

        let (mut guard, map) = parse_input(input);
        assert_eq!(guard, Guard::new(9, 7, guard::Direction::Up));

        calc_guard_path(&mut guard, &map);
        let want = 7;
        let got = guard.visited();
        assert_eq!(got, want);
    }
}

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Guard {
    x: usize,
    y: usize,
    direction: Direction,
    visited: HashSet<(usize, usize)>,
}

impl Guard {
    #[allow(dead_code)]
    pub fn new(x: usize, y: usize, direction: Direction) -> Self {
        let mut visited = HashSet::new();
        visited.insert((x, y));
        Self {
            x,
            y,
            direction,
            visited,
        }
    }

    #[allow(dead_code)]
    pub const fn direction(&self) -> &Direction {
        &self.direction
    }

    #[allow(dead_code)]
    pub const fn start(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    pub fn visited(&self) -> i32 {
        self.visited.len() as i32
    }

    #[allow(dead_code)]
    pub const fn next_move(&self) -> (usize, usize) {
        match self.direction {
            Direction::Up => (self.x - 1, self.y),
            Direction::Down => (self.x + 1, self.y),
            Direction::Left => (self.x, self.y - 1),
            Direction::Right => (self.x, self.y + 1),
            Direction::Unknown => (self.x, self.y),
        }
    }

    #[allow(dead_code)]
    pub fn move_guard(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.x -= 1,
            Direction::Down => self.x += 1,
            Direction::Left => self.y -= 1,
            Direction::Right => self.y += 1,
            Direction::Unknown => {
                return;
            }
        }
        self.direction = direction.clone();
        self.visited.insert((self.x, self.y));
        println!("Moved to: x: {} | y: {}", self.x, self.y);
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Unknown,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '^' => Self::Up,
            'v' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            _ => Self::Unknown,
        }
    }
}

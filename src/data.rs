#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn inversely(&self, other: &Direction) -> bool {
        if *self == Direction::Up && *other == Direction::Down {
            true
        } else if *self == Direction::Down && *other == Direction::Up {
            true
        } else if *self == Direction::Left && *other == Direction::Right {
            true
        } else if *self == Direction::Right && *other == Direction::Left {
            true
        } else {
            false
        }
    }
}

pub const FOOD: char = '@';
pub const SNAKE: char = '*';
pub const BLANK: char = ' ';
pub const ROW: usize = 30;
pub const COLUMN: usize = 30;
pub const FPS: f64 = 0.1;

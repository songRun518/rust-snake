pub const FOOD: char = '@';
pub const SNAKE: char = '*';
pub const BLANK: char = ' ';
pub const ROW: usize = 30;
pub const COLUMN: usize = 30;
pub const FPS: f64 = 1.0 / 60.0 * 5.0;

pub enum EndState {
    Die,
    Manual,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn inversely(&self, other: &Direction) -> bool {
        *self == Direction::Up && *other == Direction::Down
            || *self == Direction::Down && *other == Direction::Up
            || *self == Direction::Left && *other == Direction::Right
            || *self == Direction::Right && *other == Direction::Left
    }
}

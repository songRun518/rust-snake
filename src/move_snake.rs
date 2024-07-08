use crate::{Direction, BLANK, SNAKE, ROW, COLUMN};

pub fn movement(body: &mut Vec<((usize, usize), Direction)>, stage: &mut [[char; COLUMN]; ROW]) {
    for ((row, column), dir) in body.iter_mut() {
        stage[*row][*column] = BLANK;
        safe_move(row, column, &dir);
        stage[*row][*column] = SNAKE;
    }
}

pub fn safe_move(row: &mut usize, column: &mut usize, dir: &Direction) {
    match dir {
        Direction::Up => {
            *row = if (*row as i32 - 1).is_negative() {
                ROW - 1
            } else {
                *row - 1
            }
        }
        Direction::Down => *row = if (*row + 1).lt(&ROW) { *row + 1 } else { 0 },
        Direction::Left => {
            *column = if (*column as i32 - 1).is_negative() {
                COLUMN - 1
            } else {
                *column - 1
            }
        }
        Direction::Right => {
            *column = if (*column + 1).lt(&COLUMN) {
                *column + 1
            } else {
                0
            }
        }
    }
}

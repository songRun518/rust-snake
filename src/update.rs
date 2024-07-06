use crate::Direction;
use k_board::{keyboard, keys::Keys};
use std::collections::HashMap;

pub fn input_key(
    body: &[((usize, usize), Direction)],
    record: &mut HashMap<(usize, usize), Direction>,
) {
    let dir = match keyboard::get_key_from_keyboard() {
        Keys::Up => Some(Direction::Up),
        Keys::Down => Some(Direction::Down),
        Keys::Left => Some(Direction::Left),
        Keys::Right => Some(Direction::Right),
        _ => None,
    };
    if let Some(dir) = dir {
        record.insert(body.first().unwrap().0, dir);
    }
}

pub fn update_pos(
    body: &mut Vec<((usize, usize), Direction)>,
    record: &mut HashMap<(usize, usize), Direction>,
) {
    let last = body.len() - 1;
    for (idx, (pos, dir)) in body.iter_mut().enumerate() {
        if record.contains_key(&pos) {
            if !dir.inversely(&record[pos]) {
                *dir = record[pos];
            }
            if idx == last {
                record.remove(&pos);
            }
        }
    }
}

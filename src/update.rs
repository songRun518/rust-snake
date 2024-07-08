use device_query::{DeviceQuery, DeviceState, Keycode};

use crate::Direction;
use std::collections::HashMap;

pub fn input_key(
    end: &mut bool,
    keyboard: &DeviceState,
    body: &[((usize, usize), Direction)],
    record: &mut HashMap<(usize, usize), Direction>,
) {
    let dir = match keyboard.get_keys().first() {
        Some(Keycode::Up) => Some(Direction::Up),
        Some(Keycode::Down) => Some(Direction::Down),
        Some(Keycode::Left) => Some(Direction::Left),
        Some(Keycode::Right) => Some(Direction::Right),
        Some(Keycode::E) => {
            *end = true;
            None
        }
        _ => None,
    };
    if let Some(dir) = dir {
        record.entry(body[0].0).or_insert(dir);
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

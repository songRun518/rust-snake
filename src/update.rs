use device_query::{DeviceQuery, DeviceState, Keycode};

use crate::data::{Direction, EndState};
use std::collections::HashMap;

pub fn input_key(
    keyboard: &DeviceState,
    body: &[((usize, usize), Direction)],
    record: &mut HashMap<(usize, usize), Direction>,
    whether_end: &mut bool,
    end_state: &mut EndState,
) {
    let dir = match keyboard.get_keys().first() {
        Some(Keycode::W) => Some(Direction::Up),
        Some(Keycode::S) => Some(Direction::Down),
        Some(Keycode::A) => Some(Direction::Left),
        Some(Keycode::D) => Some(Direction::Right),
        Some(Keycode::Escape) => {
            *whether_end = true;
            *end_state = EndState::Manual;
            None
        }
        _ => None,
    };
    if let Some(dir) = dir {
        record.entry(body[0].0).or_insert(dir);
    }
}

pub fn update_pos(
    body: &mut [((usize, usize), Direction)],
    record: &mut HashMap<(usize, usize), Direction>,
) {
    let last = body.len() - 1;
    for (idx, (pos, dir)) in body.iter_mut().enumerate() {
        if record.contains_key(pos) {
            if !dir.inversely(&record[pos]) {
                *dir = record[pos];
            }
            if idx == last {
                record.remove(pos);
            }
        }
    }
}

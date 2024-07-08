use crate::{move_snake::safe_move, Direction, COLUMN, FOOD, ROW, SNAKE};
use rand::{rngs::ThreadRng, Rng};
use std::collections::HashMap;

pub fn die(body: &[((usize, usize), Direction)]) -> bool {
    let mut die = false;
    let mut record_body: HashMap<(usize, usize), usize> = HashMap::new();
    for (pos, _) in body {
        record_body
            .entry(*pos)
            .and_modify(|ele| *ele += 1)
            .or_insert(1);
    }
    for val in record_body.values() {
        if *val > 1 {
            die = true;
            break;
        }
    }
    die
}

pub fn eat(
    rng: &mut ThreadRng,
    food: &mut (usize, usize),
    body: &mut Vec<((usize, usize), Direction)>,
    stage: &mut [[char; COLUMN]; ROW],
) {
    if stage[food.0][food.1] != FOOD {
        *food = (rng.gen_range(0..ROW), rng.gen_range(0..COLUMN));

        let mut row = body.last_mut().unwrap().0 .0;
        let mut column = body.last().unwrap().0 .1;
        match body.last().unwrap().1 {
            Direction::Up => {
                safe_move(&mut row, &mut column, &Direction::Down);
                body.push(((row, column), Direction::Up));
            }
            Direction::Down => {
                safe_move(&mut row, &mut column, &Direction::Up);
                body.push(((row, column), Direction::Down));
            }
            Direction::Left => {
                safe_move(&mut row, &mut column, &Direction::Right);
                body.push(((row, column), Direction::Left));
            }
            Direction::Right => {
                safe_move(&mut row, &mut column, &Direction::Left);
                body.push(((row, column), Direction::Right));
            }
        }

        stage[row][column] = SNAKE;
        stage[food.0][food.1] = FOOD;
    }
}

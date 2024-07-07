mod after_move;
mod data;
mod move_snake;
mod screen;
mod update;

use after_move::{die, eat};
use data::{Direction, BLANK, COLUMN, FOOD, FPS, ROW, SNAKE};
use device_query::DeviceState;
use move_snake::movement;
use screen::fresh_screen;
use update::{input_key, update_pos};

use rand::Rng;
use std::{collections::HashMap, time::Instant};

fn main() {
    let keyboard = DeviceState::new();
    let mut timer = Instant::now();
    let mut rng = rand::thread_rng();
    let mut food = (rng.gen_range(0..ROW), rng.gen_range(0..COLUMN));
    let mut record_path: HashMap<(usize, usize), Direction> = HashMap::new();
    let mut stage = [[BLANK; COLUMN]; ROW];
    let mut body: Vec<((usize, usize), Direction)> = vec![(15, 4), (15, 3), (15, 2), (15, 1)]
        .into_iter()
        .zip([Direction::Right; 4].into_iter())
        .collect();

    stage[food.0][food.1] = FOOD;
    for ((row, column), _) in &body {
        stage[*row][*column] = SNAKE;
    }

    fresh_screen(&stage);
    loop {
        if timer.elapsed().as_secs_f64() > FPS {
            timer = Instant::now();

            input_key(&keyboard, &body, &mut record_path);
            update_pos(&mut body, &mut record_path);
            movement(&mut body, &mut stage);
            eat(&mut rng, &mut food, &mut body, &mut stage);

            fresh_screen(&stage);
            println!("{}", body.len());

            if die(&body) {
                println!("You die");
                break;
            }
        }
    }
}

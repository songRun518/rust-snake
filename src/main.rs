use std::io::{self, BufWriter, Write};
use std::{collections::HashMap, time::Instant};

use device_query::DeviceState;
use rand::Rng;

use snake_cli::after_move::{die, eat};
use snake_cli::data::{Direction, BLANK, COLUMN, FOOD, FPS, ROW, SNAKE};
use snake_cli::move_snake::movement;
use snake_cli::screen::fresh_screen;
use snake_cli::update::{input_key, update_pos};

fn main() {
    let mut handle: BufWriter<io::StdoutLock<'_>> = BufWriter::new(io::stdout().lock());
    let mut end = false;
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

    fresh_screen(&mut handle, &stage);
    loop {
        input_key(&mut end, &DeviceState::new(), &body, &mut record_path);
        if end {
            println!("游戏结束");
            break;
        }

        if timer.elapsed().as_secs_f64() > FPS {
            timer = Instant::now();

            update_pos(&mut body, &mut record_path);
            movement(&mut body, &mut stage);
            eat(&mut rng, &mut food, &mut body, &mut stage);

            fresh_screen(&mut handle, &stage);
            writeln!(handle, "得分: {}", body.len() - 4).unwrap();
            handle.flush().unwrap();

            if die(&body) {
                println!("你死了");
                break;
            }
        }
    }
    
    drop(handle);
    println!("\nPress \"Enter\" to continue......");
    match std::io::stdin().read_line(&mut String::new()) {
        Ok(_) => {}
        Err(_) => {}
    }
}

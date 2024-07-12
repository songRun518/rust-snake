use crate::{COLUMN, ROW};

pub fn fresh_screen(stage: &[[char; COLUMN]; ROW]) {
    clearscreen::clear().unwrap();
    println!("{}", "-".repeat(COLUMN + 2));
    for row in stage {
        print!("|");
        for column in row {
            print!("{}", column);
        }
        println!("|");
    }
    println!("{}", "-".repeat(COLUMN + 2));
    println!("按 \"Up\" \"Down\" \"Left\" \"Right\" 控制, 按 \"E\" 结束");
}

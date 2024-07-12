use std::io::{self, BufWriter, Write};

use crate::data::{COLUMN, ROW};

pub fn fresh_screen(handle: &mut BufWriter<io::StdoutLock<'_>>, stage: &[[char; COLUMN]; ROW]) {
    clearscreen::clear().unwrap();
    writeln!(handle, "{}", "-".repeat(COLUMN + 2)).unwrap();
    for row in stage {
        write!(handle, "|").unwrap();
        for column in row {
            write!(handle, "{}", column).unwrap();
        }
        writeln!(handle, "|").unwrap();
    }
    writeln!(handle, "{}", "-".repeat(COLUMN + 2)).unwrap();
    writeln!(
        handle,
        "按 \"W\" \"A\" \"S\" \"D\" 控制, 按 \"ESC\" 结束"
    )
    .unwrap();
}

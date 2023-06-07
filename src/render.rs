use std::io::{Stdout, Write};

use crossterm::{
    cursor::MoveTo,
    style::{Color, SetBackgroundColor, Stylize},
    terminal::{self, Clear, ClearType}, QueueableCommand,
};

use crate::{frame::Frame, NUM_COLS, NUM_ROWS};

pub fn render(stdout: &mut Stdout, cur_frame: &Frame, forced: bool) {
    let (width, height) = terminal::size().unwrap();
    if forced {
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();

    }

    for x in 0..NUM_ROWS {
        for y in 0..NUM_COLS {
            // if cur_frame[x][y] != last_frame[x][y] || forced {
                stdout.queue(MoveTo( (width - NUM_ROWS as u16) / 2 + x as u16, (height - NUM_COLS as u16) / 2 + y as u16)).unwrap();
                print!("{}", cur_frame[x][y].ch.with(cur_frame[x][y].color),)
            // }
        }
    }
    stdout.flush().unwrap();
}

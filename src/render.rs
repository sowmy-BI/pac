use std::io::{Stdout, Write};

use crossterm::{
    cursor::MoveTo,
    style::{Color, SetBackgroundColor, Stylize, SetColors, SetAttribute, Colors, },
    terminal::{self, Clear, ClearType},
    QueueableCommand,
};

use crate::{frame::Frame, NUM_COLS, NUM_ROWS};

pub fn render(stdout: &mut Stdout, last_frame: &Frame, cur_frame: &Frame, forced: bool, score: usize) {
    let (width, height) = terminal::size().unwrap();
    if forced {
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }

    for x in 0..NUM_ROWS {
        for y in 0..NUM_COLS {
            if cur_frame[x][y] != last_frame[x][y] || forced {
                stdout
                    .queue(MoveTo(
                        (width - NUM_COLS as u16) / 2 + y as u16,
                        (height - NUM_ROWS as u16) / 2 + x as u16,
                    ))
                    .unwrap();
                print!("{}", cur_frame[x][y].ch.with(cur_frame[x][y].color),)
            }
        }
    }
    // update score
    stdout.queue(MoveTo(
        (width / 2) + (NUM_COLS/ 2 - 10)  as u16,
        (height - NUM_ROWS as u16) / 2 - 3 as u16,
    )).unwrap();
    stdout.queue(SetColors(Colors::new(Color::Green, Color::Black))).unwrap();
    stdout.queue(SetAttribute(crossterm::style::Attribute::Bold)).unwrap();
    print!("score: {}", score);

    stdout.flush().unwrap();
}

use crossterm::style::Color;

use crate::{NUM_COLS, NUM_ROWS};

pub struct Pixel {
    pub ch: char,
    pub color: Color,
}

pub type Frame = Vec<Vec<Pixel>>;

pub fn new_frame() -> Frame {
    let mut frame: Vec<Vec<Pixel>> = Vec::new();
    for _ in 0..NUM_ROWS {
        let mut rows: Vec<Pixel> = Vec::new();
        for _ in 0..NUM_COLS {
            rows.push(Pixel {
                ch: ' ',
                color: Color::Red,
            })
        }
        frame.push(rows);
    }

    frame
}

impl PartialEq for Pixel {
    fn eq(&self, other: &Self) -> bool {
        self.ch == other.ch && self.color == other.color
    }
}
pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}

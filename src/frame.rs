use std::collections::HashMap;

use crossterm::style::Color;

use crate::{NUM_COLS, NUM_ROWS};

use crate::map::get_current_element_based_level;

pub struct Pixel {
    pub ch: char,
    pub color: Color,
}

pub type Frame = Vec<Vec<Pixel>>;

pub fn new_frame(level: i8, visited_map: &HashMap<String, bool>) -> Frame {
    let mut frame: Vec<Vec<Pixel>> = Vec::new();
    for x in 0..NUM_ROWS {
        let mut rows: Vec<Pixel> = Vec::new();
        for y in 0..NUM_COLS {
            let pixel  = get_current_element_based_level(level, x, y);
            rows.push(Pixel {
                ch: match pixel.ch {
                    '.' => {
                        if visited_map.get(&format!("{}{}", x, y)).is_none() {
                            '.'
                        } else {
                        ' '
                        }
                    },
                    x => x
                }, 
                color: pixel.color,
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

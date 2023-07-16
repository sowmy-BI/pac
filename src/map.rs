use crossterm::style::Color;

use crate::{frame::{Frame, Pixel}, NUM_ROWS};


pub fn level_1(frame: &mut Frame) {
    for y in 0..NUM_ROWS {
        frame[0][y] = Pixel {
            ch: '#',
            color: Color::Blue
        }
    };
    
}
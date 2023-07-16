use std::time::Duration;

use crossterm::style::Color;
use rusty_time::Timer;

use crate::{
    direction::Direction,
    frame::{Drawable, Frame, Pixel},
    NUM_COLS, NUM_ROWS,
};

pub struct PacMan {
    x: usize,
    y: usize,
    ch: char,
    cur_dir: Direction,
    timer: Timer,
    breath: Timer,
}

impl PacMan {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            ch: '}',
            cur_dir: Direction::Left,
            timer: Timer::from_millis(200),
            breath: Timer::from_millis(100),
        }
    }

    pub fn update_position(&mut self, delta: Duration) {
        self.timer.update(delta);
        if self.timer.ready {
            match self.cur_dir {
                Direction::Down => self.move_down(),
                Direction::Left => self.move_left(),
                Direction::Right => self.move_right(),
                Direction::Up => self.move_up(),
            };
            self.timer.reset();
        }
    }

    pub fn update_character(&mut self, delta: Duration) {
        self.breath.update(delta);
        if self.breath.ready {
            match self.cur_dir {
                Direction::Right => {
                    self.ch = match self.ch {
                        '{' => '(',
                        '(' => '<',
                        _ => '{',
                    };
                }
                Direction::Left => {
                    self.ch = match self.ch {
                        '}' => ')',
                        ')' => '>',
                        _ => '}',
                    };
                }
                Direction::Down => {
                    self.ch = match self.ch {
                        '⋀' => '^',
                        '^' => '|',
                        _ => '^',
                    };
                }
                Direction::Up => {
                    self.ch = match self.ch {
                        'V' => '˅',
                        '˅' => '|',
                        _ => 'V',
                    };
                }
            }
            self.breath.reset();
        }
    }

    pub fn update_direction(&mut self, direction: Direction) {
        // todo: update the position the availability
        self.cur_dir = direction
    }

    fn move_up(&mut self) {
        if self.y > 0 {
            self.y -= 1
        }
    }

    fn move_down(&mut self) {
        if self.y < NUM_COLS - 1 {
            self.y += 1;
        }
    }

    fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    fn move_right(&mut self) {
        if self.x < NUM_ROWS - 1 {
            self.x += 1;
        }
    }
}

impl Drawable for PacMan {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = Pixel {
            ch: self.ch,
            color: Color::Yellow,
        }
    }
}

use std::{time::Duration, collections::HashMap};

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
            x: 7,
            y: 28,
            ch: '}',
            cur_dir: Direction::Up,
            timer: Timer::from_millis(200),
            breath: Timer::from_millis(90),
        }
    }

    pub fn update_position(&mut self, delta: Duration, frame: &Frame,visited_map: &mut HashMap<String, bool>) {
        self.timer.update(delta);
        if self.timer.ready {
            match self.cur_dir {
                Direction::Down => self.move_down(&frame),
                Direction::Left => self.move_left(&frame),
                Direction::Right => self.move_right(&frame),
                Direction::Up => self.move_up(&frame),
            };

            if visited_map.get(&format!("{}{}", self.x, self.y)).is_none() {
                visited_map.insert(format!("{}{}", self.x, self.y), true);
            }
            
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
        self.cur_dir = direction
    }

    fn move_up(&mut self, frame: &Frame) {
        if self.x > 0 && frame[self.x - 1][self.y].ch != '#' {
            self.x -= 1
        }
    }

    fn move_down(&mut self, frame: &Frame) {
        if self.x < NUM_ROWS - 1 && frame[self.x + 1][self.y ].ch != '#' {
            self.x += 1;
        }
    }

    fn move_left(&mut self, frame: &Frame) {
        if self.y > 0 && frame[self.x][self.y - 1].ch != '#' {
            self.y -= 1;
        } else if self.y == 0 && (frame[self.x][self.y].ch == '.' || frame[self.x][self.y].ch == ' ') {
            self.y = NUM_COLS - 1;
        }
    }

    fn move_right(&mut self, frame: &Frame) {
        if self.y < NUM_COLS - 1 && frame[self.x ][self.y + 1].ch != '#'{
            self.y += 1;
        } else if self.y == NUM_COLS - 1 && (frame[self.x][self.y].ch == '.' || frame[self.x][self.y].ch == ' ') {
            self.y = 0;
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

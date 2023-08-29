use std::time::Duration;

use crossterm::style::Color;
use rand::Rng;
use rusty_time::Timer;

use crate::{frame::{Drawable, Pixel, Frame}, direction::{Direction, self}, NUM_ROWS, NUM_COLS};

struct Ghost {
    x: usize,
    y: usize,
    color: Color,
    direction: Direction,
    timer: Timer
}

pub struct Ghosts {
    ghosts: Vec<Ghost>
}

impl Ghosts {
    pub fn new() -> Self {
        Self{ ghosts: vec![Ghost{
            x: 9,
            y: 27,
            color: Color::Red,
            timer: Timer::from_millis(200),
            direction: Direction::Up
        }, Ghost{
            x: 9,
            y: 29,
            color: Color::Cyan,
            timer: Timer::from_millis(220),
            direction: Direction::Up
        },Ghost{
            x: 9,
            y: 31,
            color: Color::DarkMagenta,
            timer: Timer::from_millis(150),
            direction: Direction::Up
        }]}
    }


    pub fn update_position(&mut self, frame: &Frame, delta: Duration, rng: &mut rand::rngs::ThreadRng) {
        
        for ghost in &mut self.ghosts {
            ghost.timer.update(delta);
            if ghost.timer.ready {
                match get_available_position(&frame, &ghost, rng){
                    Direction::Up => {
                        ghost.x -= 1; 
                        ghost.direction = Direction::Up;
                    }
                    Direction::Right => {
                        ghost.y += 1; 
                        ghost.direction = Direction::Right;
                    },

                    Direction::Down => {
                        ghost.x += 1;
                        ghost.direction = Direction::Down;
                    },
                    Direction::Left => {
                        ghost.y -= 1; 
                        ghost.direction = Direction::Left;
                    },
                   
                }
                ghost.timer.reset();
            }
        }
    }
}


fn get_available_position(frame: &Frame, ghost: &Ghost,  rng: &mut rand::rngs::ThreadRng)-> Direction {
    
    fn check_updation_available_for_given_direction(frame: &Frame, ghost: &Ghost, direction: &Direction)-> bool {
        let Ghost{x, y, ..} = *ghost;
        match direction {
            Direction::Up => {
                if x > 0 && frame[x - 1][y].ch != '#' {
                    return true;
                }
                false
            },
            Direction::Right => {
                if y < NUM_COLS - 1 && frame[x ][y + 1].ch != '#' {
                    return true;
                }
                false
            }
            Direction::Down => {
                if x < NUM_ROWS - 1 && frame[x + 1][y ].ch != '#'  {
                    return true;
                }
                false
            }
            Direction::Left => {
                if y > 0 && frame[x][y - 1].ch != '#'  {
                    return true
                }
                false
            }
        }
    }


    if check_updation_available_for_given_direction(frame, &ghost, &ghost.direction) {
        
        return match ghost.direction {
            Direction::Up => Direction::Up,
            Direction::Right => Direction::Right,
            Direction::Down => Direction::Down,
            Direction::Left => Direction::Left
        }
    } 

    let dir_flag = rng.gen();

    match ghost.direction {
        Direction::Up => {
            if dir_flag && check_updation_available_for_given_direction(frame, &ghost, &Direction::Left) {
                return Direction::Left
            }else if  check_updation_available_for_given_direction(frame, &ghost, &Direction::Right) {
                return Direction::Right
            } 
            return Direction::Down
        },
        Direction::Right => {
            if dir_flag && check_updation_available_for_given_direction(frame, &ghost, &Direction::Up) {
                return Direction::Up
            }else if  check_updation_available_for_given_direction(frame, &ghost, &Direction::Down) {
                return Direction::Down
            } 
            return Direction::Left
        },
        Direction::Down => {
            if dir_flag && check_updation_available_for_given_direction(frame, &ghost, &Direction::Right) {
                return Direction::Right
            }else if  check_updation_available_for_given_direction(frame, &ghost, &Direction::Left) {
                return Direction::Left
            } 
            return Direction::Up
        },
        Direction::Left => {
            if dir_flag && check_updation_available_for_given_direction(frame, &ghost, &Direction::Down) {
                return Direction::Down
            }else if  check_updation_available_for_given_direction(frame, &ghost, &Direction::Up) {
                return Direction::Up
            } 
            return Direction::Right
        }
    }


}


impl Drawable for Ghosts {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        for ghost in &self.ghosts {
            frame[ghost.x][ghost.y] = Pixel {
                ch: 'ἦ',
                color: ghost.color
            }
        }
    }
}
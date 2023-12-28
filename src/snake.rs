#![no_main]
#![no_std]
use alloc::vec::{self, Vec};
use microbit::board::Buttons;

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Down,
    Right,
    Left,
    Up,
}

pub struct Snake {
    body_length: u8,
    pub body_position: Vec<(usize, usize)>,
    pub current_direction: Direction,
}

impl Snake {
    pub fn _move(&mut self, move_direction: Direction) {
        let mut current_head: &(usize, usize) = self.body_position.first().unwrap();
        match move_direction {
            Direction::Down => {
                self.body_position
                    .insert(0, Snake::move_down(*current_head));
                self.body_position.pop();
                self.current_direction = Direction::Down
            }
            Direction::Right => {
                self.body_position
                    .insert(0, Snake::move_right(*current_head));
                self.body_position.pop();
                self.current_direction = Direction::Right
            }
            Direction::Left => {
                self.body_position
                    .insert(0, Snake::move_left(*current_head));
                self.body_position.pop();
                self.current_direction = Direction::Left
            }
            Direction::Up => {
                self.body_position.insert(0, Snake::move_up(*current_head));
                self.body_position.pop();
                self.current_direction = Direction::Up
            }
        }
    }

    fn determine_direction(&self, input: Buttons) -> Direction {
        let current_direction: Direction = self.current_direction;
        match input {
            button_a => {
                if current_direction == Direction::Left {
                    Direction::Down
                } else if current_direction == Direction::Up || current_direction == Direction::Down
                {
                    Direction::Left
                } else {
                    panic!()
                }
            }
            button_b => {
                if current_direction == Direction::Right {
                    Direction::Up
                } else if current_direction == Direction::Up || current_direction == Direction::Down
                {
                    Direction::Right
                } else {
                    panic!()
                }
            }
        }
    }

    pub fn new() -> Self {
        let mut body_pos: Vec<(usize, usize)> = Vec::new();
        body_pos = [(0, 0)].to_vec();
        Self {
            body_length: 1,
            body_position: body_pos,
            current_direction: Direction::Down,
        }
    }
}

impl Snake {
    pub fn move_down(current_head: (usize, usize)) -> (usize, usize) {
        let mut new_head: (usize, usize) = (0, 0);
        if current_head.0 == 4 {
            new_head.0 = 0;
            new_head.1 = current_head.1;
            new_head
        } else {
            new_head.0 = current_head.0 + 1;
            new_head.1 = current_head.1;
            new_head
        }
    }

    pub fn move_right(current_head: (usize, usize)) -> (usize, usize) {
        let mut new_head: (usize, usize) = (0, 0);
        if current_head.1 == 4 {
            new_head.1 = 0;
            new_head.0 = current_head.0;
            new_head
        } else {
            new_head.1 = current_head.1 + 1;
            new_head.0 = current_head.0;
            new_head
        }
    }

    pub fn move_left(current_head: (usize, usize)) -> (usize, usize) {
        let mut new_head: (usize, usize) = (0, 0);
        if current_head.1 == 0 {
            new_head.1 = 4;
            new_head.0 = current_head.0;
            new_head
        } else {
            new_head.1 = current_head.1 - 1;
            new_head.0 = current_head.0;
            new_head
        }
    }

    pub fn move_up(current_head: (usize, usize)) -> (usize, usize) {
        let mut new_head: (usize, usize) = (0, 0);
        if current_head.0 == 0 {
            new_head.0 = 4;
            new_head.1 = current_head.1;
            new_head
        } else {
            new_head.0 = current_head.0 - 1;
            new_head.1 = current_head.1;
            new_head
        }
    }
}

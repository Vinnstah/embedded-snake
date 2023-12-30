use alloc::{
    borrow::ToOwned,
    vec::{self, Vec},
};

use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};

use crate::snake::{Direction, Snake};

#[derive(PartialEq, Copy, Clone)]
pub struct Game {
    pub food_spawned: bool,
    pub food_position: Option<(usize, usize)>,
    pub current_board: [[u8; 5]; 5],
    pub default_board: [[u8; 5]; 5],
}

impl Game {
    /// Creates a new [`Game`].
    pub fn new() -> Self {
        Self {
            food_spawned: false,
            food_position: None,
            current_board: [
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
            ],
            default_board: [
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
            ],
        }
    }
    pub fn tick(&mut self, snake: &mut Snake) -> Option<Vec<(usize, usize)>> {
        let current_head = snake.body_positions.first().unwrap().clone();
        let mut current_body_positions = &mut snake.body_positions;
        let mut new_head = (0, 0);

        match snake.current_direction {
            Direction::Down => {
                new_head = Snake::move_down(current_head.to_owned());
                if self.food_position != Some(new_head) {
                    current_body_positions.pop();
                } else {
                    self.food_spawned = false;
                    self.food_position = None;
                }
                current_body_positions.insert(0, new_head);
                Some(current_body_positions.to_vec())
            }
            Direction::Up => {
                new_head = Snake::move_up(current_head.to_owned());
                if self.food_position != Some(new_head) {
                    current_body_positions.pop();
                }  else {
                    self.food_spawned = false;
                    self.food_position = None;
                }
                current_body_positions.insert(0, new_head);
                Some(current_body_positions.to_vec())
            }
            Direction::Right => {
                new_head = Snake::move_right(current_head.to_owned());
                if self.food_position != Some(new_head) {
                    current_body_positions.pop();
                }  else {
                    self.food_spawned = false;
                    self.food_position = None;
                }
                current_body_positions.insert(0, new_head);
                Some(current_body_positions.to_vec())
            }
            Direction::Left => {
                new_head = Snake::move_left(current_head.to_owned());
                if self.food_position != Some(new_head) {
                    current_body_positions.pop();
                }  else {
                    self.food_spawned = false;
                    self.food_position = None;
                }
                current_body_positions.insert(0, new_head);
                Some(current_body_positions.to_vec())
            }
        }
    }

    pub fn spawn_food(&mut self, snake_position: &Vec<(usize, usize)>) -> [[u8; 5]; 5] {
        let mut possible_spawn_locations: Vec<(usize, usize)> = Vec::new();

        for (row_id, row) in self.current_board.iter().enumerate() {
            for (column_id, column) in row.iter().enumerate() {
                if u128::from_be((*column).into()) == 0 {
                    possible_spawn_locations.push((row_id, column_id))
                } else {
                    continue;
                }
            }
        }

        let mut small_rng =
            SmallRng::seed_from_u64(snake_position.last().unwrap().0.try_into().unwrap());
        possible_spawn_locations.shuffle(&mut small_rng);
        let chosen_spawn = possible_spawn_locations.first().expect("Failed");
        self.food_position = Some(*chosen_spawn);
        self.food_spawned = true;
        self.current_board[chosen_spawn.0][chosen_spawn.1] = 1;
        self.current_board
    }

    fn game_over() -> Vec<[[u8; 5]; 5]> {
        let first: [[u8; 5]; 5] = [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [1, 1, 1, 1, 1],
        ];
        let second: [[u8; 5]; 5] = [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1],
        ];
        let thrid: [[u8; 5]; 5] = [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1],
        ];
        let fourth: [[u8; 5]; 5] = [
            [0, 0, 0, 0, 0],
            [1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1],
        ];
        let fifth: [[u8; 5]; 5] = [
            [1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1],
        ];

        let mut game_over: Vec<[[u8; 5]; 5]> = Vec::new();

        game_over.push(first);
        game_over.push(second);
        game_over.push(thrid);
        game_over.push(fourth);
        game_over.push(fifth);
        game_over
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

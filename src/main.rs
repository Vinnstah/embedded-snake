#![no_main]
#![no_std]
mod snake;

// #[macro_use]
extern crate alloc;

use core::borrow::Borrow;

use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};

use alloc::{
    borrow::ToOwned,
    vec::{self, Vec},
};
use cortex_m_rt::entry;
use embedded_alloc::Heap;
use embedded_hal as _;
use microbit::{
    board::Board, board::Buttons, display::blocking::Display, hal::prelude::*, hal::Timer,
};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use snake::{Direction, Snake};

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[entry]
fn main() -> ! {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    rtt_init_print!();
    let mut board = Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);

    let mut default_board = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    let mut display = Display::new(board.display_pins);
    let mut snake = Snake::new();
    let mut game = Game::new();

    loop {
        default_board = [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ];

        if !game.food_spawned {
            default_board = Game::spawn_food(&mut game, default_board, &snake.body_position);
        }
        if let Ok(true) = board.buttons.button_b.is_low() {
            match snake.current_direction {
                Direction::Down => snake._move(Direction::Right),
                Direction::Right => snake._move(Direction::Down),
                Direction::Left => snake._move(Direction::Down),
                Direction::Up => snake._move(Direction::Right),
            }
        }
        if let Ok(true) = board.buttons.button_a.is_low() {
            match snake.current_direction {
                Direction::Down => snake._move(Direction::Left),
                Direction::Right => snake._move(Direction::Up),
                Direction::Left => snake._move(Direction::Up),
                Direction::Up => snake._move(Direction::Left),
            }
        }
        for snake_bits in &snake.body_position {
            default_board[snake_bits.0][snake_bits.1] = 1;
            // default_board = Game::spawn_food(default_board, &snake.body_position);
            rprintln!("{:#?}", snake.body_position);
        }
        default_board[game.food_position.unwrap().0][game.food_position.unwrap().1] = 1;
        display.show(&mut timer, default_board, 1000);
        timer.delay_ms(100u16);
        display.clear();
        snake.body_position = Game::tick(&mut snake).unwrap()
    }
}

struct Game {
    food_spawned: bool,
    food_position: Option<(usize, usize)>,
    board: [[u8; 5]; 5],
}

impl Game {
    /// Creates a new [`Game`].
    fn new() -> Self {
        Self {
            food_spawned: false,
            food_position: None,
            board: [
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
            ],
        }
    }
    fn tick(snake: &mut Snake) -> Option<Vec<(usize, usize)>> {
        let current_head = snake.body_position.first().unwrap().clone();
        let mut current_body_position = &mut snake.body_position;
        let mut new_head = (0, 0);

        match snake.current_direction {
            Direction::Down => {
                current_body_position.pop();
                new_head = Snake::move_down(current_head.to_owned());
                current_body_position.insert(0, new_head);
                Some(current_body_position.to_vec())
            }
            Direction::Up => {
                current_body_position.pop();
                new_head = Snake::move_up(current_head.to_owned());
                current_body_position.insert(0, new_head);
                Some(current_body_position.to_vec())
            }
            Direction::Right => {
                current_body_position.pop();
                new_head = Snake::move_right(current_head.to_owned());
                current_body_position.insert(0, new_head);
                Some(current_body_position.to_vec())
            }
            Direction::Left => {
                current_body_position.pop();
                new_head = Snake::move_left(current_head.to_owned());
                current_body_position.insert(0, new_head);
                Some(current_body_position.to_vec())
            }
        }
    }

    pub fn spawn_food(
        &mut self,
        current_board: [[u8; 5]; 5],
        snake_position: &Vec<(usize, usize)>,
    ) -> [[u8; 5]; 5] {
        let mut current_board = current_board;
        let mut possible_spawn_locations: Vec<(usize, usize)> = Vec::new();

        for (row_id, row) in current_board.iter().enumerate() {
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
        current_board[chosen_spawn.0][chosen_spawn.1] = 1;
        current_board
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

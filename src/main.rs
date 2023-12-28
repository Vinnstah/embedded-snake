#![no_main]
#![no_std]
mod snake;

// #[macro_use]
extern crate alloc;

use alloc::{
    borrow::ToOwned,
    vec::{self, Vec},
};
use core::{
    borrow::{Borrow, BorrowMut},
    ops::Index,
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

    loop {
        snake.body_position = Game::game_tick(&mut snake);
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
        }
        rprintln!("{:#?}", snake.body_position);
        // rprintln!("{:#?}", default_board);
        display.show(&mut timer, default_board, 1000);
        timer.delay_ms(100u16);
        default_board = [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ];
    }
}

struct Game {}

impl Game {
    fn game_tick(snake: &mut Snake) -> Vec<(usize, usize)> {
        let current_head = snake.body_position.first().unwrap().clone();
        let mut current_body_position = &mut snake.body_position;
        let mut new_head = (0, 0);

        match snake.current_direction {
            Direction::Down => {
                current_body_position.pop();
                new_head = Snake::move_down(current_head.to_owned());
                current_body_position.insert(0, new_head);
                current_body_position.to_vec()
            }
            Direction::Up => {
                current_body_position.pop();
                new_head = Snake::move_up(current_head.to_owned());
                current_body_position.insert(0, new_head);
                current_body_position.to_vec()
            }
            Direction::Right => {
                current_body_position.pop();
                new_head = Snake::move_right(current_head.to_owned());
                current_body_position.insert(0, new_head);
                current_body_position.to_vec()
            }
            Direction::Left => {
                current_body_position.pop();
                new_head = Snake::move_left(current_head.to_owned());
                current_body_position.insert(0, new_head);
                current_body_position.to_vec()
            }
        }
    }

    fn spawn_food(
        current_board: [[i32; 5]; 5],
        snake_position: Vec<(usize, usize)>,
    ) -> [[i32; 5]; 5] {
        let mut current_board = current_board;
        let mut possible_spawn_locations: Vec<(usize, usize)> = Vec::new();

        possible_spawn_locations = snake_position
            .into_iter()
            .filter(|position| current_board.get(index) !current_board.binary_search(position))
            .collect();
        let chosen_spawn = possible_spawn_locations.first().unwrap();
        current_board[chosen_spawn.0][chosen_spawn.1] = 1;
        current_board
    }
}

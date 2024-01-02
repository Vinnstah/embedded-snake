#![no_main]
#![no_std]
mod game;
mod snake;

extern crate alloc;

use core::borrow::Borrow;

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

use crate::game::Game;

// Global allocator to allow for Vec<> usage.
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

    let mut display = Display::new(board.display_pins);
    let mut game = Game::new();
    let mut snake = Snake::new(game);

    loop {
        game.current_board = game.default_board;

        if !game.food_spawned {
            game.current_board = Game::spawn_food(&mut game, &snake.body_positions);
        }
        if let Ok(true) = board.buttons.button_b.is_low() {
            match snake.current_direction {
                Direction::Down => snake._move(Direction::Right).unwrap(),
                Direction::Right => snake._move(Direction::Down).unwrap(),
                Direction::Left => snake._move(Direction::Down).unwrap(),
                Direction::Up => snake._move(Direction::Right).unwrap(),
            };
        }
        if let Ok(true) = board.buttons.button_a.is_low() {
            match snake.current_direction {
                Direction::Down => snake._move(Direction::Left).unwrap(),
                Direction::Right => snake._move(Direction::Up).unwrap(),
                Direction::Left => snake._move(Direction::Up).unwrap(),
                Direction::Up => snake._move(Direction::Left).unwrap(),
            };
        }
        for snake_bits in &snake.body_positions {
            game.current_board[snake_bits.0][snake_bits.1] = 1;
        }
        game.current_board[game.food_position.unwrap().0][game.food_position.unwrap().1] = 1;
        display.show(&mut timer, game.current_board, 800);
        timer.delay_ms(10u16);

        match Game::tick(&mut game, &mut snake) {
            Some(body_positions) => snake.body_positions = body_positions,
            None => {
                for game_over_screen in Game::game_over() {
                    display.show(&mut timer, game_over_screen, 100)
                }
                panic!();
            }
        }
    }
}

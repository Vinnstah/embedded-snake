#![no_main]
#![no_std]
mod snake;

// #[macro_use]
extern crate alloc;

use cortex_m_rt::entry;
use embedded_alloc::Heap;
use embedded_hal as _;
use microbit::{
    board::Board, board::Buttons, display::blocking::Display, hal::prelude::*, hal::Timer,
};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;
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
            display.show(&mut timer, default_board, 100);
        }
    }
}

#![no_main]
#![no_std]

// #[macro_use]
extern crate alloc;

use alloc::vec::{Vec, self};
use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
// use embedded_alloc::LlffHeap as Heap;
use embedded_alloc::Heap;
use core::panic::PanicInfo;
use embedded_hal as _;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::Timer,
    hal::prelude::*
};
// Two-Level Segregated Fit Heap allocator (feature = "tlsf")
// use embedded_alloc::TlsfHeap as Heap;

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


    let new_board = [[0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [1, 0, 1, 0, 1],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0]
    ];

    let mut default_board = [[0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0]
    ];

    let mut display = Display::new(board.display_pins);
    let mut snake = Snake::new();

    loop {
        for snake_bits in &snake.body_position {
            default_board[snake_bits.0][snake_bits.1] = 1;
            display.show(&mut timer, default_board, 100)
        }
        if let Ok(true) = board.buttons.button_b.is_high() {
            snake._move(Direction::Down)
        }

        // display.show(&mut timer, new_board, 300);
        // display.show(&mut timer, default_board, 300);
        // row1.set_low().unwrap();
        // rprintln!("Dark!");
        // row1.set_high().unwrap();
        // rprintln!("Light!");
    }
}

// #[panic_handler]
// fn panic(_: &PanicInfo) -> ! {
//     loop {}
// }


enum Direction {
    Down,
    Right,
}

struct Snake {
    body_length: u8,
    body_position: Vec<(usize, usize)>,
}

impl Snake {
    fn _move(&mut self, move_direction: Direction) {
        let current_head = self.body_position.first().unwrap();
        let mut new_head = (0, 0);
        match move_direction {
            Direction::Down =>  { 
                if current_head.1 == 4 { 
                    new_head.1 = 0;
                    self.body_position.insert(0, new_head)
                } else {
                    if current_head.1 == 0 {
                        new_head.1 = current_head.1 - 1;
                        self.body_position.insert(0, new_head)  
                    } 
                    self.body_position.insert(0, new_head)  
                }
            },
            Direction::Right => { 

                new_head.1 = current_head.1 + 1

            } 
        }
    }

    fn new() -> Self {
        let mut body_pos: Vec<(usize, usize)> = Vec::new();
        body_pos = [(2, 2), (2, 3)].to_vec();
        Self { body_length: 2, body_position: body_pos.to_vec() }
    }
}

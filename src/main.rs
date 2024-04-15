
#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

use microbit::{
    board::Board,
    hal::{
        prelude::{OutputPin, PinState, _embedded_hal_blocking_delay_DelayMs},
        timer::Timer,
    },
};



#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello, world!");

    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let blink_delay_ms = 300_u32;
    
    board.display_pins.col3.set_state(PinState::Low).unwrap();
    board.display_pins.row3.set_state(PinState::High).unwrap();

    loop {
        board.display_pins.row3.set_state(PinState::High).unwrap();
        timer.delay_ms(blink_delay_ms);
        board.display_pins.row3.set_state(PinState::Low).unwrap();
        timer.delay_ms(blink_delay_ms);
        // rprintln!("Echo...");
        // for _ in 0..100_000 {
        //     nop();
        // }
    }
}

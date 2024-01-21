#![no_std]
#![no_main]

use nucleo_f446re as _;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

fn delay(count: u32) {
    for _ in 0..count {
        cortex_m::asm::nop();
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();
    loop {
        rprintln!("Hello, world!");
        delay(150_000); // dev build, about 1 second delay
        //delay(2_500_000); // release build, about 1 second delay
    }
}

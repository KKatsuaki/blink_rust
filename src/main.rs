#![no_std]
#![no_main]
#![feature(asm, global_asm)]

mod boot;
mod gpio;
use gpio::*;

const MMIO_BASE: usize = 0xFE00_0000;

/// # This is the function loaded by _rust_start in boot.rs
/// see the code in boot.rs
fn main() -> ! {
    // set gpio pin
    let mut led = gpio::GpioPin::new(21);
    led.set_function(GpioFunction::OUTPUT);

    let mut dur;
    loop {
        dur = 30000;
        while dur > 5000 {
            led.set_high();
            busy_wait(dur);
            led.set_low();
            busy_wait(dur);
            dur -= 500;
        }
        led.set_low();
    }
}

/// busy wait function
fn busy_wait(time: usize) {
    for _ in 0..time {
        unsafe {
            asm!("");
        }
    }
}
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#![no_std]
#![no_main]
#![allow(dead_code, unused_imports)]

pub extern crate ch58x;
pub extern crate embedded_hal as hal;
pub extern crate riscv;

mod gpio;

use embassy_executor::Spawner;
use hal::digital::{InputPin, OutputPin};

use crate::gpio::GpioExt;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let peripherals = ch58x::Peripherals::take().unwrap();

    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#![no_std]
#![no_main]

pub extern crate ch58x as pac;
pub extern crate embedded_hal as hal;
pub extern crate riscv;

mod pfic;
mod sys;
mod sysclk;

use embassy_executor::Spawner;

use crate::sys::{Config, SysExt};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let peripherals = pac::Peripherals::take().unwrap();

    let sys = peripherals.sys.set(Config::pll(8));
    sysclk::init(peripherals.systick, &sys, &peripherals.pfic);

    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#![no_std]
#![allow(mismatched_lifetime_syntaxes, non_camel_case_types)]

mod critical_section;
mod generated;
mod generic;
pub mod register;

pub use generated::*;
pub use generic::*;

#[unsafe(no_mangle)]
fn __post_init(_a0: usize) {
    // Pipeline Control Bit && Dynamic Prediction Control
    unsafe { core::arch::asm!("csrw 0xBC0, {}", in(reg) 0x1F) }

    // Enable nested interrupts and hardware stack push function
    unsafe {
        register::intsyscr::write({
            let mut value = register::intsyscr::Intsyscr::from_bits(0);
            value.set_hwstken(true);
            value.set_inesten(true);
            value
        })
    };
}

#![no_std]
#![no_main]
// #![deny(warnings)] // TODO: Enable after full OS impl

mod entry;
mod param;
mod riscv;
mod start;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe { core::arch::asm!("wfi") }
    }
}

// start() jumps here in supervisor mode on all CPUs.
#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    loop {
        unsafe { core::arch::asm!("wfi") }
    }
}

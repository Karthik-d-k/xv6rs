#![no_std]
#![no_main]
#![deny(warnings)]

mod entry;
mod param;
mod start;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe { core::arch::asm!("wfi") }
    }
}

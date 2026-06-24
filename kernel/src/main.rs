#![no_std]
#![no_main]
// #![deny(warnings)] // TODO: Enable after full OS impl

mod entry;
mod memlayout;
mod param;
mod print;
mod riscv;
mod start;
mod uart;

use crate::riscv::r_tp;
use crate::uart::uartinit;
use core::panic::PanicInfo;
use core::sync::atomic::{AtomicBool, Ordering};

static STARTED: AtomicBool = AtomicBool::new(false);

// start() jumps here in supervisor mode on all CPUs.
#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    if r_tp() == 0 {
        uartinit();
        println!("xv6 kernel is booting");
        println!("Scheduler is not implemented yet !!");
        STARTED.store(true, Ordering::Release);
    } else {
        while !STARTED.load(Ordering::Acquire) {}
    }

    loop {
        unsafe { core::arch::asm!("wfi") }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe { core::arch::asm!("wfi") }
    }
}

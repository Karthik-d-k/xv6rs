// Ref: https://github.com/mit-pdos/xv6-riscv/blob/riscv/kernel/start.c

use crate::param::NCPU;

// entry.rs needs one stack per CPU.
#[repr(C, align(16))]
struct Stack([u8; 4096 * NCPU]);

#[unsafe(no_mangle)]
static mut stack0: Stack = Stack([0; 4096 * NCPU]);

// entry.rs jumps here in machine mode on stack0.
#[unsafe(no_mangle)]
pub extern "C" fn start() -> ! {
    loop {
        // TODO: Placeholder function for now, replace with actual kernel code later.
        unsafe { core::arch::asm!("wfi") }
    }
}

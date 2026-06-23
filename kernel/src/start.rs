// Ref: https://github.com/Karthik-d-k/xv6-riscv/blob/riscv/kernel/start.c

use crate::main;
use crate::param::NCPU;
use crate::riscv::*;
use core::arch::asm;

// entry.rs needs one stack per CPU.
// 16-byte Alignment is enforced by kernel.x
#[unsafe(no_mangle)]
#[unsafe(link_section = ".bss.stack0")]
static mut stack0: [u8; 4096 * NCPU] = [0; 4096 * NCPU];

// entry.rs jumps here in machine mode on stack0.
#[unsafe(no_mangle)]
pub extern "C" fn start() -> ! {
    // set M Previous Privilege mode to Supervisor, for mret.
    let mut x: u64 = r_mstatus();
    x &= !MSTATUS_MPP_MASK;
    x |= MSTATUS_MPP_S;
    w_mstatus(x);

    // set M Exception Program Counter to main, for mret.
    // requires gcc -mcmodel=medany
    w_mepc(main as *const () as u64);

    // disable paging for now.
    w_satp(0);

    // delegate all interrupts and exceptions to supervisor mode.
    w_medeleg(0xffff);
    w_mideleg(0xffff);
    w_sie(r_sie() | SIE_SEIE | SIE_STIE);

    // configure Physical Memory Protection to give supervisor mode
    // access to all of physical memory.
    w_pmpaddr0(0x3fffffffffffff);
    w_pmpcfg0(0xf);

    // ask for clock interrupts.
    timerinit();

    // keep each CPU's hartid in its tp register, for cpuid().
    let id: u64 = r_mhartid();
    w_tp(id);

    // switch to supervisor mode and jump to main().
    unsafe {
        asm!("mret", options(noreturn));
    };
}

// ask each hart to generate timer interrupts.
fn timerinit() {
    // enable supervisor-mode timer interrupts.
    w_mie(r_mie() | MIE_STIE);

    // enable the sstc extension (i.e. stimecmp).
    w_menvcfg(r_menvcfg() | (1 << 63));

    // allow supervisor to use stimecmp and time.
    w_mcounteren(r_mcounteren() | 2);

    // ask for the very first timer interrupt.
    w_stimecmp(r_time() + 1000000);
}

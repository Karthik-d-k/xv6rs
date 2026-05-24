// Ref: https://github.com/mit-pdos/xv6-riscv/blob/riscv/kernel/entry.S

use core::arch::global_asm;

global_asm!(
    "
    # qemu -kernel loads the kernel at 0x80000000
    # and causes each hart (i.e. CPU) to jump there.
    # kernel.x causes the following code to
    # be placed at 0x80000000.

    .section .text.entry
    .global _entry
    _entry:
            # set up a stack for Rust.
            # stack0 is declared in start.rs,
            # with a 4096-byte stack per CPU.
            # sp = stack0 + ((hartid + 1) * 4096)
            la sp, stack0
            csrr a1, mhartid
            addi a1, a1, 1
            slli a0, a1, 12 # `li a0, 4096`; `mul a0, a0, a1`;
            add sp, sp, a0
            # jump to start() in start.rs
            call start
    ",
);

// Notes:
// -----
//
// `.option arch, +m` could be used to solve LLVM bug...
// Refer: https://github.com/rust-embedded/riscv/blob/fec25a239b707eedf58e6ade7078af17d6482190/riscv-target-parser/src/lib.rs#L205
// Refer: https://sourceware.org/binutils/docs-2.35/as/RISC_002dV_002dDirectives.html
// so that `mul a0, a0, a1` works without the following issue...
// error: mul instruction requires the following: 'Zmmul' (Integer Multiplication)

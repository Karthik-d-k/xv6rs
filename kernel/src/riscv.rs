// Ref: https://github.com/mit-pdos/xv6-riscv/blob/riscv/kernel/riscv.h

use core::arch::asm;

macro_rules! read_csr {
    ($fn_name:ident, $csr:literal) => {
        #[inline]
        pub fn $fn_name() -> u64 {
            unsafe {
                let x: u64;
                asm!(concat!("csrr {x}, ", $csr), x = out(reg) x);
                x
            }
        }
    };
}

macro_rules! write_csr {
    ($fn_name:ident, $csr:literal) => {
        #[inline]
        pub fn $fn_name(x: u64) {
            unsafe {
                asm!(concat!("csrw ", $csr, ", {x}"), x = in(reg) x);
            }
        }
    };
}

// which hart (core) is this?
read_csr!(r_mhartid, "mhartid");

// Machine Status Register, mstatus

pub const MSTATUS_MPP_MASK: u64 = 3 << 11; // previous mode.
pub const MSTATUS_MPP_M: u64 = 3 << 11;
pub const MSTATUS_MPP_S: u64 = 1 << 11;
pub const MSTATUS_MPP_U: u64 = 0 << 11;

read_csr!(r_mstatus, "mstatus");
write_csr!(w_mstatus, "mstatus");

// machine exception program counter, holds the
// instruction address to which a return from
// exception will go.
write_csr!(w_mepc, "mepc");

// Supervisor Status Register, sstatus

pub const SSTATUS_SPP: u64 = 1 << 8; // Previous mode, 1=Supervisor, 0=User
pub const SSTATUS_SPIE: u64 = 1 << 5; // Supervisor Previous Interrupt Enable
pub const SSTATUS_UPIE: u64 = 1 << 4; // User Previous Interrupt Enable
pub const SSTATUS_SIE: u64 = 1 << 1; // Supervisor Interrupt Enable
pub const SSTATUS_UIE: u64 = 1 << 0; // User Interrupt Enable

read_csr!(r_sstatus, "sstatus");
write_csr!(w_sstatus, "sstatus");
read_csr!(r_sip, "sip");
write_csr!(w_sip, "sip");

// Supervisor Interrupt Enable
pub const SIE_SEIE: u64 = 1 << 9; // external
pub const SIE_STIE: u64 = 1 << 5; // timer

read_csr!(r_sie, "sie");
write_csr!(w_sie, "sie");

// Machine-mode Interrupt Enable
pub const MIE_STIE: u64 = 1 << 5; // supervisor timer

read_csr!(r_mie, "mie");
write_csr!(w_mie, "mie");

// supervisor exception program counter, holds the
// instruction address to which a return from
// exception will go.
read_csr!(r_sepc, "sepc");
write_csr!(w_sepc, "sepc");

// Machine Exception Delegation
read_csr!(r_medeleg, "medeleg");
write_csr!(w_medeleg, "medeleg");

// Machine Interrupt Delegation
read_csr!(r_mideleg, "mideleg");
write_csr!(w_mideleg, "mideleg");

// Supervisor Trap-Vector Base Address
// low two bits are mode.
read_csr!(r_stvec, "stvec");
write_csr!(w_stvec, "stvec");

// Supervisor Timer Comparison Register
read_csr!(r_stimecmp, "stimecmp");
write_csr!(w_stimecmp, "stimecmp");

// Machine Environment Configuration Register
read_csr!(r_menvcfg, "menvcfg");
write_csr!(w_menvcfg, "menvcfg");

// Physical Memory Protection
write_csr!(w_pmpcfg0, "pmpcfg0");
write_csr!(w_pmpaddr0, "pmpaddr0");

// use riscv's sv39 page table scheme.
pub const SATP_SV39: u64 = 8 << 60;

#[allow(non_snake_case)]
pub fn MAKE_SATP(pagetable: Pagetable) -> u64 {
    SATP_SV39 | (pagetable as u64 >> 12)
}

// supervisor address translation and protection;
// holds the address of the page table.
read_csr!(r_satp, "satp");
write_csr!(w_satp, "satp");

// Supervisor Trap Cause
read_csr!(r_scause, "scause");

// Supervisor Trap Value
read_csr!(r_stval, "stval");

// Machine-mode Counter-Enable
read_csr!(r_mcounteren, "mcounteren");
write_csr!(w_mcounteren, "mcounteren");

// machine-mode cycle counter
read_csr!(r_time, "time");

// enable device interrupts
#[inline]
pub fn intr_on() {
    w_sstatus(r_sstatus() | SSTATUS_SIE);
}

// disable device interrupts
#[inline]
pub fn intr_off() {
    w_sstatus(r_sstatus() & !SSTATUS_SIE);
}

// are device interrupts enabled?
#[inline]
pub fn intr_get() -> bool {
    (r_sstatus() & SSTATUS_SIE) != 0
}

#[inline]
pub fn r_sp() -> u64 {
    unsafe {
        let x: u64;
        asm!("mv {x}, sp", x = out(reg) x);
        x
    }
}

// read and write tp, the thread pointer, which xv6 uses to hold
// this core's hartid (core number), the index into cpus[].
#[inline]
pub fn r_tp() -> u64 {
    unsafe {
        let x: u64;
        asm!("mv {x}, tp", x = out(reg) x);
        x
    }
}

#[inline]
pub fn w_tp(x: u64) {
    unsafe {
        asm!("mv tp, {x}", x = in(reg) x);
    }
}

#[inline]
pub fn r_ra() -> u64 {
    unsafe {
        let x: u64;
        asm!("mv {x}, ra", x = out(reg) x);
        x
    }
}

// flush the TLB.
#[inline]
pub fn sfence_vma() {
    // the zero, zero means flush all TLB entries.
    unsafe {
        asm!("sfence.vma zero, zero");
    }
}

pub type Pte = u64;
pub type Pagetable = *mut u64; // 512 PTEs // TODO: Maybe better type would be `*mut [u64; 512]` ??

pub const PGSIZE: u64 = 4096; // bytes per page
pub const PGSHIFT: u64 = 12; // bits of offset within a page

#[allow(non_snake_case)]
pub const fn PGROUNDUP(sz: u64) -> u64 {
    (sz + PGSIZE - 1) & !(PGSIZE - 1)
}

#[allow(non_snake_case)]
pub const fn PGROUNDDOWN(a: u64) -> u64 {
    a & !(PGSIZE - 1)
}

pub const PTE_V: u64 = 1 << 0; // valid
pub const PTE_R: u64 = 1 << 1;
pub const PTE_W: u64 = 1 << 2;
pub const PTE_X: u64 = 1 << 3;
pub const PTE_U: u64 = 1 << 4; // user can access

// shift a physical address to the right place for a PTE.
#[allow(non_snake_case)]
pub const fn PA2PTE(pa: u64) -> u64 {
    (pa >> 12) << 10
}

#[allow(non_snake_case)]
pub const fn PTE2PA(pte: u64) -> u64 {
    (pte >> 10) << 12
}

#[allow(non_snake_case)]
pub const fn PTE_FLAGS(pte: u64) -> u64 {
    pte & 0x3FF
}

// extract the three 9-bit page table indices from a virtual address.
pub const PXMASK: u64 = 0x1FF; // 9 bits

#[allow(non_snake_case)]
pub const fn PXSHIFT(level: u64) -> u64 {
    PGSHIFT + (9 * level)
}

#[allow(non_snake_case)]
pub const fn PX(level: u64, va: u64) -> u64 {
    (va >> PXSHIFT(level)) & PXMASK
}

// one beyond the highest possible virtual address.
// MAXVA is actually one bit less than the max allowed by
// Sv39, to avoid having to sign-extend virtual addresses
// that have the high bit set.
pub const MAXVA: u64 = 1 << (9 + 9 + 9 + 12 - 1);

// Ref: https://github.com/Karthik-d-k/xv6-riscv/blob/riscv/kernel/uart.c

// low-level driver for 16550a UART.

use crate::memlayout::UART0;
use core::ptr::{read_volatile, write_volatile};

#[allow(non_snake_case)]
fn ReadReg(reg: u64) -> u8 {
    unsafe { read_volatile((UART0 + reg) as *const u8) }
}

#[allow(non_snake_case)]
fn WriteReg(reg: u64, val: u8) {
    unsafe { write_volatile((UART0 + reg) as *mut u8, val) }
}

// the UART control registers.
// some have different meanings for read vs write.
// see http://byterunner.com/16550.html

const RHR: u64 = 0; // receive holding register (for input bytes)
const THR: u64 = 0; // transmit holding register (for output bytes)
const DLAB_LSB: u64 = 0; // Divisor Latch (LSB) when DLAB bit (LCR_BAUD_LATCH) is set
const DLAB_MSB: u64 = 0; // Divisor Latch (MSB) when DLAB bit (LCR_BAUD_LATCH) is set

const IER: u64 = 1; // interrupt enable register
const IER_RX_ENABLE: u8 = 1 << 0;
const IER_TX_ENABLE: u8 = 1 << 1;

const FCR: u64 = 2; // FIFO control register
const FCR_FIFO_ENABLE: u8 = 1 << 0;
const FCR_FIFO_CLEAR: u8 = 3 << 1; // clear the content of the two FIFOs

const ISR: u64 = 2; // interrupt status register

const LCR: u64 = 3; // line control register
const LCR_EIGHT_BITS: u8 = 3 << 0;
const LCR_BAUD_LATCH: u8 = 1 << 7; // special mode to set baud rate

const LSR: u64 = 5; // line status register
const LSR_RX_READY: u8 = 1 << 0; // input is waiting to be read from RHR
const LSR_TX_IDLE: u8 = 1 << 5; // THR can accept another character to send

pub fn uartinit() {
    // disable interrupts.
    WriteReg(IER, 0x00);

    // special mode to set baud rate.
    WriteReg(LCR, LCR_BAUD_LATCH);

    // LSB for baud rate of 38.4K.
    WriteReg(DLAB_LSB, 0x03);

    // MSB for baud rate of 38.4K.
    WriteReg(DLAB_MSB, 0x00);

    // leave set-baud mode,
    // and set word length to 8 bits, no parity.
    WriteReg(LCR, LCR_EIGHT_BITS);

    // reset and enable FIFOs.
    WriteReg(FCR, FCR_FIFO_ENABLE | FCR_FIFO_CLEAR);

    // enable transmit and receive interrupts.
    WriteReg(IER, IER_TX_ENABLE | IER_RX_ENABLE);

    // initlock(&tx_lock, "uart"); // TODO: Implement locks and enable this
}

// write a byte to the uart without using
// interrupts, for use by kernel printf() and
// to echo characters. it spins waiting for the uart's
// output register to be empty.
// TODO: Add panics later
pub fn uartputc_sync(c: u8) {
    // wait for UART to set Transmit Holding Empty in LSR.
    while (ReadReg(LSR) & LSR_TX_IDLE) == 0 {}
    WriteReg(THR, c);
}

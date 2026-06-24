// Ref: https://github.com/Karthik-d-k/xv6-riscv/blob/riscv/kernel/printf.c

use crate::uart::uartputc_sync;
use core::fmt::{self, Write};

struct UartWriter;

impl Write for UartWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for &c in s.as_bytes() {
            uartputc_sync(c);
        }
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    let _ = UartWriter.write_fmt(args);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::print::_print(format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! println {
    () => {{ $crate::print::_print(format_args!("\n")); }};
    ($($arg:tt)*) => {{
        $crate::print::_print(format_args!($($arg)*));
        $crate::print::_print(format_args!("\n"));
    }};
}

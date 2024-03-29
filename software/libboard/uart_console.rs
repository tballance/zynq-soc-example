
use core::fmt;

pub struct Console;

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        use uart;

        for c in s.bytes() {
            unsafe { uart::write(c) }
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        write!($crate::uart_console::Console, $($arg)*).unwrap()
    })
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

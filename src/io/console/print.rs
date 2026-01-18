use core::fmt::Write;

use crate::io::console::tty::TTY_TABLE;

pub fn print(args: core::fmt::Arguments) {
    // use core::fmt::Write;

    let mut ttys = TTY_TABLE.lock();
    ttys.active().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($args:expr) => {
        $crate::io::console::print::print($args);
    };
}

#[macro_export]
macro_rules! println {
    () => {
        ($crate::print!(core::format_args!("\n")))
    };
    ($($arg:tt)*) => {{
        $crate::print!(core::format_args!($($arg)*));
        $crate::print!(core::format_args!("\n"));
    }};
}

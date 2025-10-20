use core::{cell::UnsafeCell, fmt::Write};

use crate::io::console::{Console, writer::Writer};

pub struct GlobalConsole {
    pub console: UnsafeCell<Console<Writer>>,
}

unsafe impl Sync for GlobalConsole {}

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
pub static CONSOLE: GlobalConsole = GlobalConsole {
    console: UnsafeCell::new(Console::new(Writer::new(VGA_BUFFER))),
};

pub fn print(args: core::fmt::Arguments) {
    // use core::fmt::Write;

    let console = unsafe { &mut *CONSOLE.console.get() };
    console.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($args:expr) => {
        $crate::io::print::print($args);
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

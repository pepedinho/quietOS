use core::{
    cell::UnsafeCell,
    fmt::{self, Write},
};

use crate::io::console::Console;

pub struct GlobalConsole {
    console: UnsafeCell<Console>,
}

unsafe impl Sync for GlobalConsole {}

static CONSOLE: GlobalConsole = GlobalConsole {
    console: UnsafeCell::new(Console::new()),
};

pub fn print(args: core::fmt::Arguments) {
    // use core::fmt::Write;

    let console = unsafe { &mut *CONSOLE.console.get() };
    fmt::write(console, args).unwrap();
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

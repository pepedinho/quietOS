#![no_std]
#![no_main]

use core::panic::PanicInfo;

use quiet::{
    io::{console::Color, print::CONSOLE},
    println,
};

#[unsafe(no_mangle)]
pub extern "C" fn _entrypoint() -> ! {
    println!(
        "Welcome to {}Quiet Os\n{}42{}",
        Color::Yellow,
        Color::BRed,
        Color::White
    );
    println!("\x1B[34;41mtests\x1B[0m");
    let console = unsafe { &mut *CONSOLE.console.get() };
    console.read_stdin();
    // #[allow(clippy::empty_loop)]
    // loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    if let Some(location) = _info.location() {
        println!(
            "panic occurred in file '{}' at line {}",
            location.file(),
            location.line(),
        );
    }
    println!("panicked: {}", _info.message());
    loop {}
}

#![no_std]
#![no_main]

use core::panic::PanicInfo;

use quiet::{io::console::Color, println};

// const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

#[unsafe(no_mangle)]
pub extern "C" fn _entrypoint() -> ! {
    println!(
        "Welcome to {}Quiet Os\n{}42{}",
        Color::Yellow,
        Color::BRed,
        Color::White
    );
    #[allow(clippy::empty_loop)]
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

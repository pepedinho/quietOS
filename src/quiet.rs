#![no_std]
#![no_main]

use core::panic::PanicInfo;

use quiet::{io::console::Color, println};

// const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

#[unsafe(no_mangle)]
pub extern "C" fn _entrypoint() -> ! {
    println!("Welcome to \x1B[33mQuiet Os\n\x1B[91m42\x1B[0m.");
    println!(
        "Welcome to {}Quiet Os\n{}42{}",
        Color::Yellow,
        Color::BRed,
        Color::White
    );
    // let mut console = Console::new();
    // console
    //     .write_str("Quiet Os \x1B[31m42\x1B[0m.\nwelcome")
    //     .ok();
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

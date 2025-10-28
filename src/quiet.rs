#![no_std]
#![no_main]

use core::{panic::PanicInfo, sync::atomic::Ordering};

use quiet::{
    io::console::{colors::Color, print::TTY_TABLE},
    println,
    sync::mutex::PANIC_IN_PROGRESS,
};

#[unsafe(no_mangle)]
pub extern "C" fn _entrypoint() -> ! {
    println!("+-----------------------------------------+");
    println!(
        "|        Welcome to {}Quiet Os {}42{}           |",
        Color::Yellow,
        Color::BRed,
        Color::White
    );
    println!("+-----------------------------------------+");
    println!("\x1B[34;41mtests\x1B[0m");
    let mut console = TTY_TABLE.lock();

    loop {
        console.read_once_in_active();
    }
    // console.read_in_active();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    PANIC_IN_PROGRESS.store(true, Ordering::SeqCst);
    println!("");
    println!("{_info}");
    loop {}
}

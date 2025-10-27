#![no_std]
#![no_main]

use core::panic::PanicInfo;

use quiet::{
    io::{
        AlignedStack, VGA,
        console::{colors::Color, print::TTY_TABLE},
        keyborad::{
            SCANCODE_BUF,
            // idt::init_idt,
            pic::{PIC1_DATA, PIC2_DATA},
            pop_scancode,
            scancode_to_ascii,
        },
    },
    println,
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

    loop {
        if let Some(sc) = pop_scancode() {
            if let Some(ch) = scancode_to_ascii(sc) {
                let mut console = TTY_TABLE.lock();
                console.interpret_byte_from_keyboard(&ch);
            }
        } else {
            unsafe { core::arch::asm!("hlt", options(nomem, nostack)) }
        }
    }
    // console.read_in_active();
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

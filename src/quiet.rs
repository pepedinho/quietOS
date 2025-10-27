#![no_std]
#![no_main]

use core::panic::PanicInfo;

use quiet::{
    io::{
        console::{colors::Color, print::TTY_TABLE},
        keyborad::{SCANCODE_BUF, idt::init_idt, pop_scancode, scancode_to_ascii},
    },
    println,
};

#[unsafe(no_mangle)]
pub extern "C" fn _entrypoint() -> ! {
    init_idt();
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
            println!("oui");
            if let Some(ch) = scancode_to_ascii(sc) {
                let mut console = TTY_TABLE.lock();
                console.interpret_byte_from_keyboard(&ch);
            }
        } else {
            println!("oui");
            let oui = &raw mut SCANCODE_BUF;
            unsafe {
                let a = *oui;
                println!("{:?}", a);
            }
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

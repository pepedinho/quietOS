#![no_std]
#![no_main]

use core::panic::PanicInfo;

use quiet::{
    io::{
        console::colors::Color,
        keyborad::isr::{keyboard_handler, register_irq_handler, stack_usage},
    },
    println,
};

#[unsafe(no_mangle)]
pub extern "C" fn _entrypoint() -> ! {
    println!("stack usage => {}", stack_usage());
    register_irq_handler(0x21, keyboard_handler);
    println!("+-----------------------------------------+");
    println!(
        "|        Welcome to {}Quiet Os {}42{}           |",
        Color::Yellow,
        Color::BRed,
        Color::White
    );
    println!("+-----------------------------------------+");
    println!("\x1B[34;41mtests\x1B[0m");
    let _a = 10;
    core::hint::black_box(_a);
    // println!("stack usage => {}", stack_usage());

    loop {
        unsafe { core::arch::asm!("hlt", options(nomem, nostack)) }
    }
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

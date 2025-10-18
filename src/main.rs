#![no_std]
#![no_main]

use core::panic::PanicInfo;

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

#[unsafe(no_mangle)]
pub extern "C" fn _entrypoint() -> ! {
    let s = b"42";
    unsafe {
        for (i, &byte) in s.iter().enumerate() {
            *VGA_BUFFER.add(i * 2) = byte;
            *VGA_BUFFER.add(i * 2 + 1) = 0x0f;
        }
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

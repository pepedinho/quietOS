use crate::{
    io::{VGA, console::print::TTY_TABLE, keyborad::scancode_to_ascii},
    println,
};

pub type IrqHandler = fn();

#[used]
static mut IRQ_HANDLERS: [Option<IrqHandler>; 256] = [None; 256];

pub fn register_irq_handler(irq: u8, handler: IrqHandler) {
    println!("stack usage => {}", stack_usage());
    unsafe { IRQ_HANDLERS[irq as usize] = Some(handler) }
    unsafe {
        println!(
            "handler[{:#x}] set exec addr => {:#x}",
            irq,
            IRQ_HANDLERS[irq as usize].unwrap() as usize
        );
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn irq_dispatch(irq: u8) {
    unsafe {
        // println!(
        //     "debug: irq: {irq}: handler[{irq}] => {:#?}",
        //     IRQ_HANDLERS[irq as usize]
        // );
        if let Some(handler) = IRQ_HANDLERS[irq as usize] {
            handler();
        }
    }
}

pub fn keyboard_handler() {
    // println!("oui");
    unsafe {
        let scancode = VGA::inb(0x60);

        if let Some(ch) = scancode_to_ascii(scancode) {
            let mut console = TTY_TABLE.lock();
            console.interpret_byte_from_keyboard(&ch);
        }
    }
}

unsafe extern "C" {
    static stack_top: u8;
}

#[inline(always)]
pub fn stack_usage() -> usize {
    let esp: usize;
    unsafe {
        core::arch::asm!("mov {}, esp", out(reg) esp);
        let stack_top_addr = &stack_top as *const u8 as usize;
        stack_top_addr - esp
    }
}

use crate::{
    io::{
        VGA,
        keyborad::{
            isr::keyboard_stub,
            pic::{PIC1_DATA, PIC2_DATA, remap_pic},
        },
    },
    println,
};

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct IdtEntry32 {
    offset_low: u16,
    selector: u16,
    zero: u8,
    flags: u8,
    offset_high: u16,
}

const PIC1_OFFSET: u8 = 0x20;
const PIC2_OFFSET: u8 = 0x28;

pub fn init_idt() {
    unsafe {
        remap_pic(PIC1_OFFSET, PIC2_OFFSET);
        VGA::outb(PIC1_DATA, 0xFD);
        VGA::outb(PIC2_DATA, 0xFF);
        set_idt_entry((PIC1_OFFSET + 1) as usize, keyboard_stub);
        load_idt();
        core::arch::asm!("sti", options(nomem, nostack, preserves_flags));
        println!("IDT & PIC OK");
    }
}

impl IdtEntry32 {
    const fn missing() -> Self {
        IdtEntry32 {
            offset_low: 0,
            selector: 0,
            zero: 0,
            flags: 0,
            offset_high: 0,
        }
    }

    fn set_handler(&mut self, handler: unsafe extern "C" fn()) {
        let addr = handler as u32;

        self.offset_low = (addr & 0xFFFF) as u16;
        self.selector = 0x08;
        self.zero = 0;
        self.flags = 0x8E;
        self.offset_high = ((addr >> 16) & 0xFFFF) as u16;
    }
}

#[repr(C, packed)]
struct IdtPtr {
    limit: u16,
    base: u32,
}

static mut IDT: [IdtEntry32; 256] = [IdtEntry32::missing(); 256];

pub unsafe fn load_idt() {
    let idt_ptr = IdtPtr {
        limit: (core::mem::size_of::<[IdtEntry32; 256]>() - 1) as u16,
        base: &raw const IDT as *const _ as u32,
    };

    unsafe {
        core::arch::asm!(
            "lidt [{}]", in(reg) &idt_ptr,
            options(readonly, nostack)
        )
    }
}

pub unsafe fn set_idt_entry(vector: usize, handler: unsafe extern "C" fn()) {
    unsafe {
        IDT[vector] = IdtEntry32::missing();
        IDT[vector].set_handler(handler);
    }
}

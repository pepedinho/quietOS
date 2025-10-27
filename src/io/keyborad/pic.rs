use crate::io::VGA;

const PIC1_COMMAND: u16 = 0x20;
const PIC2_COMMAND: u16 = 0xA0;
pub const PIC1_DATA: u16 = 0x21;
pub const PIC2_DATA: u16 = 0xA1;

const ICW1_INIT: u8 = 0x10;
const ICW1_ICW4: u8 = 0x01;
const ICW4_8086: u8 = 0x01;

// PIC remapping

pub unsafe fn remap_pic(offset1: u8, offset2: u8) {
    unsafe {
        let a1 = VGA::inb(PIC1_DATA);
        let a2 = VGA::inb(PIC2_DATA);

        VGA::outb(PIC1_COMMAND, ICW1_INIT | ICW1_ICW4);
        VGA::io_wait();
        VGA::outb(PIC2_COMMAND, ICW1_INIT | ICW1_ICW4);
        VGA::io_wait();

        VGA::outb(PIC1_DATA, offset1);
        VGA::io_wait();
        VGA::outb(PIC2_DATA, offset2);
        VGA::io_wait();

        VGA::outb(PIC1_DATA, 0x04);
        VGA::io_wait();

        VGA::outb(PIC2_DATA, 0x02);
        VGA::io_wait();

        VGA::outb(PIC1_DATA, ICW4_8086);
        VGA::io_wait();
        VGA::outb(PIC2_DATA, ICW4_8086);
        VGA::io_wait();

        VGA::outb(PIC1_DATA, a1);
        VGA::outb(PIC2_DATA, a2);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn send_eoi(irq: u8) {
    unsafe {
        if irq >= 8 {
            VGA::outb(PIC2_COMMAND, 0x20);
        }
        VGA::outb(PIC1_COMMAND, 0x20);
    }
}

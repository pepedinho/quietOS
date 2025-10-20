use crate::io::{
    VGA, VGA_CMD_PORT, VGA_DATA_PORT, VGA_WIDTH,
    console::{Cell, Pos},
};

pub mod mocker;

pub struct Writer {
    buffer: *mut u8,
}

impl Writer {
    pub const fn new(buf: *mut u8) -> Self {
        Self { buffer: buf }
    }
}

pub trait WriterSoul {
    fn write_byte(&mut self, cell: &Cell, x: usize, y: usize);
    fn move_cursor(&mut self, pos: &Pos);
    fn enable_cursor(&mut self, start: u8, end: u8);
    fn disable_cursor(&mut self);
}

impl WriterSoul for Writer {
    fn write_byte(&mut self, cell: &Cell, x: usize, y: usize) {
        unsafe {
            *self.buffer.add(y * VGA_WIDTH * 2 + x * 2) = cell.byte;
            *self.buffer.add(y * VGA_WIDTH * 2 + x * 2 + 1) = cell.color.shift();
        }
    }

    //----------------
    // cursor functions
    // doc found here: https://wiki.osdev.org/Text_Mode_Cursor
    //----------------

    /// cursor range = 0-15
    fn enable_cursor(&mut self, cursor_start: u8, cursor_end: u8) {
        unsafe {
            VGA::outb(VGA_CMD_PORT, 0x0A);
            VGA::outb(
                VGA_DATA_PORT,
                (VGA::inb(VGA_DATA_PORT) & 0xC0) | cursor_start,
            );
            VGA::outb(VGA_CMD_PORT, 0x0B);
            VGA::outb(VGA_DATA_PORT, (VGA::inb(VGA_DATA_PORT) & 0xE0) | cursor_end);
        }
    }

    fn disable_cursor(&mut self) {
        unsafe {
            VGA::outb(VGA_CMD_PORT, 0x0A);
            VGA::outb(VGA_DATA_PORT, 0x20);
        }
    }

    fn move_cursor(&mut self, p: &Pos) {
        let pos = (p.y * VGA_WIDTH + p.x) as u16;
        unsafe {
            VGA::outb(VGA_CMD_PORT, 0x0F);
            VGA::outb(VGA_DATA_PORT, (pos & 0xFF) as u8);
            VGA::outb(VGA_CMD_PORT, 0x0E);
            VGA::outb(VGA_DATA_PORT, ((pos >> 8) & 0xFF) as u8);
        }
    }
}

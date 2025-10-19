use crate::io::console::{ColorPair, Pos};

pub mod console;
pub mod keyborad;
pub mod print;

const VGA_WIDTH: usize = 80;
#[allow(dead_code)]
const VGA_HEIGHT: usize = 25;
const VGA_CMD_PORT: u16 = 0x3D4;
const VGA_DATA_PORT: u16 = 0x3D5;

pub const WHITE: u8 = 0x0f;
pub const YELLOW: u8 = 0x0E;

pub struct Writer {
    buffer: *mut u8,
}

pub struct VGA {}

impl VGA {
    /// write byte from I/O port
    #[inline(always)]
    unsafe fn outb(port: u16, value: u8) {
        unsafe {
            core::arch::asm!(
                "out dx, al",
                in("dx") port,
                in("al") value,
                options(nomem, nostack, preserves_flags),
            );
        }
    }

    /// read byte from I/O port
    #[inline(always)]
    unsafe fn inb(port: u16) -> u8 {
        let mut value: u8;
        unsafe {
            core::arch::asm!(
                "in al, dx",
                in("dx") port,
                out("al") value,
                options(nomem, nostack, preserves_flags),
            );
        }
        value
    }
}

impl Writer {
    pub const fn new(buf: *mut u8) -> Self {
        Self { buffer: buf }
    }

    fn update_pos(&mut self, pos: &mut Pos) {
        pos.x += 1;
        if pos.x >= VGA_WIDTH {
            pos.x = 0;
            pos.y += 1;
        }
        self.move_cursor(pos);
    }

    pub fn put_byte(&mut self, byte: u8, pos: &mut Pos, color: &ColorPair) {
        unsafe {
            *self.buffer.add(pos.y * VGA_WIDTH * 2 + pos.x * 2) = byte;
            *self.buffer.add(pos.y * VGA_WIDTH * 2 + pos.x * 2 + 1) = color.shift();
        }
    }

    pub fn write_byte(&mut self, byte: u8, pos: &mut Pos, color: &ColorPair) {
        self.put_byte(byte, pos, color);
        self.update_pos(pos);
    }

    //----------------
    // cursor functions
    // doc found here: https://wiki.osdev.org/Text_Mode_Cursor
    //----------------

    /// cursor range = 0-15
    pub fn enable_cursor(&mut self, cursor_start: u8, cursor_end: u8) {
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

    pub fn disable_cursor(&mut self) {
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

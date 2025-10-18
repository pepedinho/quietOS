use crate::io::console::{Color, Pos};

pub mod console;
pub mod print;

const VGA_WIDTH: usize = 80;
#[allow(dead_code)]
const VGA_HEIGHT: usize = 25;

pub const WHITE: u8 = 0x0f;
pub const YELLOW: u8 = 0x0E;

pub struct Writer {
    buffer: *mut u8,
}

impl Writer {
    pub const fn new(buf: *mut u8) -> Self {
        Self { buffer: buf }
    }

    pub fn write_byte(&mut self, byte: u8, pos: &mut Pos, color: &Color) {
        unsafe {
            *self.buffer.add(pos.y * VGA_WIDTH * 2 + pos.x * 2) = byte;
            *self.buffer.add(pos.y * VGA_WIDTH * 2 + pos.x * 2 + 1) = color.as_vga();
        }
        pos.x += 1;
        if pos.x >= VGA_WIDTH {
            pos.x = 0;
            pos.y += 1;
        }
    }
}

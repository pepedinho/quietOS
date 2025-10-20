use crate::io::console::{Cell, writer::WriterSoul};

pub struct MockWriter {}

impl WriterSoul for MockWriter {
    fn write_byte(&mut self, _cell: &Cell, _x: usize, _y: usize) {}

    fn move_cursor(&mut self, _pos: &crate::io::console::Pos) {}

    fn disable_cursor(&mut self) {}
    fn enable_cursor(&mut self, _start: u8, _end: u8) {}
}

use crate::io::console::{Cell, Pos, writer::WriterSoul};

pub struct MockWriter {}

impl WriterSoul for MockWriter {
    fn write_byte(&mut self, _cell: &Cell, _x: usize, _y: usize) {}

    fn move_cursor(&mut self, _pos: &Pos, _offset: Option<usize>) {}

    fn disable_cursor(&mut self) {}
    fn enable_cursor(&mut self, _start: u8, _end: u8) {}
}

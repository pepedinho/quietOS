use crate::io::{
    VGA_WIDTH,
    console::{Cell, Pos, writer::WriterSoul},
};

pub struct MockWriter {
    pub last_pos: Option<u16>,
    pub underflow_detected: bool,
}

impl WriterSoul for MockWriter {
    fn write_byte(&mut self, _cell: &Cell, _x: usize, _y: usize) {}

    fn move_cursor(&mut self, pos: &Pos, offset: Option<usize>) {
        let ofs = offset.unwrap_or(0);
        if pos.y < ofs {
            self.underflow_detected = true;
            return;
        }

        let pos = ((pos.y - ofs) * VGA_WIDTH + pos.x) as u16;
        self.last_pos = Some(pos);
    }

    fn disable_cursor(&mut self) {}
    fn enable_cursor(&mut self, _start: u8, _end: u8) {}
}

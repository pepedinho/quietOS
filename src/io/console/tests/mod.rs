use crate::io::console::{Cell, Console, colors::ColorPair, writer::mocker::MockWriter};

pub mod ansi_csi_test;
pub mod buffer_test;
pub mod cursor_test;

impl Console<MockWriter> {
    pub fn fill_line(&mut self) {
        self.buffer[self.cursor.y].fill(Cell::new(b'A', ColorPair::default()));
    }
}

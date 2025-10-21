use crate::io::{VGA_WIDTH, console::Cell};

pub trait U8CellLen {
    fn cell_len(&self) -> usize;
}

impl<const N: usize> U8CellLen for [Cell; N] {
    fn cell_len(&self) -> usize {
        let len = self.iter().position(|e| e.byte == 0).unwrap_or(0);
        if len == VGA_WIDTH { VGA_WIDTH - 1 } else { len }
    }
}

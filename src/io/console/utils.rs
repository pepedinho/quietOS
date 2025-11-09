use crate::io::{VGA_WIDTH, console::Cell};

pub trait U8CellLen {
    fn cell_len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

impl<const N: usize> U8CellLen for [Cell; N] {
    fn cell_len(&self) -> usize {
        let len = self.iter().position(|e| e.byte == 0).unwrap_or(VGA_WIDTH);
        if len == VGA_WIDTH { VGA_WIDTH - 1 } else { len }
    }

    fn is_empty(&self) -> bool {
        self.cell_len() == 0
    }
}

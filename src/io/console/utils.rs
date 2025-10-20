use crate::io::console::Cell;

pub trait U8CellLen {
    fn cell_len(&self) -> usize;
}

impl<const N: usize> U8CellLen for [Cell; N] {
    fn cell_len(&self) -> usize {
        self.iter().position(|e| e.byte == 0).unwrap_or(N)
    }
}

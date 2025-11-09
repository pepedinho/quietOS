use crate::io::{
    VGA_HEIGHT, VGA_WIDTH,
    console::{
        colors::{Color, ColorPair},
        utils::U8CellLen,
        writer::WriterSoul,
    },
};
use core::fmt::Write;

const ERASE_BYTE: u8 = 0x00;
const CONSOLE_HISTORY: usize = 100;

pub mod colors;
pub mod print;
pub mod utils;
pub mod writer;

#[cfg(test)]
pub mod tests;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSI {
    None,
    Some(u8),
    Err,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State {
    Default,
    Esc,
    CSI(CSI),
}

#[derive(Default, Clone, Copy)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Self {
        Pos { x, y }
    }

    pub const fn blank() -> Self {
        Pos { x: 0, y: 0 }
    }

    fn inc(&mut self, offset: usize) -> bool {
        self.x += 1;
        if self.x >= VGA_WIDTH {
            self.x = 0;
            if self.y < CONSOLE_HISTORY - 1 {
                self.y += 1;
            }
            if self.y == offset + VGA_HEIGHT {
                return true;
            }
            return false;
        }
        false
    }
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub byte: u8,
    pub color: ColorPair,
}

impl Cell {
    pub const fn blank() -> Self {
        Cell {
            byte: 0,
            color: ColorPair {
                foreground: Color::White,
                background: Color::Black,
            },
        }
    }

    pub fn new(byte: u8, color: ColorPair) -> Self {
        Cell { byte, color }
    }

    pub fn is_empty(&self) -> bool {
        self.byte == 0
    }
}

pub struct Console<W: WriterSoul> {
    buffer: [[Cell; VGA_WIDTH]; CONSOLE_HISTORY],
    cursor: Pos,
    // the visible window start at [offset] & end at [VGA_HEIGHT + offset]
    offset: usize, // the index of the first visible line (all lines behind will be hidden)

    writer: W,
    color: ColorPair,
    state: State,
}

impl<W: WriterSoul> Console<W> {
    #[allow(clippy::new_without_default)]
    pub const fn new(writer: W) -> Self {
        Self {
            buffer: [[Cell::blank(); VGA_WIDTH]; CONSOLE_HISTORY],
            cursor: Pos::blank(),
            offset: 0,
            writer,
            state: State::Default,
            color: ColorPair {
                foreground: Color::White,
                background: Color::Black,
            },
        }
    }

    pub fn replace_byte(&mut self, byte: u8) {
        self.buffer[self.cursor.y][self.cursor.x] = Cell::new(byte, self.color);
    }

    pub fn store_byte(&mut self, byte: u8) {
        self.buffer[self.cursor.y][self.cursor.x] = Cell::new(byte, self.color);
        if self.cursor.inc(self.offset) {
            // if the line is full
            self.scroll_offset_down();
            self.flush();
        };
    }

    pub fn flush(&mut self) {
        for row in 0..VGA_HEIGHT {
            for col in 0..VGA_WIDTH {
                let ch = self.buffer[self.offset + row][col];
                self.write_byte(&ch, &Pos::new(col, row));
            }
        }
        self.move_cursor();
    }

    fn move_cursor(&mut self) {
        self.writer.move_cursor(&self.cursor, Some(self.offset));
    }

    pub fn write_byte(&mut self, cell: &Cell, pos: &Pos) {
        self.writer.write_byte(cell, pos.x, pos.y);
    }

    pub fn write_string(&mut self, s: &[u8]) {
        s.iter().for_each(|b| self.handle_byte(*b));
        self.flush();
    }

    fn nl(&mut self) {
        self.cursor.x = 0;
        if self.cursor.y < CONSOLE_HISTORY - 1 {
            self.cursor.y += 1;
        }
        if self.cursor.y >= VGA_HEIGHT {
            if self.cursor.y == VGA_HEIGHT + self.offset {
                self.scroll_offset_down();
            }
            self.flush();
        }
        self.writer.move_cursor(&self.cursor, Some(self.offset));
    }

    fn scroll_offset_down(&mut self) {
        if self.offset < CONSOLE_HISTORY - VGA_HEIGHT {
            self.offset += 1;
            if self.cursor.y < self.offset {
                self.cursor.y = self.offset;
            }
        }
    }

    fn scroll_offset_up(&mut self) {
        if self.cursor.y > 0 && self.offset > 0 {
            self.offset -= 1;
        }
    }

    fn cursor_right(&mut self) {
        if self.cursor.x + 1 > VGA_WIDTH
            || (self.cursor.x >= self.buffer[self.cursor.y].cell_len()
                && self.buffer[self.cursor.y].cell_len() != 0)
        {
            if self.cursor.y < CONSOLE_HISTORY && self.buffer[self.cursor.y + 1].cell_len() != 0 {
                self.cursor_down();
            }
        } else if !self.buffer[self.cursor.y][self.cursor.x].is_empty() {
            self.cursor.x += 1;
            self.writer.move_cursor(&self.cursor, Some(self.offset));
        }
    }

    fn cursor_left(&mut self) {
        if self.cursor.y > 0
            && self.cursor.x == 0
            && self.buffer[self.cursor.y - 1].cell_len() == VGA_WIDTH - 1
        {
            self.cursor_up();
        } else if self.cursor.x > 0 {
            self.cursor.x -= 1;
            self.writer.move_cursor(&self.cursor, Some(self.offset));
        }
    }

    fn cursor_up(&mut self) {
        if self.cursor.y > 0 {
            self.cursor.x = self.buffer[self.cursor.y - 1].cell_len();
            self.cursor.y -= 1;
            if self.cursor.y == self.offset {
                self.scroll_offset_up();
            }
            self.writer.move_cursor(&self.cursor, Some(self.offset));
        }
    }

    fn try_cursor_up(&mut self) {
        if self.cursor.y > 0
            && self.cursor.x == 0
            && self.buffer[self.cursor.y - 1].cell_len() == VGA_WIDTH - 1
        {
            self.cursor_up();
        }
    }

    fn cursor_down(&mut self) {
        if self.cursor.y + 1 >= CONSOLE_HISTORY {
            return;
        }
        if self.cursor.y < self.offset + VGA_HEIGHT - 1
            && !self.buffer[self.cursor.y + 1].is_empty()
        {
            self.cursor.x = 0;
            self.cursor.y += 1;
        } else if !self.buffer[self.cursor.y + 1].is_empty() {
            self.cursor.x = 0;
            self.cursor.y += 1;
            self.scroll_offset_down();
        }
        self.writer.move_cursor(&self.cursor, Some(self.offset));
    }

    fn back_space(&mut self) {
        if self.cursor.x > 0 {
            self.cursor.x -= 1;
        } else if self.cursor.y > 0 {
            self.cursor_up();
        }
        self.replace_byte(ERASE_BYTE);
        self.flush();
        self.writer.move_cursor(&self.cursor, Some(self.offset));
    }

    fn handle_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.nl(),
            b'\x1B' => {
                self.state = State::Esc;
            }
            b'\t' => self.write_string(b"    "),
            8 => self.back_space(),
            b' '..=b'~' => self.handle_escape_byte(byte),
            _ => {
                panic!("unprintable {}", byte);
            } // non printable
        }
    }

    fn apply_csi(&mut self, code: u8) {
        match code {
            // foreground
            0 => self.color = ColorPair::default(), // reset
            30 => self.color.foreground = Color::Black,
            31 => self.color.foreground = Color::Red,
            32 => self.color.foreground = Color::Green,
            33 => self.color.foreground = Color::Yellow,
            34 => self.color.foreground = Color::Blue,
            35 => self.color.foreground = Color::Magenta,
            36 => self.color.foreground = Color::Cyan,
            37 => self.color.background = Color::White,
            90 => self.color.foreground = Color::BBlack,
            91 => self.color.foreground = Color::BRed,
            92 => self.color.foreground = Color::BGreen,
            93 => self.color.foreground = Color::BYellow,
            94 => self.color.foreground = Color::BBlue,
            95 => self.color.foreground = Color::BMagenta,
            96 => self.color.foreground = Color::BCyan,
            //background
            40 => self.color.background = Color::Black,
            41 => self.color.background = Color::Red,
            42 => self.color.background = Color::Green,
            43 => self.color.background = Color::Yellow,
            44 => self.color.background = Color::Blue,
            45 => self.color.background = Color::Magenta,
            46 => self.color.background = Color::Cyan,
            47 => self.color.background = Color::White,
            100 => self.color.background = Color::BBlack,
            101 => self.color.background = Color::BRed,
            102 => self.color.background = Color::BGreen,
            103 => self.color.background = Color::BYellow,
            104 => self.color.background = Color::BBlue,
            105 => self.color.background = Color::BMagenta,
            106 => self.color.background = Color::BCyan,
            _ => {}
        }
    }

    fn handle_escape_byte(&mut self, byte: u8) {
        match &self.state {
            State::Default => {
                self.store_byte(byte);
            }
            State::Esc => match byte {
                b'[' => self.state = State::CSI(CSI::None), // start CSI sequence
                _ => self.state = State::Default,           // no implemented yet
            },
            State::CSI(c) => {
                self.state = match byte {
                    b'0'..=b'9' => {
                        match c {
                            CSI::None => {
                                let s = CSI::Some(byte - b'0');
                                State::CSI(s)
                            }
                            CSI::Some(n) => {
                                if *n == 0 {
                                    State::CSI(CSI::None) // avoid multiplication by 0
                                } else {
                                    let s =
                                        CSI::Some(n.saturating_mul(10).saturating_add(byte - b'0'));
                                    State::CSI(s)
                                }
                            }
                            CSI::Err => State::CSI(CSI::Err),
                        }
                    }
                    b if (0x40..=0x7E).contains(&b) => {
                        match b {
                            b'A' => self.try_cursor_up(),
                            b'B' => self.cursor_down(),
                            b'C' => self.cursor_right(),
                            b'D' => self.cursor_left(),
                            b'm' => match c {
                                CSI::None | CSI::Err => self.color = ColorPair::default(),
                                CSI::Some(n) => self.apply_csi(*n),
                            },
                            b'~' => {
                                if let CSI::Some(n) = c {
                                    match n {
                                        5 => self.scroll_offset_up(),
                                        6 => self.scroll_offset_down(),
                                        _ => {}
                                    }
                                }
                            }
                            _ => {}
                        }
                        State::Default
                    }
                    b';' => {
                        match c {
                            CSI::None | CSI::Err => self.color = ColorPair::default(),
                            CSI::Some(n) => self.apply_csi(*n),
                        }
                        State::CSI(CSI::None) // continue
                    }
                    _ => State::Default,
                };
            }
        }
    }
}

impl<W: WriterSoul> Write for Console<W> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s.as_bytes());
        Ok(())
    }
}

// impl<W: WriterSoul> Read for Console<W> {}

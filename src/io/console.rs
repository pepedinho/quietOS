use core::fmt::{Display, Write};

use crate::{io::Writer, println};

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

pub enum Color {
    Black,
    White,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    BBlack,
    BRed,
    BGreen,
    BYellow,
    BBlue,
    BMagenta,
    BCyan,
}

impl Display for Color {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let code = match self {
            Color::White => "0",
            Color::Black => "30",
            Color::Red => "31",
            Color::Green => "32",
            Color::Yellow => "33",
            Color::Blue => "34",
            Color::Magenta => "35",
            Color::Cyan => "36",
            Color::BBlack => "40",
            Color::BRed => "91",
            Color::BGreen => "92",
            Color::BYellow => "93",
            Color::BBlue => "94",
            Color::BMagenta => "95",
            Color::BCyan => "96",
        };

        write!(f, "\x1B[{}m", code)
    }
}

impl Color {
    pub fn as_vga(&self) -> u8 {
        match self {
            Color::Black => 0x00,
            Color::White => 0x0f,
            Color::Blue => 0x01,
            Color::Green => 0x02,
            Color::Cyan => 0x03,
            Color::Red => 0x04,
            Color::Magenta => 0x05,
            Color::Yellow => 0x06,
            Color::BBlack => 0x08,
            Color::BBlue => 0x09,
            Color::BGreen => 0x0a,
            Color::BCyan => 0x0b,
            Color::BRed => 0x0c,
            Color::BMagenta => 0x0d,
            Color::BYellow => 0x0e,
        }
    }
}

pub enum State {
    Default,
    Escaping,
    CSI,
}

impl Default for State {
    fn default() -> Self {
        State::Default
    }
}

pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub const fn new() -> Pos {
        Pos { x: 0, y: 0 }
    }
}

pub struct Console {
    writer: Writer,
    color: Color,
    state: State,
    cursor: Pos,
}

impl Console {
    pub const fn new() -> Self {
        Self {
            writer: Writer::new(VGA_BUFFER),
            state: State::Default,
            cursor: Pos::new(),
            color: Color::White,
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.writer.write_byte(byte, &mut self.cursor, &self.color);
    }

    fn handle_escape_sequence<I>(&mut self, iter: &mut I)
    where
        I: Iterator<Item = u8>,
    {
        match iter.next() {
            Some(b'[') => {}
            _ => {
                self.state = State::Default;
                return;
            }
        }

        let mut code: u8 = 0;
        // ici
        while let Some(b) = iter.next() {
            match b {
                b'0'..=b'9' => {
                    code = code.saturating_mul(10).saturating_add(b - b'0');
                }
                b'm' => {
                    match code {
                        0 => self.color = Color::White,
                        30 => self.color = Color::Black,
                        31 => self.color = Color::Red,
                        32 => self.color = Color::Green,
                        33 => self.color = Color::Yellow,
                        34 => self.color = Color::Blue,
                        35 => self.color = Color::Magenta,
                        36 => self.color = Color::Cyan,
                        40 => self.color = Color::BBlack,
                        91 => self.color = Color::BRed,
                        92 => self.color = Color::BGreen,
                        93 => self.color = Color::BYellow,
                        94 => self.color = Color::BBlue,
                        95 => self.color = Color::BMagenta,
                        96 => self.color = Color::Cyan,
                        _ => {}
                    }
                    break;
                }
                _ => {
                    // unknown sequence (no changes)
                    self.state = State::Default;
                    break;
                }
            }
        }
    }
}

impl Write for Console {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let mut iter = s.bytes();
        while let Some(byte) = iter.next() {
            match byte {
                b'\n' => {
                    self.cursor.x = 0;
                    self.cursor.y += 1;
                }
                b'\x1B' => {
                    self.state = State::Escaping;
                    self.handle_escape_sequence(&mut iter);
                }
                _ => self.write_byte(byte),
            }
        }
        Ok(())
    }

    // fn write_fmt(&mut self, args: core::fmt::Arguments<'_>) -> core::fmt::Result {
    //     struct Adapter<'a>(&'a mut Console);
    //     impl<'a> core::fmt::Write for Adapter<'a> {
    //         fn write_str(&mut self, s: &str) -> core::fmt::Result {
    //             self.0.write_str(s)
    //         }
    //     }

    //     let mut adaptater = Adapter(self);
    //     core::fmt::write(&mut adaptater, args)
    // }
}

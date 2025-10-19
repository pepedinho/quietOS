use core::fmt::{Display, Write};

use crate::io::{Writer, keyborad::Read};

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const ERASE_BYTE: u8 = 0x00;

#[derive(Default)]
pub enum Color {
    #[default]
    White,
    Black,
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

#[derive(Clone, Copy)]
pub enum CSI {
    None,
    Some(u8),
    Err,
}

pub enum State {
    Default,
    Esc,
    CSI(CSI),
}

#[derive(Default)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub const fn new() -> Pos {
        Pos { x: 0, y: 0 }
    }
}

pub struct ColorPair {
    foreground: Color,
    background: Color,
}

impl Default for ColorPair {
    fn default() -> Self {
        Self {
            foreground: Color::White,
            background: Color::Black,
        }
    }
}

impl ColorPair {
    pub fn shift(&self) -> u8 {
        let fg = self.foreground.as_vga();
        let bg = self.background.as_vga();
        (bg << 4) | fg
    }
}

pub struct Console {
    writer: Writer,
    color: ColorPair,
    state: State,
    cursor: Pos,
}

impl Console {
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            writer: Writer::new(VGA_BUFFER),
            state: State::Default,
            cursor: Pos::new(),
            color: ColorPair {
                foreground: Color::White,
                background: Color::Black,
            },
        }
    }

    pub fn relace_byte(&mut self, byte: u8) {
        self.writer.put_byte(byte, &mut self.cursor, &self.color);
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.writer.write_byte(byte, &mut self.cursor, &self.color);
    }

    pub fn write_string(&mut self, s: &str) {
        s.bytes().for_each(|b| self.handle_byte(b));
    }

    fn nl(&mut self) {
        self.cursor.x = 0;
        self.cursor.y += 1;
        self.writer.move_cursor(&self.cursor);
    }

    fn back_space(&mut self) {
        if self.cursor.x > 0 {
            self.cursor.x -= 1;
        } else if self.cursor.y > 0 {
            self.cursor.y -= 1;
        }
        self.relace_byte(ERASE_BYTE);
        self.writer.move_cursor(&self.cursor);
    }

    fn handle_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.nl(),
            b'\x1B' => {
                self.state = State::Esc;
            }
            b'\t' => self.write_string("    "),
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
            State::Default => self.write_byte(byte),
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
                    b'm' => {
                        match c {
                            CSI::None | CSI::Err => self.color = ColorPair::default(),
                            CSI::Some(n) => self.apply_csi(*n),
                        };
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

impl Write for Console {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

impl Read for Console {}

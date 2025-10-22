use core::fmt::Display;

#[derive(Default, Clone, Copy, PartialEq, Debug)]
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
pub struct ColorPair {
    pub foreground: Color,
    pub background: Color,
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

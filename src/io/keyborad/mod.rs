use crate::io::VGA;

#[warn(dead_code)]
pub const QWERTY_SCANCODES: [Option<u8>; 128] = {
    let mut t = [None; 128];
    t[0x02] = Some(b'1');
    t[0x03] = Some(b'2');
    t[0x04] = Some(b'3');
    t[0x05] = Some(b'4');
    t[0x06] = Some(b'5');
    t[0x07] = Some(b'6');
    t[0x08] = Some(b'7');
    t[0x09] = Some(b'8');
    t[0x0A] = Some(b'9');
    t[0x0B] = Some(b'0');
    t[0x10] = Some(b'q');
    t[0x11] = Some(b'w');
    t[0x12] = Some(b'e');
    t[0x13] = Some(b'r');
    t[0x14] = Some(b't');
    t[0x15] = Some(b'y');
    t[0x16] = Some(b'u');
    t[0x17] = Some(b'i');
    t[0x18] = Some(b'o');
    t[0x19] = Some(b'p');
    t[0x1E] = Some(b'a');
    t[0x1F] = Some(b's');
    t[0x20] = Some(b'd');
    t[0x21] = Some(b'f');
    t[0x22] = Some(b'g');
    t[0x23] = Some(b'h');
    t[0x24] = Some(b'j');
    t[0x25] = Some(b'k');
    t[0x26] = Some(b'l');
    t[0x2C] = Some(b'z');
    t[0x2D] = Some(b'x');
    t[0x2E] = Some(b'c');
    t[0x2F] = Some(b'v');
    t[0x30] = Some(b'b');
    t[0x31] = Some(b'n');
    t[0x32] = Some(b'm');
    t[0x39] = Some(b' '); // space
    t[0x1C] = Some(b'\n'); // Enter
    t[0x0E] = Some(8); // Backspace
    t
};

#[derive(Clone, Copy)]
pub enum ANSI {
    Up,
    Down,
    Left,
    Right,
    Pup,   // page up
    Pdown, // page down
}

pub trait Convert {
    fn to_seq(&self) -> &'static [u8];
}

impl Convert for ANSI {
    fn to_seq(&self) -> &'static [u8] {
        match self {
            ANSI::Up => &[0x1B, 0x5B, 0x41],
            ANSI::Down => &[0x1B, 0x5B, 0x42],
            ANSI::Left => &[0x1B, 0x5B, 0x44],
            ANSI::Right => &[0x1B, 0x5B, 0x43],
            ANSI::Pup => &[0x1B, 0x5B, 0x35, 0x7E],
            ANSI::Pdown => &[0x1B, 0x5B, 0x36, 0x7E],
        }
    }
}

// handler keys
#[derive(Clone, Copy)]
pub enum KeyboardState {
    None,
    CTRL,
    SHIFT,
    ALT,
}

#[derive(Clone, Copy)]
pub enum CHAR {
    C(u8),
}

impl CHAR {
    pub fn from_state(&self, state: KeyboardState, action: &mut KeyboardActions) -> Option<u8> {
        match self {
            CHAR::C(c) => match state {
                KeyboardState::CTRL => {
                    match c {
                        b't' => *action = KeyboardActions::TabInc,
                        b'd' => *action = KeyboardActions::TabDec,
                        _ => {
                            // println!("None");
                        }
                    }
                    None
                }
                KeyboardState::None => Some(*c),
                KeyboardState::ALT => match c {
                    b'\'' => Some(b'{'),
                    b'(' => Some(b'['),
                    b'-' => Some(b'|'),
                    b'_' => Some(b'\\'),
                    b')' => Some(b']'),
                    b'=' => Some(b'}'),
                    _ => Some(*c),
                },
                KeyboardState::SHIFT if c.is_ascii_alphabetic() => Some(*c - 32),
                KeyboardState::SHIFT => match c {
                    b'<' => Some(*c + 2),
                    b';' => Some(b'.'),
                    b',' => Some(b'?'),
                    b':' => Some(b'/'),
                    b'&' => Some(b'1'),
                    b'~' => Some(b'2'),
                    b'"' => Some(b'3'),
                    b'\'' => Some(b'4'),
                    b'(' => Some(b'5'),
                    b'-' => Some(b'6'),
                    b'`' => Some(b'7'),
                    b'_' => Some(b'8'),
                    b'^' => Some(b'9'),
                    b'@' => Some(b'0'),
                    b'=' => Some(b'+'),
                    _ => Some(*c),
                },
            },
        }
    }
}

#[derive(Clone, Copy)]
pub enum Sequence {
    ASCII(CHAR),
    ANSI(ANSI),
    StateChange(KeyboardState),
}

const AZERTY_SCANCODES: [Option<Sequence>; 256] = {
    let mut t = [None; 256];
    t[0x02] = Some(Sequence::ASCII(CHAR::C(b'&'))); // 1
    t[0x03] = Some(Sequence::ASCII(CHAR::C(b'~'))); // 2
    t[0x04] = Some(Sequence::ASCII(CHAR::C(b'"'))); // 3
    t[0x05] = Some(Sequence::ASCII(CHAR::C(b'\''))); // 4
    t[0x06] = Some(Sequence::ASCII(CHAR::C(b'('))); // 5
    t[0x07] = Some(Sequence::ASCII(CHAR::C(b'-'))); // 6
    t[0x08] = Some(Sequence::ASCII(CHAR::C(b'`'))); // 7
    t[0x09] = Some(Sequence::ASCII(CHAR::C(b'_'))); // 8
    t[0x0A] = Some(Sequence::ASCII(CHAR::C(b'^'))); // 9
    t[0x0B] = Some(Sequence::ASCII(CHAR::C(b'@'))); // 0
    t[0x0C] = Some(Sequence::ASCII(CHAR::C(b')'))); // )
    t[0x0D] = Some(Sequence::ASCII(CHAR::C(b'='))); // 0
    t[0x10] = Some(Sequence::ASCII(CHAR::C(b'a'))); // Q -> A
    t[0x11] = Some(Sequence::ASCII(CHAR::C(b'z'))); // W -> Z
    t[0x12] = Some(Sequence::ASCII(CHAR::C(b'e')));
    t[0x13] = Some(Sequence::ASCII(CHAR::C(b'r')));
    t[0x14] = Some(Sequence::ASCII(CHAR::C(b't')));
    t[0x15] = Some(Sequence::ASCII(CHAR::C(b'y')));
    t[0x16] = Some(Sequence::ASCII(CHAR::C(b'u')));
    t[0x17] = Some(Sequence::ASCII(CHAR::C(b'i')));
    t[0x18] = Some(Sequence::ASCII(CHAR::C(b'o')));
    t[0x19] = Some(Sequence::ASCII(CHAR::C(b'p')));
    t[0x1E] = Some(Sequence::ASCII(CHAR::C(b'q'))); // A -> Q
    t[0x1F] = Some(Sequence::ASCII(CHAR::C(b's')));
    t[0x20] = Some(Sequence::ASCII(CHAR::C(b'd')));
    t[0x21] = Some(Sequence::ASCII(CHAR::C(b'f')));
    t[0x22] = Some(Sequence::ASCII(CHAR::C(b'g')));
    t[0x23] = Some(Sequence::ASCII(CHAR::C(b'h')));
    t[0x24] = Some(Sequence::ASCII(CHAR::C(b'j')));
    t[0x25] = Some(Sequence::ASCII(CHAR::C(b'k')));
    t[0x26] = Some(Sequence::ASCII(CHAR::C(b'l')));
    t[0x27] = Some(Sequence::ASCII(CHAR::C(b'm')));
    t[0x2C] = Some(Sequence::ASCII(CHAR::C(b'w'))); // Z -> W
    t[0x2D] = Some(Sequence::ASCII(CHAR::C(b'x')));
    t[0x2E] = Some(Sequence::ASCII(CHAR::C(b'c')));
    t[0x2F] = Some(Sequence::ASCII(CHAR::C(b'v')));
    t[0x30] = Some(Sequence::ASCII(CHAR::C(b'b')));
    t[0x31] = Some(Sequence::ASCII(CHAR::C(b'n')));
    t[0x32] = Some(Sequence::ASCII(CHAR::C(b',')));
    t[0x33] = Some(Sequence::ASCII(CHAR::C(b';')));
    t[0x34] = Some(Sequence::ASCII(CHAR::C(b':')));
    t[0x35] = Some(Sequence::ASCII(CHAR::C(b'!')));
    t[0x56] = Some(Sequence::ASCII(CHAR::C(b'<')));
    t[0x48] = Some(Sequence::ANSI(ANSI::Up));
    t[0x50] = Some(Sequence::ANSI(ANSI::Down));
    t[0x4B] = Some(Sequence::ANSI(ANSI::Left));
    t[0x4D] = Some(Sequence::ANSI(ANSI::Right));
    t[0x49] = Some(Sequence::ANSI(ANSI::Pup));
    t[0x51] = Some(Sequence::ANSI(ANSI::Pdown));
    t[0x1D] = Some(Sequence::StateChange(KeyboardState::CTRL)); // ctrl
    t[0x2A] = Some(Sequence::StateChange(KeyboardState::SHIFT)); // shift
    t[0x38] = Some(Sequence::StateChange(KeyboardState::ALT)); // alt
    t[0x9D] = Some(Sequence::StateChange(KeyboardState::None)); // ctrl release
    t[0xAA] = Some(Sequence::StateChange(KeyboardState::None)); // shift release
    t[0xB8] = Some(Sequence::StateChange(KeyboardState::None)); // alt release
    t[0x39] = Some(Sequence::ASCII(CHAR::C(b' '))); // space
    t[0x1C] = Some(Sequence::ASCII(CHAR::C(b'\n'))); // Enter
    t[0x0E] = Some(Sequence::ASCII(CHAR::C(8))); // Backspace
    t[0x0F] = Some(Sequence::ASCII(CHAR::C(b'\t'))); // Backspace
    t
};

pub trait Read {
    fn has_data(&mut self) -> bool {
        unsafe { (VGA::inb(0x64) & 1) != 0 }
    }

    fn read_byte(&mut self) -> Option<u8> {
        if self.has_data() {
            unsafe { Some(VGA::inb(0x60)) }
        } else {
            None
        }
    }

    fn read(&mut self, buf: &mut [u8]) -> usize {
        let mut i = 0;
        while i < buf.len() {
            if let Some(b) = self.read_byte() {
                buf[i] = b;
                i += 1;
            } else {
                break;
            }
        }
        i
    }
}

fn scancode_to_ascii(scancode: u8) -> Option<Sequence> {
    AZERTY_SCANCODES.get(scancode as usize).copied().flatten()
}

#[derive(Clone, Copy)]
pub enum KeyboardActions {
    None,
    TabInc,
    TabDec,
}

#[derive(Clone, Copy)]
pub struct Keyboard {
    pub state: KeyboardState,
    pub action: KeyboardActions,
}

impl Keyboard {
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Keyboard {
            state: KeyboardState::None,
            action: KeyboardActions::None,
        }
    }
    pub fn switch_state(&mut self, state: KeyboardState) {
        self.state = state
    }

    pub fn get_action(&self) -> KeyboardActions {
        self.action
    }

    pub fn no_action(&mut self) {
        self.action = KeyboardActions::None
    }
}

impl Read for Keyboard {}

impl Keyboard {
    pub fn read_stdin_once(&mut self) -> Sequence {
        loop {
            if let Some(scancode) = self.read_byte()
                && let Some(ch) = scancode_to_ascii(scancode)
            {
                return ch;
            }
        }
    }

    // pub fn read_stdin(&mut self, keyboard: &mut Keyboard) -> ! {
    //     loop {
    //         if let Some(scancode) = self.read_byte()
    //             && let Some(ch) = scancode_to_ascii(scancode)
    //         {
    //             match ch {
    //                 Sequence::ANSI(e) => {
    //                     let seq = e.to_seq();
    //                     self.write_string(seq);
    //                 }
    //                 Sequence::ASCII(ch) => {
    //                     if let Some(c) = ch.from_state(keyboard.state, &mut keyboard.action) {
    //                         self.write_string(&[c]);
    //                     } else {
    //                         keyboard.state = KeyboardState::None;
    //                     }
    //                 }
    //                 Sequence::StateChange(h) => {
    //                     keyboard.switch_state(h);
    //                 }
    //             }
    //         }
    //     }
    // }
}

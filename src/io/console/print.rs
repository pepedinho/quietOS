use core::fmt::Write;

use crate::{
    io::{
        console::{Console, writer::Writer},
        keyborad::{Convert, Keyboard, KeyboardActions, Sequence},
    },
    sync::mutex::Mutex,
};

pub fn print(args: core::fmt::Arguments) {
    // use core::fmt::Write;

    let mut ttys = TTY_TABLE.lock();
    ttys.active().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($args:expr) => {
        $crate::io::console::print::print($args);
    };
}

#[macro_export]
macro_rules! println {
    () => {
        ($crate::print!(core::format_args!("\n")))
    };
    ($($arg:tt)*) => {{
        $crate::print!(core::format_args!($($arg)*));
        $crate::print!(core::format_args!("\n"));
    }};
}

const TTY_COUNT: usize = 10;

#[allow(non_camel_case_types)]
pub struct TTY_POOL {
    pub consoles: [Console<Writer>; TTY_COUNT],
    pub keyboard: Keyboard,
    active: usize,
}

unsafe impl Sync for TTY_POOL {}

pub static TTY_TABLE: Mutex<TTY_POOL> = Mutex::new(TTY_POOL::new());

impl TTY_POOL {
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        TTY_POOL {
            consoles: [
                Console::new(Writer::new()),
                Console::new(Writer::new()),
                Console::new(Writer::new()),
                Console::new(Writer::new()),
                Console::new(Writer::new()),
                Console::new(Writer::new()),
                Console::new(Writer::new()),
                Console::new(Writer::new()),
                Console::new(Writer::new()),
                Console::new(Writer::new()),
            ],
            keyboard: Keyboard::new(),
            active: 0,
        }
    }

    pub fn keyboard_action(&mut self) {
        match self.keyboard.get_action() {
            KeyboardActions::TabInc => self.inc_tab(),
            KeyboardActions::TabDec => self.dec_tab(),
            KeyboardActions::None => {}
        }
    }

    pub fn active(&mut self) -> &mut Console<Writer> {
        &mut self.consoles[self.active]
    }

    pub fn inc_tab(&mut self) {
        self.active = (self.active + 1) % TTY_COUNT;
        self.consoles[self.active].flush();
        self.keyboard.no_action();
    }

    pub fn dec_tab(&mut self) {
        self.active = (self.active + TTY_COUNT - 1) % TTY_COUNT;
        self.consoles[self.active].flush();
        self.keyboard.no_action();
    }

    pub fn print_byte_from_keyboard(&mut self, seq: &Sequence) {
        // let seq = self.keyboard.read_stdin_once();

        let console = &mut self.consoles[self.active];
        match seq {
            Sequence::ANSI(e) => {
                let seq = e.to_seq();
                console.write_string(seq);
            }
            Sequence::ASCII(ch) => {
                if let Some(c) = ch.from_state(self.keyboard.state, &mut self.keyboard.action) {
                    console.write_string(&[c]);
                }
            }
            Sequence::StateChange(h) => {
                self.keyboard.switch_state(*h);
            }
        }
    }

    // pub fn read_in_active(&mut self) -> ! {
    //     let console = &mut self.consoles[self.active];
    //     console.read_stdin(&mut self.keyboard);
    // }

    pub fn interpret_byte_from_keyboard(&mut self, seq: &Sequence) {
        self.print_byte_from_keyboard(seq);
        self.keyboard_action();
    }
}

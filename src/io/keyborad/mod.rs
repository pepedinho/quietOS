use crate::io::{VGA, console::Console};

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

const AZERTY_SCANCODES: [Option<u8>; 128] = {
    let mut t = [None; 128];
    t[0x02] = Some(b'1'); // 1
    t[0x03] = Some(b'2'); // 2
    t[0x04] = Some(b'3'); // 3
    t[0x05] = Some(b'4'); // 4
    t[0x06] = Some(b'5'); // 5
    t[0x07] = Some(b'6'); // 6
    t[0x08] = Some(b'7'); // 7
    t[0x09] = Some(b'8'); // 8
    t[0x0A] = Some(b'9'); // 9
    t[0x0B] = Some(b'0'); // 0
    t[0x10] = Some(b'a'); // Q -> A
    t[0x11] = Some(b'z'); // W -> Z
    t[0x12] = Some(b'e');
    t[0x13] = Some(b'r');
    t[0x14] = Some(b't');
    t[0x15] = Some(b'y');
    t[0x16] = Some(b'u');
    t[0x17] = Some(b'i');
    t[0x18] = Some(b'o');
    t[0x19] = Some(b'p');
    t[0x1E] = Some(b'q'); // A -> Q
    t[0x1F] = Some(b's');
    t[0x20] = Some(b'd');
    t[0x21] = Some(b'f');
    t[0x22] = Some(b'g');
    t[0x23] = Some(b'h');
    t[0x24] = Some(b'j');
    t[0x25] = Some(b'k');
    t[0x26] = Some(b'l');
    t[0x27] = Some(b'm');
    t[0x2C] = Some(b'w'); // Z -> W
    t[0x2D] = Some(b'x');
    t[0x2E] = Some(b'c');
    t[0x2F] = Some(b'v');
    t[0x30] = Some(b'b');
    t[0x31] = Some(b'n');
    t[0x39] = Some(b' '); // space
    t[0x1C] = Some(b'\n'); // Enter
    t[0x0E] = Some(8); // Backspace
    t[0x0F] = Some(b'\t'); // Backspace
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

fn scancode_to_ascii(scancode: u8) -> Option<u8> {
    AZERTY_SCANCODES.get(scancode as usize).copied().flatten()
}

impl Console {
    pub fn read_stdin(&mut self) -> ! {
        loop {
            if let Some(scancode) = self.read_byte()
                && let Some(ch) = scancode_to_ascii(scancode)
            {
                self.write_string(str::from_utf8(&[ch]).unwrap());
            }
        }
    }
}

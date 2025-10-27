pub mod console;
pub mod keyborad;

const VGA_WIDTH: usize = 80;
#[allow(dead_code)]
const VGA_HEIGHT: usize = 25;
const VGA_CMD_PORT: u16 = 0x3D4;
const VGA_DATA_PORT: u16 = 0x3D5;

pub const WHITE: u8 = 0x0f;
pub const YELLOW: u8 = 0x0E;

pub struct VGA {}

impl VGA {
    /// write byte from I/O port
    #[inline(always)]
    pub unsafe fn outb(port: u16, value: u8) {
        unsafe {
            core::arch::asm!(
                "out dx, al",
                in("dx") port,
                in("al") value,
                options(nomem, nostack, preserves_flags),
            );
        }
    }

    /// read byte from I/O port
    #[inline(always)]
    unsafe fn inb(port: u16) -> u8 {
        let mut value: u8;
        unsafe {
            core::arch::asm!(
                "in al, dx",
                in("dx") port,
                out("al") value,
                options(nomem, nostack, preserves_flags),
            );
        }
        value
    }

    #[inline(always)]
    unsafe fn io_wait() {
        unsafe {
            core::arch::asm!("out 0x80, al", in("al") 0u8, options(nomem, nostack, preserves_flags));
        }
    }
}

#[repr(align(16))]
pub struct AlignedStack(pub [u8; 4096]);

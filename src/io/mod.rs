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
    /// # Safety
    /// ```text
    ///     - Make sure that port is a valid I/O device.
    ///     - Don't call it unprotected multitasking if the device isn't designed for it.
    ///     - Avoid ports reserved for the chipset or BIOS.
    /// ```
    ///
    ///
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
}

#[repr(align(16))]
pub struct AlignedStack(pub [u8; 4096]);

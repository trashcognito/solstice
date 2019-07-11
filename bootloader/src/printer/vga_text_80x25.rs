use core::{
    fmt::{Result, Write},
    intrinsics,
    sync::atomic::{AtomicUsize, Ordering},
};

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut _;
const SCREEN_SIZE: usize = 80 * 25;

pub static CURRENT_OFFSET: AtomicUsize = AtomicUsize::new(160);

pub struct Printer;

impl Printer {
    pub fn clear_screen(&mut self) {
        unsafe {
            intrinsics::volatile_set_memory(VGA_BUFFER, 0, SCREEN_SIZE);
        }

        CURRENT_OFFSET.store(0, Ordering::Relaxed);
    }
}

impl Write for Printer {
    fn write_str(&mut self, s: &str) -> Result {
        for byte in s.bytes() {
            let index = CURRENT_OFFSET.fetch_add(2, Ordering::Relaxed) as isize;

            unsafe {
                VGA_BUFFER.offset(index).write_volatile(byte);
                VGA_BUFFER.offset(index + 1).write_volatile(0x4f);
            }
        }

        Ok(())
    }
}

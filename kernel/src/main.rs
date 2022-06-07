#![no_std]
#![no_main]


mod screen;

use core::arch::asm;
use core::{fmt, slice};
use core::fmt::Arguments;
use shared::framebuffer;
use crate::screen::{Renderable, Screen};

static mut CONSOLE: Option<Screen> = Option::None;

#[derive(Debug)]
pub struct ConsoleWrite;

impl fmt::Write for ConsoleWrite {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe { CONSOLE.as_mut().unwrap().write_string(0, 0, s, [0, 0, 0]); }
        Ok(())
    }
}
#[allow(unused_macros)]
macro_rules! println {
    ($($t:tt)*)=>{{
        use core::fmt::Write;
        writeln!(ConsoleWrite, $($t)*).unwrap();
    }}
}

macro_rules! clear_screen {
    ()=>{
    unsafe{CONSOLE.as_mut().unwrap().clear([255,255,255])};
    }
}

#[no_mangle]
pub extern "sysv64" fn kernel_main(fb: &framebuffer::FrameBuffer) {
    unsafe {
        CONSOLE = Some((*fb).into());
    }
    clear_screen!();
    println!("hello world");
    loop {
        unsafe { asm!("hlt") }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        unsafe { asm!("hlt") }
    }
}

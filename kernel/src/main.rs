#![no_std]
#![no_main]


mod screen;
mod console;

use core::arch::asm;
use core::{fmt, slice};
use core::fmt::Arguments;
use shared::framebuffer;
use crate::screen::{Renderable, Screen};

#[no_mangle]
pub extern "sysv64" fn kernel_main(fb: &framebuffer::FrameBuffer) {
    screen::initialize_screen(fb);
unsafe{screen::MAIN_SCREEN.as_mut().unwrap().clear([255,255,255]);}
    let mut console = console::Console::new();
    console.put_str("HELLO WORLd");
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

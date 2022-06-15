#![no_std]
#![no_main]


mod screen;
#[macro_use]
mod console;
mod pci;
use core::arch::asm;
use core::{fmt, slice};
use core::fmt::Arguments;
use shared::framebuffer;
use crate::screen::{Renderable, Screen};

#[no_mangle]
pub extern "sysv64" fn kernel_main(fb: &framebuffer::FrameBuffer,rsdp:*const u8) {
    screen::initialize_screen(fb);
    console::initialize_console();
    unsafe {
        screen::MAIN_SCREEN.as_mut().unwrap().clear([255, 255, 255]);
        screen::MAIN_SCREEN.as_mut().unwrap().draw_cursor(100,100,[255,0,0]);
    }
    println!("hoge {:#016x}",rsdp as u64);
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

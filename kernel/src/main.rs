#![no_std]
#![no_main]


mod screen;

use core::arch::asm;
use core::slice;
use shared::framebuffer;
use crate::screen::Renderable;

#[no_mangle]
pub extern "sysv64" fn kernel_main(fb:&framebuffer::FrameBuffer) {
    let mut screen = screen::Screen::new(&fb);
    screen.clear([255,255,255]);
    screen.write_string(100,30,"HELLO WORLD!!",[255,0,255]);
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

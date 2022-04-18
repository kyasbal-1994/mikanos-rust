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
    for i in 0..25 {
        let c = ('A' as u8) + i;
        screen.write_char(10* (i as u32),100,c as char,[255,0,0]);
    }
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
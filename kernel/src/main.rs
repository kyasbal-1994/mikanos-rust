#![no_std]
#![no_main]
#![no_mangle]

use core::arch::asm;
use core::slice;
use shared::framebuffer;

pub extern "sysv64" fn kernel_main(fb:&framebuffer::FrameBuffer) {
    let mut buf = unsafe {
        slice::from_raw_parts_mut(fb.framebuffer ,(fb.stride*fb.resolution.1*4) as usize)
    };
    for i in 0..buf.len(){
        unsafe {
            buf[i] = 255;
        }

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
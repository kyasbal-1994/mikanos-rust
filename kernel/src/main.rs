#![no_std]
#![no_main]
#![no_mangle]

use core::arch::asm;

pub extern "sysv64" fn kernel_main() {
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
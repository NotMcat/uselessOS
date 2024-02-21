#![no_std]
#![no_main]

use libk::println;
use core::arch::asm;

use core::panic::PanicInfo;

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _start() -> ! {

    unsafe { asm!("int 0x80");}

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

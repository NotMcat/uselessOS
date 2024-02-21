#![no_std]
#![no_main]

mod disk;

use core::arch::asm;
use core::include_str;
use core::panic::PanicInfo;
use core::arch::global_asm;

global_asm!(include_str!("stage1.asm"));

#[no_mangle]
pub extern "C" fn _boot() -> ! {
    
    disk::read_stub();
    unsafe { asm!("jmp {0:x}", in(reg) 0x7e00); }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn fail() -> ! {
    loop {}
}
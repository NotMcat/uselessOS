#![no_std]
#![no_main]

use core::panic::PanicInfo;
use libk::println;

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _start() -> ! {
    let mut x = 0;
    for _ in 0..999999 {
        x += 1;
        println!("{}", x);
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {} 
}
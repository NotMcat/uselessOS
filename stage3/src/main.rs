#![no_std]
#![no_main]
#![feature(pointer_byte_offsets)]

mod disk;

use libk::*;
use libk::println;
use core::arch::asm;

use core::panic::PanicInfo;

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _start() -> ! {

    let bx: u16;

    unsafe {
        asm!(
            "mov {0:e}, 0x10",
            "mov ds, {0:e}",
            "mov es, {0:e}",
            "mov ss, {0:e}",

            "mov esp, {1:e}",

            out(reg) _,
            in(reg) 0x30_0000,
            out("bx") bx,
        );
    }

    println!("[+] Loading kernel ...");

    println!("[!] Loading args... {:X}", bx);

    let target = 0x10_0000 as *mut u8;
    disk::read(4096, 2048, target);

    println!("[+] Jumping to kernel ...");

    unsafe {
        asm!(
            "mov bx, {1:x}",
            "jmp {0:e}",
            in(reg) 0x10_0000,
            in(reg) bx,
        );
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
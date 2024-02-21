#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(pointer_byte_offsets)]

mod tss;
mod idt;
mod pic;
mod boot;
mod disk;
mod keyboard;
mod exceptions;

use libk;
use libk::*;

use idt::IDT;
use pic::PICS;
use core::arch::asm;
use core::panic::PanicInfo;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
struct MemoryMapEntry {
    base: u64,
    length: u64,
    memory_type: u32,
    reserved_acpi: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
struct MemoryMap {
    entries: [MemoryMapEntry; 32],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct BootInfo {
    mmap: MemoryMap,
    tss: u16,
}


#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _start() -> ! {
    let info = args();

    println!("[!] Kernel reached and args loaded");

    idt();
    tss_flush();
    umode(info.tss as u32);

    loop {}
}

fn idt() {
    unsafe {
        IDT.init();
        IDT.processor_exceptions();
        IDT.add(exceptions::TIMER_INT as usize, exceptions::timer_handler as u32, );
        IDT.add(exceptions::KEYBOARD_INT as usize, exceptions::keyboard_handler as u32, );
        IDT.add(exceptions::DISK_INT as usize, exceptions::disk_handler as u32, );
        IDT.add(exceptions::RTC_INT as usize, exceptions::rtc_handler as u32, );
        IDT.add(0x80, exceptions::syscall as u32, );
        IDT.load();
        PICS.init();

        asm!("sti");
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("PANIC\n {}", info);
    loop {}
}

fn args() -> BootInfo {
    let bx: u16;
    unsafe { asm!( "mov {0:x}, bx" , out(reg) bx ); }
    let info = bx as *const BootInfo;
    unsafe{ *info }
}

fn tss_esp(addr: u32) {
    let ptr = addr as *mut tss::TaskStateSegment;

    unsafe {

        let esp:u32;
        asm!("mov {0:e}, esp", out(reg) esp);

        let mut tss = *ptr;
        tss.esp0 = esp;

        core::ptr::write(ptr, tss);

        println!("{:?}", tss);
    }
}

fn tss_flush() {
    unsafe{
        asm!(
            "mov ax, 0x28",
            "ltr ax",
        );
    }
}

fn umode(addr: u32) {
    let target = 0x60_0000 as *mut u8;
    disk::read(8192, 2048, target);

    tss_esp(addr);

    println!("[+] Jumping to userland ...");

    unsafe {
        asm!(
            "mov ax, 0x23",
            "mov ds, ax",
            "mov es, ax",
            "mov fs, ax",
            "mov gs, ax",
        
            //"mov esp, {1:e}",
            "mov eax, esp",
            "push 0x23",
            "push eax",
            "pushfd",
            "push 0x1B",
            "push {0:e}",
            "iretd",

            in(reg) 0x60_0000,
            //in(reg) 0x40_0000,
        );
    }
}
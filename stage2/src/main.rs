#![no_std]
#![no_main]
#![feature(pointer_byte_offsets)]

use libk::println;

mod tss;
mod gdt;
mod disk;

use gdt::GDT;

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
struct BootInfo {
    mmap: MemoryMap,
    tss: u16,
}


#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _start() -> ! {

    println!("[+] Loading stage 3 ...");

    disk::read_stub();

    println!("[+] Jumping to stage 3 ...");

    protected_mode();

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

fn protected_mode() {

    unsafe {
        let tss_addr = GDT.write_tss();
        GDT.load();

        let boot_info = BootInfo {
            mmap: mmap(),
            tss: tss_addr,
        };

        asm!("mov eax, cr0", "or al, 1", "mov cr0, eax");

        asm!("mov bx, {0:x}", in(reg) &boot_info as *const _ as u16);

        asm!("ljmp $0x8, $0xbe00", options(att_syntax));
    }

}

fn mmap() -> MemoryMap { 
    unsafe {
        let mut z = 0;

        let mut mmap = MemoryMap {
            entries: [MemoryMapEntry {
                base: 0,
                length: 0,
                memory_type: 0,
                reserved_acpi: 0,
            };32],
        };

        let mut map = MemoryMapEntry {
            base: 0,
            length: 0,
            memory_type: 0,
            reserved_acpi: 0,
        };

        let mut sign: u32 = 0;
        let mut bytes: u32 = 0;
        let mut cont: u32 = 0;

        for i in 0..32 {

            asm!(
                "mov es, ax",
                "mov eax, 0xE820",
                "mov ecx, 24",
                "mov edx, 0x534D4150",
                "int 0x15",
                in("ebx") i,
                in("ax") ((&mmap.entries[z] as *const _ as usize) / 16),
                in("di") ((&mmap.entries[z] as *const _ as usize) % 16),
                lateout("eax") sign,
                lateout("ecx") bytes,
                lateout("ebx") cont,
            );         
            
            z += 1;

            if cont == 0 {
                break;
            }
        }

        mmap
        
    }
}

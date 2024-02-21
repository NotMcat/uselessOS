use crate::port::{outb, inb, inw};
use core::arch::asm;

pub fn read<T>(lba: u64, sectors: u16, target: *mut T) {

    while is_busy() {}

    outb(0x3f6, 0b00000010);

    outb(0x1F1, 0x00);
    outb(0x1F2, sectors as u8);
    outb(0x1F3, lba as u8);
    outb(0x1F4, (lba >> 8) as u8);
    outb(0x1F5, (lba >> 16) as u8);
    outb(0x1F6, (0xE0 | ((lba >> 24) & 0x0F)) as u8);

    outb(0x1F7, 0x20);

    let mut sectors_left = sectors;
    let mut target_pointer = target;
    
    while sectors_left > 0 {
        for _ in 0..256 {
            while is_busy() {}
            while !is_ready() {}

            let bytes_16 = inw(0x1F0) as u16;
            
            unsafe {
                core::ptr::write_unaligned(target_pointer as *mut u16, bytes_16);
                target_pointer = target_pointer.byte_add(2);
            }
            
        }
        sectors_left -= 1;
        
    }

    reset();
}


pub fn reset() {
    outb(0x3f6, 0b00000110);
    outb(0x3f6, 0b00000010);
}

pub fn is_ready() -> bool {
    let status: u8 = inb(0x1F7);

    (status & 0b01000000) != 0
}

pub fn is_busy() -> bool {
    let status: u8 = inb(0x1F7);

    (status & 0b10000000) != 0
}

fn delay() {
    for _ in 0..10000 {
        unsafe { asm!("nop") };
    }
}

pub fn check_disk() -> [bool; 2] {
    let mut master = false;
    let mut slave = false;

    outb(0x1F6, 0xF0);
    outb(0x1F7, 0xEC);

    delay();

    let status = inb(0x1F7);
    if status != 0 {
        slave = true;
    }
    
    delay();

    outb(0x1F6, 0xE0);
    outb(0x1F7, 0xEC);

    delay();

    let status = inb(0x1F7);
    if status != 0 {
        master = true;
    }

    [master, slave]
}
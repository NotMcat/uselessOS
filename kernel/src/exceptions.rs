use crate::println;
use crate::port::{inb, outb};
use crate::pic::PICS;
use crate::keyboard;
use core::arch::asm;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct InterruptStackFrame {
    err_code: u32,
    eip: u32,
    cs: u32,
    eflags: u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct StackFrame {
    eip: u32,
    cs: u32,
    eflags: u32,
}


pub extern "x86-interrupt" fn div_error() {
    println!("DIVISION ERROR!");
}

pub extern "x86-interrupt" fn bounds(info: InterruptStackFrame) {
    println!("OUT OF BOUNDS! -> {:?}", info);
}

pub extern "x86-interrupt" fn invalid_opcode(info: StackFrame) {

    println!("IO -> {:?}", info);
}

pub extern "x86-interrupt" fn double_fault() {

    println!("DOUBLE FAULT!");
}

pub extern "x86-interrupt" fn general_protection_fault(info: InterruptStackFrame) {

    println!("GPF -> {:?}", info);
}

pub extern "x86-interrupt" fn page_fault() {

    println!("PAGE FAULT!");
}

pub extern "x86-interrupt" fn generic_handler() {

    println!("EXCEPTION!");
}

/* SPECIFIC STUFF */

pub const TIMER_INT: u8 = 32;

pub extern "x86-interrupt" fn timer_handler() {
    unsafe{ PICS.end_interrupt(TIMER_INT); }
}

pub const KEYBOARD_INT: u8 = 33;

pub extern "x86-interrupt" fn keyboard_handler() {

    let data: u8 = inb(0x60);

    keyboard::keyboard_italian(data);

    unsafe{ PICS.end_interrupt(KEYBOARD_INT); }
}

pub const DISK_INT: u8 = 46;

pub extern "x86-interrupt" fn disk_handler() {
    outb(0x20, 0x20);
}

pub const RTC_INT: u8 = 40;

pub extern "x86-interrupt" fn rtc_handler() {
    outb(0x70, 0x0C);
    let _ = inb(0x71);	

    unsafe{ PICS.end_interrupt(RTC_INT); }
}

pub extern "x86-interrupt" fn syscall() {
    println!("SYS-EXCEPTION!");
}
use lazy_static::lazy_static;
use crate::port::{inb, outb};
use spin::Mutex;

pub static mut PICS: Pics = Pics {
    master: Pic {
        offset: 32,
        port: 0x20,
        data: 0x21,
    },
    slave: Pic {
        offset: 40,
        port: 0xa0,
        data: 0xa1,
    },
};

struct Pic {
    offset: u8,
    port: u8,
    data: u8,
}

pub struct Pics {
    master: Pic,
    slave: Pic,
}

impl Pic {
    pub fn read_data(&self) -> u8 {
        let data: u8;
        
        data = inb(self.data as u16);

        data
    }

    pub fn write_data(&self, data: u8) {
        outb(self.data as u16, data);
    }

    pub fn send_command(&self, command: u8) {
        outb(self.port as u16, command);
    }

    pub fn end_interrupt(&self) {
        outb(self.port as u16, 0x20 as u8);
    }

    pub fn handles_interrupt(&self, interupt: u8) -> bool {
        self.offset <= interupt && interupt < self.offset + 8
    }
}

impl Pics {
    pub fn init(&self) {

        let mask1 = self.master.read_data();
        let mask2 = self.slave.read_data();

        self.master.send_command(0x11);
        wait();
        self.slave.send_command(0x11);
        wait();

        self.master.write_data(self.master.offset);
        wait();
        self.slave.write_data(self.slave.offset);
        wait();

        self.master.write_data(4);
        wait();

        self.slave.write_data(2);
        wait();

        self.master.write_data(0x01);
        wait();
        self.slave.write_data(0x01);
        wait();

        self.master.write_data(mask1);
        self.slave.write_data(mask2);
    }

    pub fn handles_interrupt(&self, interrupt: u8) -> bool {
        self.master.handles_interrupt(interrupt) || self.slave.handles_interrupt(interrupt)
    }

    pub fn end_interrupt(&self, interrupt: u8) {
        if self.handles_interrupt(interrupt) {
            if self.slave.handles_interrupt(interrupt) {
                self.slave.end_interrupt();
            }
            self.master.end_interrupt();
        }
    }
}


pub fn wait() {
    outb(0x80 as u16, 0 as u8);
}
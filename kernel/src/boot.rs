#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct BootInfo {
    pub mmap: MemoryMap,
    pub rsdp: Rsdp,
    pub vbe: VbeInfoBlock,
    pub mode: VbeModeInfoBlock,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct VbeInfoBlock {
    pub signature: [u8; 4],
    version: u16,
    oem: [u16; 2],
    dunno: [u8; 4],
    video_ptr: [u16; 2],
    memory_size: u16,
    reserved: [u8; 492],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct MemoryMapEntry {
    pub base: u64,
    pub length: u64,
    pub memory_type: u32,
    pub reserved_acpi: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct MemoryMap {
    pub entries: [MemoryMapEntry; 32],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Rsdp {
    pub signature: [u8; 8],
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub revision: u8,
    pub rsdt_address: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct RsdtHeader {
    signature: [u8; 4],
    length: u32,
    revision: u8,
    checksum: u8,
    oem_id: [u8; 6],
    oem_table_id: [u8; 8],
    oem_revision: u32,
    creator_id: u32,
    creator_revision: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Rsdt {
    header: RsdtHeader,
    ptr: [u32; 10],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct GenericTable {
    signature: [u8; 4]
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct VbeModeInfoBlock {
    attributes: u16,
	window_a: u8,
	window_b: u8,
	granularity: u16,
	window_size: u16,
	segment_a: u16,
	segment_b: u16,
	win_func_ptr: u32,
	pitch: u16,
	width: u16,
	height: u16,
	w_char: u8,
	y_char: u8,
	planes: u8,
	bpp: u8,
	banks: u8,
	memory_model: u8,
	bank_size: u8,
	image_pages: u8,
	reserved0: u8,
 
	red_mask: u8,
	red_position: u8,
	green_mask: u8,
	green_position: u8,
	blue_mask: u8,
	blue_position: u8,
	reserved_mask: u8,
	reserved_position: u8,
	direct_color_attributes: u8,
 
	pub framebuffer: u32,
	off_screen_mem_off: u32,
	off_screen_mem_size: u16,
	reserved1: [u8; 206],
}


impl BootInfo {
    pub fn get_acpi(&self) {
        unsafe {
            let rsdt_ptr = self.rsdp.rsdt_address as *const Rsdt;
            let rsdt = rsdt_ptr.read();

            let entries = (rsdt.header.length as usize - core::mem::size_of::<RsdtHeader>()) / 4;

            for i in 0..entries {
                let entry = *(rsdt.ptr[i] as *const GenericTable);
                crate::print!("{}", entry.signature[0] as char);
                crate::print!("{}", entry.signature[1] as char);
                crate::print!("{}", entry.signature[2] as char);
                crate::print!("{}", entry.signature[3] as char);
                crate::println!("");

            }       
        }
    }

    pub fn get_mmap(&self, start: u64) -> MemoryMapEntry {
        for i in 0..32 {
            if self.mmap.entries[i].base == start {
                return self.mmap.entries[i];
            }
        }

        panic!("NOP STUPIDO COGLIONE");
    }
}


ENTRY(_start)

SECTIONS {
    . = 0xbe00;

    _stub_start = .;

    .start : {
        *(.start)
    }

    .text : {
        *(.text .text.*)
    }

    .bss : {
        *(.bss .bss.*)
    }

    .rodata : {
        *(.rodata .rodata.*)
    }
    
    .data : {
        *(.data .data.*)
    }

    .eh_frame : {
        *(.eh_frame .eh_frame.*)
    }

    .eh_frame_hdr : {
        *(.eh_frame_hdr .eh_frame_hdr.*)
    }

    . = _stub_start + 0x4000;

}
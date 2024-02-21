ENTRY(_start)

SECTIONS {
    . = 0x00600000;

    USER_START = .;

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

    . = USER_START + 0x00100000 - 2;

    .phony : {
        SHORT(0xFFFF);
    }
}
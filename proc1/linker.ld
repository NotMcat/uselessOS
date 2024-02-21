ENTRY(_start)

SECTIONS {
    . = 0x02000000;

    APP_START = .;

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

    .end_marker :
    {
        SHORT(0x0000)
    }
}
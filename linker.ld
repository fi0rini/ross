OUTPUT_ARCH(aarch64)
ENTRY(_start)

SECTIONS {
    . = 0x80000; /* Raspberry Pi loads kernel at 0x80000 */

    .text : {
        KEEP(*(.text._start))
        *(.text*)
    }

    .rodata : { *(.rodata*) }
    .data : { *(.data*) }
    .bss : { *(.bss*) }
}
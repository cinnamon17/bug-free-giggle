ENTRY(_start)

SECTIONS
{
    /* Aquí es donde QEMU espera encontrar el código en la máquina 'virt' */
    . = 0x80000000;

    .text : {
        /* Metemos la función _start al principio de todo */
        *(.text._start)
        *(.text .text.*)
    }

    .rodata : { *(.rodata .rodata.*) }
    .data : { *(.data .data.*) }
    .bss : { *(.bss .bss.*) }
}

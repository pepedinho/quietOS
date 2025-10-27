ENTRY(_start)

SECTIONS
{
    /* On commence à 1 Mo */
    . = 0x00100000;

    .multiboot :
    {
        KEEP(*(.multiboot))
    }

    /* Code ASM + Rust */
    .text :
    {
        *(.text*)          /* Rust et ASM .text */
        *(.rodata*)        /* lecture seule */
    }

    .data :
    {
        *(.data*)          /* données initialisées */
    }

    .bss :
    {
        *(COMMON)
        *(.bss*)           /* données non initialisées */
    }
    .data.rel.ro :
    {
        *(.data.rel.ro*)
    }
}

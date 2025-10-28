section .multiboot
align 4
    dd 0x1BADB002 ; MAGIC
    dd (1 << 0) | (1 << 1) ; FLAGS (ALIGN | MEMINFO)
    dd -(0x1BADB002 + ((1 << 0) | (1 << 1))) ; CHECKSUM

section .bss
align 16
stack_bottom:
    resb 65536  ; 64Ko

global stack_top
stack_top:

.hang:
    cli
    hlt
    jmp .hang
section .data
align 8

gdt_start:
    dq 0x0000000000000000      ; Null descriptor
    dq 0x00CF9A000000FFFF      ; Code segment
    dq 0x00CF92000000FFFF      ; Data segment
gdt_end:

global gdt_descriptor
gdt_descriptor:
    dw gdt_end - gdt_start - 1 ; Limit
    dd gdt_start               ; Base

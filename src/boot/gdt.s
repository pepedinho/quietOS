section .data
align 8

gdt_start:
    ; Null
    dd 0x0
    dd 0x0
    ; Code segment
    dw 0xFFFF        ; limit low
    dw 0x0           ; base low
    db 0x0           ; base middle
    db 0x9A          ; access
    db 0xCF          ; granularity + limit high
    db 0x0           ; base high
    ; Data segment
    dw 0xFFFF
    dw 0x0
    db 0x0
    db 0x92          ; access
    db 0xCF          ; granularity
    db 0x0           ; base high
gdt_end:

global gdt_descriptor
gdt_descriptor:
    dw gdt_end - gdt_start - 1 ; Limit
    dd gdt_start               ; Base

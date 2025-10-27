; isr_keyboard.s
global keyboard_stub
extern SCANCODE_BUF     ; symbol in C or ASM
extern SCANCODE_HEAD    ; head index symbol
; Use nasm syntax, no rust/ABI assumptions.

keyboard_stub:
    cli
    pushad
    push ds
    push es

    mov ax, 0x10
    mov ds, ax
    mov es, ax

    ; read scancode
    in al, 0x60

    ; SCANCODE_BUF is base pointer; SCANCODE_HEAD is offset at base+BUF_CAP maybe
    mov edi, SCANCODE_BUF
    mov ebx, [SCANCODE_HEAD]
    mov edx, edi
    add edx, ebx
    mov [edx], al

    inc ebx
    cmp ebx, 1024
    jne .no_wrap
    xor ebx, ebx
.no_wrap:
    mov [SCANCODE_HEAD], ebx

    ; send EOI to PICs
    mov dx, 0xA0
    mov al, 0x20
    out dx, al
    mov dx, 0x20
    mov al, 0x20
    out dx, al

    pop es
    pop ds
    popad
    iret

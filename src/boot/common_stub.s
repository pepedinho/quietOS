global irq_common_stub
extern irq_dispatch

irq_common_stub:
    cli                 ;  stop int
    push eax
    push ecx
    push edx
    push ebx
    push esp
    push ebp
    push esi
    push edi

    ; save segments
    push ds
    push es
    push fs
    push gs

    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    mov eax, [esp + 0x2C] ; retrieve the irq from the stack
    ; pop eax
    push eax
    call irq_dispatch
    add esp, 4          ; clean arg

    cmp eax, 8
    jb .send_master_only
        out 0xA0, al
    .send_master_only:
        mov al, 0x20
        out 0x20, al

    pop gs
    pop fs
    pop es
    pop ds

    pop edi
    pop esi
    pop ebp
    pop esp
    pop ebx
    pop edx
    pop ecx
    pop eax

    sti  ; restore int
    iret
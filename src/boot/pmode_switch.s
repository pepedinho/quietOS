section .text
global _start
extern _entrypoint
extern early_init
extern gdt_descriptor
extern stack_top

_start:
    cli                     ; Désactive interruptions

    ; Charge la GDT
    lgdt [gdt_descriptor]

    ; Active le mode protégé
    mov eax, cr0
    or eax, 0x1
    mov cr0, eax

    ; Far jump pour flush l'instruction queue et mettre CS
    jmp 0x08:protected_entry

protected_entry:
    ; Initialise les segments
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    ; Initialise le stack
    mov esp, stack_top          ; Exemple : stack au-dessus du boot

    call early_init

    sti                         ; Active interruptions
    call _entrypoint           ; Appelle ton entrypoint Rust

.hang:
    cli
    hlt
    jmp .hang


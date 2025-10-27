core::arch::global_asm!(
    r#"
.global keyboard_stub
.extern SCANCODE_BUF
.extern isr_stack_top

keyboard_stub:
    cli                     
    mov esp, isr_stack_top  

    pushad                  
    push ds
    push es

    mov ax, 0x10            
    mov ds, ax
    mov es, ax

    in al, 0x60

    
    mov edi, SCANCODE_BUF
    mov ebx, [edi + 1024]
    mov edx, edi
    add edx, ebx
    mov byte ptr [edx], al
    

    inc ebx
    cmp ebx, 1024
    jne .skip_wrap
    xor ebx, ebx
.skip_wrap:
    mov [edi + 1024], ebx      

    mov dx, 0xA0
    mov al, 0x20
    out dx, al
    mov dx, 0x20
    out dx, al


    pop es
    pop ds
    popad                   
    iret
"#
);

unsafe extern "C" {
    pub unsafe fn keyboard_stub();
}

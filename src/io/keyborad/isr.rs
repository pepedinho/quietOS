use crate::io::AlignedStack;

// core::arch::global_asm!(
//     r#"
// .global keyboard_stub
// keyboard_stub:
//     cli
//     mov eax, irq_stack_top
//     mov esp, eax
//     push 0

//     mov dx, 0x20
//     mov al, 0x20
//     out dx, al

//     pop eax
//     sti
//     iret
// "#
// );

// pub unsafe extern "C" fn keyboard_stub() {}

// static mut IRQ_STACK: AlignedStack = AlignedStack([0; 4096]);

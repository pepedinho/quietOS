section .bss
align 16

isr_stack:
    resb 4096       ; réserve 4 Ko pour la pile ISR

global isr_stack_top
isr_stack_top:      ; label pour la fin de la pile (adresse du sommet)

global irq0_wrapper
global irq1_wrapper
global irq2_wrapper
global irq3_wrapper
global irq4_wrapper
global irq5_wrapper
global irq6_wrapper
global irq7_wrapper
global irq8_wrapper
global irq9_wrapper
global irq10_wrapper
global irq11_wrapper
global irq12_wrapper
global irq13_wrapper
global irq14_wrapper
global irq15_wrapper

extern irq_common_stub

; Macro pour générer les wrappers
%macro DEFINE_IRQ_WRAPPER 2
global %1
%1:
    mov al, %2      ; numéro d'IRQ
    jmp irq_common_stub
%endmacro

; Génération des wrappers
DEFINE_IRQ_WRAPPER irq0_wrapper, 0
DEFINE_IRQ_WRAPPER irq1_wrapper, 33
DEFINE_IRQ_WRAPPER irq2_wrapper, 2
DEFINE_IRQ_WRAPPER irq3_wrapper, 3
DEFINE_IRQ_WRAPPER irq4_wrapper, 4
DEFINE_IRQ_WRAPPER irq5_wrapper, 5
DEFINE_IRQ_WRAPPER irq6_wrapper, 6
DEFINE_IRQ_WRAPPER irq7_wrapper, 7
DEFINE_IRQ_WRAPPER irq8_wrapper, 8
DEFINE_IRQ_WRAPPER irq9_wrapper, 9
DEFINE_IRQ_WRAPPER irq10_wrapper, 10
DEFINE_IRQ_WRAPPER irq11_wrapper, 11
DEFINE_IRQ_WRAPPER irq12_wrapper, 12
DEFINE_IRQ_WRAPPER irq13_wrapper, 13
DEFINE_IRQ_WRAPPER irq14_wrapper, 14
DEFINE_IRQ_WRAPPER irq15_wrapper, 15

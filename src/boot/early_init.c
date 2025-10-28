#include <stdint.h>

static inline void outb(uint16_t port, uint8_t val) {
    __asm__ volatile ("outb %0, %1" : : "a"(val), "Nd"(port));
}
static inline uint8_t inb(uint16_t port) {
    uint8_t ret;
    __asm__ volatile ("inb %1, %0" : "=a"(ret) : "Nd"(port));
    return ret;
}

extern void irq0_wrapper(void);
extern void irq1_wrapper(void);
extern void irq2_wrapper(void);
extern void irq3_wrapper(void);
extern void irq4_wrapper(void);
extern void irq5_wrapper(void);
extern void irq6_wrapper(void);
extern void irq7_wrapper(void);
extern void irq8_wrapper(void);
extern void irq9_wrapper(void);
extern void irq10_wrapper(void);
extern void irq11_wrapper(void);
extern void irq12_wrapper(void);
extern void irq13_wrapper(void);
extern void irq14_wrapper(void);
extern void irq15_wrapper(void);


/* PIC (Programmable Interupt Controller) ports */
#define PIC1_CMD  0x20
#define PIC1_DATA 0x21
#define PIC2_CMD  0xA0
#define PIC2_DATA 0xA1

#define ICW1_INIT 0x10
#define ICW1_ICW4 0x01
#define ICW4_8086 0x01

extern void irq_common_stub(void);

struct idt_entry {
    uint16_t offset_low;
    uint16_t selector;
    uint8_t zero;
    uint8_t type_attr;
    uint16_t offset_high;
} __attribute__((packed));

struct idt_ptr {
    uint16_t limit;
    uint32_t base;
} __attribute__((packed));

static struct idt_entry idt[256];
static struct idt_ptr idt_p;

void set_idt_entry(int vector, void (*handler)(void)) {
    uint32_t addr = (uint32_t) handler;
    idt[vector].offset_low = addr & 0xFFFF;
    idt[vector].selector = 0x08;
    idt[vector].zero = 0;
    idt[vector].type_attr = 0x8E;
    idt[vector].offset_high = (addr >> 16) & 0xFFFF;
}

void lidt(void* idtptr) {
    __asm__ volatile ("lidt (%0)" : : "r"(idtptr));
}

void remap_pic(int offset1, int offset2) {
    uint8_t a1 = inb(PIC1_DATA);
    uint8_t a2 = inb(PIC2_DATA);

    outb(PIC1_CMD, ICW1_INIT | ICW1_ICW4);
    outb(PIC2_CMD, ICW1_INIT | ICW1_ICW4);

    outb(PIC1_DATA, offset1);
    outb(PIC2_DATA, offset2);

    outb(PIC1_DATA, 4);
    outb(PIC2_DATA, 2);

    outb(PIC1_DATA, ICW4_8086);
    outb(PIC2_DATA, ICW4_8086);

    outb(PIC1_DATA, a1);
    outb(PIC2_DATA, a2);
}


extern void keyboard_stub(void);

#define BUF_CAP 1024
uint8_t SCANCODE_BUF[BUF_CAP];
uint32_t SCANCODE_HEAD = 0;

void early_init(void) {
    remap_pic(0x20, 0x28);

    outb(PIC1_DATA, 0xFF);
    outb(PIC2_DATA, 0xFF);

    for (int i = 0; i < 256; i++) {
        set_idt_entry(i, irq_common_stub); 
    }

    set_idt_entry(0x20, irq0_wrapper);
    set_idt_entry(0x21, irq1_wrapper);
    set_idt_entry(0x22, irq2_wrapper);
    set_idt_entry(0x23, irq3_wrapper);
    set_idt_entry(0x24, irq4_wrapper);
    set_idt_entry(0x25, irq5_wrapper);
    set_idt_entry(0x26, irq6_wrapper);
    set_idt_entry(0x27, irq7_wrapper);
    set_idt_entry(0x28, irq8_wrapper);
    set_idt_entry(0x29, irq9_wrapper);
    set_idt_entry(0x2A, irq10_wrapper);
    set_idt_entry(0x2B, irq11_wrapper);
    set_idt_entry(0x2C, irq12_wrapper);
    set_idt_entry(0x2D, irq13_wrapper);
    set_idt_entry(0x2E, irq14_wrapper);
    set_idt_entry(0x2F, irq15_wrapper);

    idt_p.limit = sizeof(idt) - 1;
    idt_p.base  = (uint32_t) &idt;
    lidt(&idt_p);

    outb(PIC1_DATA, 0xFD); /* enable keyboard IRQ only */
    outb(PIC2_DATA, 0xFF);
}

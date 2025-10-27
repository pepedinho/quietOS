// c/early_init.c
#include <stdint.h>

/* I/O port helpers */
static inline void outb(uint16_t port, uint8_t val) {
    __asm__ volatile ("outb %0, %1" : : "a"(val), "Nd"(port));
}
static inline uint8_t inb(uint16_t port) {
    uint8_t ret;
    __asm__ volatile ("inb %1, %0" : "=a"(ret) : "Nd"(port));
    return ret;
}


/* PIC ports */
#define PIC1_CMD  0x20
#define PIC1_DATA 0x21
#define PIC2_CMD  0xA0
#define PIC2_DATA 0xA1

/* ICW */
#define ICW1_INIT 0x10
#define ICW1_ICW4 0x01
#define ICW4_8086 0x01

/* From asm symbol: keyboard_stub */
extern void keyboard_stub(void);

/* Structures for IDT entry */
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

/* We'll allocate IDT here (static) */
static struct idt_entry idt[256];
static struct idt_ptr idt_p;

void set_idt_entry(int vector, void (*handler)(void)) {
    uint32_t addr = (uint32_t) handler;
    idt[vector].offset_low = addr & 0xFFFF;
    idt[vector].selector = 0x08;
    idt[vector].zero = 0;
    idt[vector].type_attr = 0x8E; /* present, DPL=0, 32-bit interrupt gate */
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

    outb(PIC1_DATA, 4); // tell Master about slave at IRQ2
    outb(PIC2_DATA, 2);

    outb(PIC1_DATA, ICW4_8086);
    outb(PIC2_DATA, ICW4_8086);

    outb(PIC1_DATA, a1);
    outb(PIC2_DATA, a2);
}

extern void keyboard_stub(void);

/* Simple scancode buffer in C */
#define BUF_CAP 1024
uint8_t SCANCODE_BUF[BUF_CAP];
uint32_t SCANCODE_HEAD = 0;

void early_init(void) {
    /* Remap PIC to 0x20/0x28 */
    remap_pic(0x20, 0x28);

    /* Mask everything initially */
    outb(PIC1_DATA, 0xFF);
    outb(PIC2_DATA, 0xFF);

    /* Set IDT entry for keyboard (IRQ1 -> vector 0x21) */
    for (int i = 0; i < 256; i++) {
        set_idt_entry(i, (void*)0); /* default empty */
    }
    set_idt_entry(0x21, keyboard_stub);

    idt_p.limit = sizeof(idt) - 1;
    idt_p.base  = (uint32_t) &idt;
    lidt(&idt_p);

    /* Optionally unmask keyboard only, but do NOT enable interrupts here if you want
       to enable later in ASM before calling rust_main. If you want interrupts active
       now, do outb(PIC1_DATA, 0xFD) and sti in ASM BEFORE calling rust_main. */
    outb(PIC1_DATA, 0xFD); /* enable keyboard IRQ only */
    outb(PIC2_DATA, 0xFF);
}

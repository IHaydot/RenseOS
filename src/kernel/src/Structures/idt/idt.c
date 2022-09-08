#include "stdint.h"
#include "stddef.h"

// Extra code

char *formatOutput;
/// everything will be formatted to the previous value
const char *format(const char *a, const char *b, const char *c, const char *d)
{
    // formatOutput = (char*)malloc(sizeof(const char*) * 100);
    if (a == 0)
    {
        return "need more values to format!";
    }
    const char *new_str;
    uint8_t *aPTR = (uint8_t *)a;
    int index = 0;
    while (*aPTR != 0)
    {
        formatOutput[index] = *aPTR;
        aPTR++;
        index++;
    }
    if (b != 0)
    {
        uint8_t *bPTR = (uint8_t *)b;
        while (*bPTR != 0)
        {
            formatOutput[index] = *bPTR;
            bPTR++;
            index++;
        }
    }
    if (c != 0)
    {
        uint8_t *cPTR = (uint8_t *)c;
        while (*cPTR != 0)
        {
            formatOutput[index] = *cPTR;
            cPTR++;
            index++;
        }
    }
    if (d != 0)
    {
        uint8_t *dPTR = (uint8_t *)d;
        while (*dPTR != 0)
        {
            formatOutput[index] = *dPTR;
            dPTR++;
            index++;
        }
    }

    return formatOutput;
}

char IntToStringOutput[128];
const char *IntToString(int value)
{

    uint8_t Neg = 0;
    if (value < 0)
    {
        Neg = 1;
        value *= -1;
        IntToStringOutput[0] = '-';
    }
    uint8_t size = 0;
    uint64_t STester = (uint64_t)value;
    while (STester / 10 > 0)
    {
        STester /= 10;
        size++;
    }
    uint8_t ind = 0;
    uint64_t NewVal = (uint64_t)value;
    while (NewVal / 10 > 0)
    {
        uint8_t remainder = NewVal % 10;
        NewVal /= 10;
        IntToStringOutput[Neg + size - ind] = remainder + 48;
        ind++;
    }
    uint8_t remainder = NewVal % 10;
    IntToStringOutput[Neg + size - ind] = remainder + 48;
    IntToStringOutput[Neg + size + 1] = 0;
    return IntToStringOutput;
}

extern void Cprintln(const char *s);
extern void C__outb(uint16_t port, uint8_t data);
extern uint8_t C__inb(uint16_t port);

// IDT

#define PIC1_COMMAND 0x20
#define PIC1_DATA 0x21
#define PIC2_COMMAND 0xa0
#define PIC2_DATA 0xa1
#define ICW1_INIT 0x10
#define ICW1_ICW4 0x01
#define ICW4_8086 0x01

struct IDT64
{
    uint16_t low_offset;
    uint16_t selector;
    uint8_t ist;
    uint8_t types_n_attr;
    uint16_t mid_offset;
    uint32_t high_offset;
    uint32_t zero;
};

extern struct IDT64 _idt[256] = {};
extern void InitIDTASM();
extern uint64_t isr1;
extern uint64_t general_handler;
extern uint64_t breakpoint_handler;
extern uint64_t division_handler;
extern uint64_t overflow_hanlder;
extern uint64_t double_fault_handler;
extern uint64_t page_fault_handler;

void C__init_idt()
{
    for (int i = 0; i < 257; i++)
    {
        _idt[i].zero = 0;
        _idt[i].low_offset = (uint16_t)(((uint64_t)&general_handler & 0x000000000000ffff));
        _idt[i].mid_offset = (uint16_t)(((uint64_t)&general_handler & 0x00000000ffff0000) >> 16);
        _idt[i].high_offset = (uint32_t)(((uint64_t)&general_handler & 0xffffffff00000000) >> 32);
        _idt[i].ist = 0;
        _idt[i].selector = 0x08;
        _idt[i].types_n_attr = 0x8e;
    }

    _idt[0].zero = 0;
    _idt[0].low_offset = (uint16_t)(((uint64_t)&division_handler & 0x000000000000ffff));
    _idt[0].mid_offset = (uint16_t)(((uint64_t)&division_handler & 0x00000000ffff0000) >> 16);
    _idt[0].high_offset = (uint32_t)(((uint64_t)&division_handler & 0xffffffff00000000) >> 32);
    _idt[0].ist = 0;
    _idt[0].selector = 0x08;
    _idt[0].types_n_attr = 0x8e;

    _idt[3].zero = 0;
    _idt[3].low_offset = (uint16_t)(((uint64_t)&breakpoint_handler & 0x000000000000ffff));
    _idt[3].mid_offset = (uint16_t)(((uint64_t)&breakpoint_handler & 0x00000000ffff0000) >> 16);
    _idt[3].high_offset = (uint32_t)(((uint64_t)&breakpoint_handler & 0xffffffff00000000) >> 32);
    _idt[3].ist = 0;
    _idt[3].selector = 0x08;
    _idt[3].types_n_attr = 0x8e;

    _idt[4].zero = 0;
    _idt[4].low_offset = (uint16_t)(((uint64_t)&overflow_hanlder & 0x000000000000ffff));
    _idt[4].mid_offset = (uint16_t)(((uint64_t)&overflow_hanlder & 0x00000000ffff0000) >> 16);
    _idt[4].high_offset = (uint32_t)(((uint64_t)&overflow_hanlder & 0xffffffff00000000) >> 32);
    _idt[4].ist = 0;
    _idt[4].selector = 0x08;
    _idt[4].types_n_attr = 0x8e;

    _idt[8].zero = 0;
    _idt[8].low_offset = (uint16_t)(((uint64_t)&double_fault_handler & 0x000000000000ffff));
    _idt[8].mid_offset = (uint16_t)(((uint64_t)&double_fault_handler & 0x00000000ffff0000) >> 16);
    _idt[8].high_offset = (uint32_t)(((uint64_t)&double_fault_handler & 0xffffffff00000000) >> 32);
    _idt[8].ist = 0;
    _idt[8].selector = 0x08;
    _idt[8].types_n_attr = 0x8e;

    _idt[0x0e].zero = 0;
    _idt[0x0e].low_offset = (uint16_t)(((uint64_t)&page_fault_handler & 0x000000000000ffff));
    _idt[0x0e].mid_offset = (uint16_t)(((uint64_t)&page_fault_handler & 0x00000000ffff0000) >> 16);
    _idt[0x0e].high_offset = (uint32_t)(((uint64_t)&page_fault_handler & 0xffffffff00000000) >> 32);
    _idt[0x0e].ist = 0;
    _idt[0x0e].selector = 0x08;
    _idt[0x0e].types_n_attr = 0x8e;

    _idt[1].zero = 0;
    _idt[1].low_offset = (uint16_t)(((uint64_t)&isr1 & 0x000000000000ffff));
    _idt[1].mid_offset = (uint16_t)(((uint64_t)&isr1 & 0x00000000ffff0000) >> 16);
    _idt[1].high_offset = (uint32_t)(((uint64_t)&isr1 & 0xffffffff00000000) >> 32);
    _idt[1].ist = 0;
    _idt[1].selector = 0x08;
    _idt[1].types_n_attr = 0x8e;

    uint8_t mask1, mask2;

    mask1 = C__inb(PIC2_DATA);
    mask2 = C__inb(PIC2_DATA);

    C__outb(PIC1_COMMAND, ICW1_INIT | ICW1_ICW4);
    C__outb(PIC2_COMMAND, ICW1_INIT | ICW1_ICW4);
    C__outb(PIC1_DATA, 0);
    C__outb(PIC2_DATA, 8);
    C__outb(PIC1_DATA, 4);
    C__outb(PIC2_DATA, 2);
    C__outb(PIC1_DATA, ICW4_8086);
    C__outb(PIC2_DATA, ICW4_8086);

    C__outb(PIC1_DATA, mask1);
    C__outb(PIC2_DATA, mask2);

    C__outb(0x21, 0xfd);
    C__outb(0xa1, 0xff);

    InitIDTASM();
}

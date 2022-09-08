global LMBegin
extern _start
section .text
bits 64

LMBegin:
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    call activateSSE
    call _start
    jmp $

activateSSE:
    mov rax, cr0
    and ax, 11111101b
    or ax, 00000001b
    mov cr0, rax
    mov rax, cr4
    or ax, 1100000000b
    mov cr4, rax
    ret

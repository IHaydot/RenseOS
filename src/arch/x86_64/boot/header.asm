section .multiboot_header 
start:
    dd 0xE85250D6
    dd 0
    dd end - start
    dd 0x100000000 - (0xE85250D6 + 0 + (end - start))

    dw 0
    dw 0
    dd 8
end:



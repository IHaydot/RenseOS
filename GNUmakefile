final=redoxos-amd64.iso

linker=x86_64-elf-ld

OBJS=$(shell find build/ -name "*.o")

ASM_SRC=$(shell find src/kernel/src/ -name "*.asm")
ASM_OBJ=$(ASM_SRC:.asm=.o)
C_SRC=$(shell find src/kernel/src/ -name "*.c")
C_OBJ=$(C_SRC:.c=.co)

%.o: %.asm
	nasm $< -f elf64 -o $@
%.co: %.c
	x86_64-elf-gcc -c -C src/kernel/src/shared/ $< -ffreestanding -O0 -Wall -o $@

kernel_obj=src/kernel/target/target/debug/libRedoxOS_Kernel.a

process: 
	@make -C src/arch/x86_64 build

iso: process $(OBJS) $(ASM_OBJ) $(C_OBJ)
	@make -C src/kernel/ build

	$(linker) --nmagic --gc-sections -o build/kernel.bin -T src/arch/x86_64/target/linker.ld $(OBJS) $(kernel_obj) $(ASM_OBJ) $(C_OBJ)
	cp build/kernel.bin isofiles/boot
	grub-mkrescue -o $(final) isofiles

run: iso
	qemu-system-x86_64 -cdrom $(final) -m 6G -d cpu_reset -D logs/qemu.log -serial file:logs/serial.log
	make clean

build: iso

.PHONY: run clean build process

clean:
	rm -rf build/* *.o *.img *.iso isofiles/boot/kernel.bin
	rm -rf $(C_OBJ) $(ASM_OBJ) $(OBJS)
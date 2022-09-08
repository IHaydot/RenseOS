
build_dir=../../../build
assembler=nasm
assembler-flags?=-f elf64

ASM_SRC=$(shell find boot -name "*.asm")
ASM_OBJ=${ASM_SRC:.asm=.o}

%.o: %.asm
	$(assembler) $< $(assembler-flags) -o $@
	cp $@ $(build_dir)/$@
	rm -rf $@

setup:
	if [ ! -d $(build_dir) ]; then \
		mkdir $(build_dir); \
	elif [ ! -d $(build_dir)/boot ]; then \
		mkdir $(build_dir)/boot; \
	fi

build: setup $(ASM_OBJ)
	

.PHONY: setup build
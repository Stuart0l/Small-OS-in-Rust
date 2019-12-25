arch	?= x86_64
kernel	:= build/kernel-$(arch).elf
iso	:= build/os-$(arch).iso

linker_script	:= arch/$(arch)/link.ld
grub_cfg	:= arch/$(arch)/grub.cfg
assembly	:= $(shell find ./arch/$(arch)/ -name "*.asm")
object		:= $(assembly:%.asm=%.o)
rust_os		:= target/$(arch)-rsos/debug/librsos.a

.PHONY: all clean run debug iso kernel

all: $(kernel)

clean:
	rm -r build
	rm $(object)

run:
	qemu-system-x86_64 -cdrom $(iso)

debug:
	qemu-system-x86_64 -cdrom $(iso) -s -S

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	mkdir -p build/iso/boot/grub
	cp $(kernel) build/iso/boot/kernel.elf
	cp $(grub_cfg) build/iso/boot/grub
	grub-mkrescue -o $(iso) build/iso
	rm -r build/iso

$(kernel): $(object) $(linker_script) kernel
	mkdir -p build
	ld -n -T $(linker_script) -o $(kernel) $(object) $(rust_os)

kernel:
	cargo xbuild --target $(arch)-rsos.json

%.o: %.asm
	nasm -f elf64 $< -o $@
arch	?= x86_64
kernel	:= build/kernel-$(arch).elf
iso	:= build/os-$(arch).iso

linker_script	:= arch/$(arch)/link.ld
grub_cfg	:= arch/$(arch)/grub.cfg
assembly	:= $(shell find ./arch/$(arch)/ -name "*.asm")
object		:= $(assembly:%.asm=%.o)

.PHONY: all clean run iso

all: $(kernel)

clean:
	rm -r build
	rm $(object)

run:
	qemu-system-x86_64 -cdrom $(iso)

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	mkdir -p build/iso/boot/grub
	cp $(kernel) build/iso/boot/kernel.elf
	cp $(grub_cfg) build/iso/boot/grub
	grub-mkrescue -o $(iso) build/iso
	rm -r build/iso

$(kernel): $(object) $(linker_script)
	mkdir -p build
	ld -n -T $(linker_script) -o $(kernel) $(object)

%.o: %.asm
	nasm -f elf64 $< -o $@
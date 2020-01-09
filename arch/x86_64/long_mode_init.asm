global real64
extern stack_top
KERNEL_VIRT_START equ 0xffffffff80000000

section .boottext
bits 64
real64:
	mov ax, 0
	mov ss, ax
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax

	mov rsp, stack_top + KERNEL_VIRT_START

	extern _start
	call _start

	mov rax, 0x2f592f412f4b2f4f
	mov qword [0xb8000 + KERNEL_VIRT_START], rax
	hlt

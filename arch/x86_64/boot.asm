global start, stack_top
extern real64

%define pgd_index(addr) (((addr >> 39)) & 0x1ff)
%define	pud_index(addr) (((addr >> 30)) & 0x1ff)

MAGIC_VALUE equ 0x36d76289
KERNEL_VIRT_START equ 0xffffffff80000000

section .bootbss
align 4096
; +-------------+------------+-------------+-------------+----------+
; |global dir(9)|upper dir(9)|middle dir(9)|page table(9)|offset(12)|
; +-------------+------------+-------------+-------------+----------+
global_dir:
	times 4096 db 0
upper_dir_low:
	times 4096 db 0
upper_dir_high:
	times 4096 db 0
middle_dir_low:
	times 4096 db 0
middle_dir_high:
	times 4096 db 0
page_table:
	times 4096 db 0
stack_bottom:
	times 4096 * 4 db 0
stack_top:

section .bootrodata
gdt64:
.null:
	dq 0	; zero entry
.code: equ $ - gdt64
	dw 0
	dw 0
	db 0
	db 10011000b	; present, ring0, code/data, code
	db 00101111b	; long mode
	db 0
.data: equ $ - gdt64
	dw 0
	dw 0
	db 0
	db 10010010b	; present, code/data, data
	db 00000000b
	db 0
.pointer:
	dw $ - gdt64 - 1
	dq gdt64

section .boottext
bits 32
start:
	mov esp, stack_top
	mov edi, ebx	; multiboot information

	call check_multiboot
	call check_cpuid
	call check_long_mode

	call setup_page_tables
	call enable_paging

	lgdt [gdt64.pointer]
	jmp gdt64.code:real64	; far jump to real64, change cs to base address in gdt.code

	mov dword [0xb8000], 0x2f4b2f4f
	hlt

error:
	mov dword [0xb8000], 0x4f524f45
	mov dword [0xb8004], 0x4f3a4f52
	mov dword [0xb8008], 0x4f204f20
	mov byte  [0xb800a], al
	hlt

check_multiboot:
	cmp eax, MAGIC_VALUE
	jne .no_multiboot
	ret
.no_multiboot:
	mov al, "0"
	jmp error

check_cpuid:
	; Check if CPUID is supported by attempting to flip the ID bit (bit 21) in
	; the FLAGS register. If we can flip it, CPUID is available.

	; Copy FLAGS in to EAX via stack
	pushfd
	pop eax

	; Copy to ECX as well for comparing later on
	mov ecx, eax

	; Flip the ID bit
	xor eax, 1 << 21

	; Copy EAX to FLAGS via the stack
	push eax
	popfd

	; Copy FLAGS back to EAX (with the flipped bit if CPUID is supported)
	pushfd
	pop eax

	; Restore FLAGS from the old version stored in ECX (i.e. flipping the ID bit
	; back if it was ever flipped).
	push ecx
	popfd

	; Compare EAX and ECX. If they are equal then that means the bit wasn't
	; flipped, and CPUID isn't supported.
	xor eax, ecx
	jz .no_cpuid
	ret
.no_cpuid:
	mov al, "1"
	jmp error

check_long_mode:
	mov eax, 0x80000001	; Set the A-register to 0x80000001.
	cpuid			; CPU identification.
	test edx, 1 << 29	; Test if the LM-bit, which is bit 29, is set in the D-register.
	jz .no_long_mode	; They aren't, there is no long mode.
	ret
.no_long_mode:
	mov al, "2"
	jmp error

setup_page_tables:
	; map lower half: 0x0000000000000000 ~ 0x000000003fffffff
	mov eax, upper_dir_low
	or eax, 11b	; present | writable
	mov [global_dir + pgd_index(0) * 8], eax

	mov eax, middle_dir_low
	or eax, 11b	; present | writable
	mov [upper_dir_low + pud_index(0) * 8], eax

	; map higher half: 0xffffffff80000000 ~ 0xffffffffbfffffff
	mov eax, upper_dir_high
	or eax, 11b	; present | writable
	mov [global_dir + pgd_index(KERNEL_VIRT_START) * 8], eax

	mov eax, middle_dir_high
	or eax, 11b	; present | writable
	mov [upper_dir_high + pud_index(KERNEL_VIRT_START) * 8], eax

	mov ecx, 512
	mov edi, middle_dir_low
	mov esi, middle_dir_high
	mov eax, 0x0
	or eax, 10000011b ; present | writable | huge 
.map_middle_dir:
	mov [edi], eax
	mov [esi], eax
	add eax, 0x200000
	add edi, 8
	add esi, 8
	loop .map_middle_dir
	ret

enable_paging:
	; set page global directory start address
	mov eax, global_dir
	mov cr3, eax

	; enable PAE
	mov eax, cr4
	or eax, 1 << 5
	mov cr4, eax

	mov ecx, 0xC0000080	; Set the C-register to 0xC0000080, which is the EFER MSR.
	rdmsr			; Read from the model-specific register.
	or eax, 1 << 8		; Set the LM-bit which is the 9th bit (bit 8).
	wrmsr			; Write to the model-specific register.

	; enable paging
	mov eax, cr0
	or eax, 1 << 31
	mov cr0, eax
	ret

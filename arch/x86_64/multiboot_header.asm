MAGIC_NUMBER equ 0xe85250d6

section .multiboot_header
header_start:
	dd MAGIC_NUMBER			; magic number
	dd 0				; architecture (i386)
	dd header_end - header_start	; header length
	; checksum
	dd -(MAGIC_NUMBER + 0 + (header_end - header_start))

	; tags
	dw 0	; type
	dw 0	; flags
	dd 8	; size
header_end:
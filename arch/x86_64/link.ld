ENTRY(start)

KERNEL_VIRT_START = 0xffffffff80000000;

SECTIONS {
	. = 0x00100000;

	.boot ALIGN (4096) :
	{
		KEEP(* (.multiboot_header))
		* (.bootbss)
		* (.boottext)
		* (.bootrodata)
	}

	. += KERNEL_VIRT_START;

	.text ALIGN (4096) : AT(ADDR(.text) - KERNEL_VIRT_START)
	{
		*(.text .text.*)
	}

	.rodata ALIGN (4096) : AT(ADDR(.rodata) - KERNEL_VIRT_START)
	{
		*(.rodata .rodata.*)
	}

	.data.rel.ro ALIGN (4096) : AT(ADDR(.data.rel.ro) - KERNEL_VIRT_START)
	{
		*(.data.rel.ro.local*) *(.data.rel.ro .data.rel.ro.*)
	}

	.bss ALIGN (4096) : AT(ADDR(.bss) - KERNEL_VIRT_START)
	{
		*(COMMON)
		*(.bss)
	}
}

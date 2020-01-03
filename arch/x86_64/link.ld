ENTRY(start)

SECTIONS {
	. = 0x00100000;

	.boot :
	{
		KEEP(*(.multiboot_header))
	}

	.text :
	{
		*(.text .text.*)
	}

	.rodata :
	{
		*(.rodata .rodata.*)
	}

	.data.rel.ro :
	{
		*(.data.rel.ro.local*) *(.data.rel.ro .data.rel.ro.*)
	}
}
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use spin::Mutex;
use crate::serial_println;

static IDT: Mutex<InterruptDescriptorTable> = Mutex::new({
	let idt = InterruptDescriptorTable::new();
	idt
});

pub fn setup_idt()
{
	use x86_64::instructions::tables::{lidt, DescriptorTablePointer};
	use core::mem::size_of;
	
	let ptr = DescriptorTablePointer {
		limit: (size_of::<InterruptDescriptorTable>() - 1) as u16,
		base: &*IDT.lock() as *const InterruptDescriptorTable as u64,
	};

	unsafe { lidt(&ptr) };
}

pub fn idt_set()
{
	IDT.lock().breakpoint.set_handler_fn(int3);
	IDT.lock().double_fault.set_handler_fn(doublefault_fn);
}

extern "x86-interrupt" fn int3(stack_frame: &mut InterruptStackFrame)
{
	serial_println!("EXCEPTION: breakpoint\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn doublefault_fn(stack_frame: &mut InterruptStackFrame, _error_code: u64) -> !
{
	panic!("EXCEPTION: double fault\n{:#?}", stack_frame);
}

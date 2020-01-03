use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;
use crate::serial_println;

lazy_static! {
	static ref IDT: InterruptDescriptorTable = {
		let mut idt = InterruptDescriptorTable::new();
		idt.breakpoint.set_handler_fn(int3);
		idt.double_fault.set_handler_fn(doublefault_fn);
		idt
	};
}

pub fn init()
{
	IDT.load();
}

extern "x86-interrupt" fn int3(stack_frame: &mut InterruptStackFrame)
{
	serial_println!("EXCEPTION: breakpoint\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn doublefault_fn(stack_frame: &mut InterruptStackFrame, _error_code: u64) -> !
{
	panic!("EXCEPTION: double fault\n{:#?}", stack_frame);
}

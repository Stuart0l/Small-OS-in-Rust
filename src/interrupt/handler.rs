use x86_64::structures::idt::InterruptStackFrame;
use crate::{print, serial_println};
use crate::interrupt::pic;

pub extern "x86-interrupt" fn int3(stack_frame: &mut InterruptStackFrame)
{
	serial_println!("EXCEPTION: breakpoint\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn doublefault_fn(stack_frame: &mut InterruptStackFrame, _error_code: u64) -> !
{
	panic!("EXCEPTION: double fault\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn timer_handler(stack_frame: &mut InterruptStackFrame)
{
	print!(".");
	pic::notify_eoi(32);
}

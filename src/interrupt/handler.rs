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

pub extern "x86-interrupt" fn timer_handler(_stack_frame: &mut InterruptStackFrame)
{
	print!(".");
	pic::notify_eoi(32);
}

pub extern "x86-interrupt" fn keyboard_handler(_stack_frame: &mut InterruptStackFrame)
{
	use x86_64::instructions::port::Port;
	use pc_keyboard::{Keyboard, ScancodeSet1, DecodedKey, layouts, HandleControl};
	use spin::Mutex;
	use lazy_static::lazy_static;

	lazy_static! {
		static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
			Mutex::new(Keyboard::new(layouts::Us104Key,
						 ScancodeSet1,
						 HandleControl::Ignore));
	}

	let mut keyboard = KEYBOARD.lock();
	let mut kbd_port: Port<u8> = Port::new(0x0060);
	let scancode: u8 = unsafe { kbd_port.read() };
	
	if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
		if let Some(key) = keyboard.process_keyevent(key_event) {
			match key {
				DecodedKey::RawKey(key) => print!("{:?}", key),
				DecodedKey::Unicode(character) => print!("{}", character),
			}
		}
	}

	pic::notify_eoi(33);
}

#![no_std]

extern crate spin;
extern crate lazy_static;
extern crate x86_64;
extern crate uart_16550;

mod vga_buffer;
mod serial;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start()
{
	serial_println!("hello world{}", '!');
	//panic!("some message");

	loop {};
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !
{
	serial_println!("[Failed]\n");
	serial_println!("{}", _info);
	exit_qemu(QemuExitCode::Failed);
	loop {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
	Success = 0x10,
	Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode)
{
	use x86_64::instructions::port::Port;

	unsafe {
		let mut port = Port::new(0xf4);
		port.write(exit_code as u32);
	}
}

#![no_std]

extern crate spin;
extern crate lazy_static;

mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start()
{
	println!("hello world{}", '!');

	loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}

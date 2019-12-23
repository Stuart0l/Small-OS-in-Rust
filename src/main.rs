#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start()
{
	let hello_b = b"hello world!";
	let hello:[u8; 12] = [104,101,108,108,111,32,119,111,114,108,100,33];  // hello world!
	let color_byte: u8 = 0x1f;
	let buffer_ptr = (0xb8000) as *mut u8;

	for (i, &char_byte) in hello_b.iter().enumerate() {
		unsafe {
			*buffer_ptr.offset(i as isize * 2) = char_byte;
			*buffer_ptr.offset(i as isize * 2 + 1) = color_byte;
		}
	}

	loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}

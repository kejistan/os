#![no_main]
#![no_std]

mod framebuffer;

use bootloader_api::{entry_point, BootInfo};
use core::panic::PanicInfo;
use framebuffer::FrameBuffer;

entry_point!(start);

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}

pub fn start(boot_info: &'static mut BootInfo) -> ! {
	if let Some(raw_framebuffer) = boot_info.framebuffer.as_mut() {
		let mut framebuffer = FrameBuffer::new(raw_framebuffer);

		framebuffer.clear();
	}

	loop {}
}

#![feature(abi_x86_interrupt)]
#![no_main]
#![no_std]

mod gdt;
mod interrupts;
mod logger;
mod vga;

use bootloader_api::{entry_point, BootInfo};
use core::panic::PanicInfo;
use vga::Vga;

use crate::logger::LOGGER;

entry_point!(start);

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!("{}", info);
	halt();
}

pub fn start(boot_info: &'static mut BootInfo) -> ! {
	if let Some(raw_framebuffer) = boot_info.framebuffer.as_mut() {
		let mut vga = Vga::new(raw_framebuffer);
		vga.clear();

		LOGGER.lock().set_vga(vga);
	}

	gdt::init();
	interrupts::init();

	println!("Hello world!");
	x86_64::instructions::interrupts::enable();

	// x86_64::instructions::interrupts::int3();

	halt();
}

fn halt() -> ! {
	println!("Stopped.");

	loop {
		x86_64::instructions::hlt();
	}
}

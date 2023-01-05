use core::fmt;

use spin::Mutex;
use x86_64::instructions::interrupts::without_interrupts;

use crate::vga::Vga;

use self::vga_text::VgaText;

mod vga_text;

pub static LOGGER: Mutex<Logger> = Mutex::new(Logger::new());

pub struct Logger {
	vga: Option<VgaText>,
}

impl Logger {
	pub const fn new() -> Self {
		Self { vga: None }
	}

	pub fn set_vga(&mut self, vga: Vga) {
		self.vga = Some(VgaText::new(vga));
	}
}

impl fmt::Write for Logger {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		if let Some(vga) = &mut self.vga {
			for c in s.chars() {
				vga.write_char(c);
			}
		}

		Ok(())
	}
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::logger::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
	use core::fmt::Write;

	without_interrupts(|| {
		LOGGER.lock().write_fmt(args).unwrap();
	});
}

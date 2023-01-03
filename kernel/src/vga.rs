use core::cmp::max;

use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use volatile::Volatile;

pub struct Pixel {
	pub r: u8,
	pub g: u8,
	pub b: u8,
}

pub struct Vga {
	info: FrameBufferInfo,
	buffer: Volatile<&'static mut [u8]>,
}

impl Vga {
	pub fn new(frame_buffer: &'static mut bootloader_api::info::FrameBuffer) -> Self {
		Self {
			info: frame_buffer.info(),
			buffer: Volatile::new(frame_buffer.buffer_mut()),
		}
	}

	pub fn height(&self) -> usize {
		self.info.height
	}

	pub fn width(&self) -> usize {
		self.info.width
	}

	pub fn clear(&mut self) {
		self.buffer.fill(0);
	}

	pub fn draw_pixel_at(&mut self, pixel: &Pixel, x: usize, y: usize) {
		let pixel_addr =
			self.info.stride * self.info.bytes_per_pixel * y + self.info.bytes_per_pixel * x;

		match self.info.pixel_format {
			PixelFormat::Bgr => {
				self.buffer
					.index_mut(pixel_addr..(pixel_addr + 3))
					.copy_from_slice(&[pixel.b, pixel.g, pixel.r]);
			}
			PixelFormat::Rgb => {
				self.buffer
					.index_mut(pixel_addr..(pixel_addr + 3))
					.copy_from_slice(&[pixel.r, pixel.g, pixel.b]);
			}
			PixelFormat::U8 => {
				self.buffer
					.index_mut(pixel_addr)
					.write(max(max(pixel.r, pixel.g), pixel.b));
			}
			PixelFormat::Unknown {
				red_position,
				green_position,
				blue_position,
			} => {
				self.buffer
					.index_mut(pixel_addr + red_position as usize)
					.write(pixel.r);
				self.buffer
					.index_mut(pixel_addr + green_position as usize)
					.write(pixel.g);
				self.buffer
					.index_mut(pixel_addr + blue_position as usize)
					.write(pixel.b);
			}
			_ => {
				// Pick a pixel format and hope it'll work for printing the panic message
				self.info.pixel_format = PixelFormat::U8;
				panic!();
			}
		}
	}

	pub fn shift_y(&mut self, y: isize) {
		let window_size = y.abs() as usize * self.info.stride * self.info.bytes_per_pixel;
		let buffer_size = self.info.byte_len;

		if y > 0 {
			self.buffer
				.copy_within(0..(buffer_size - window_size), window_size);
			self.buffer.index_mut(0..window_size).fill(0);
		} else {
			self.buffer.copy_within(window_size..buffer_size, 0);
			self.buffer
				.index_mut((buffer_size - window_size)..buffer_size)
				.fill(0);
		}
	}
}

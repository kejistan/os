use bootloader_api::info::FrameBufferInfo;
use volatile::{access::WriteOnly, Volatile};

pub struct FrameBuffer {
	info: FrameBufferInfo,
	buffer: Volatile<&'static mut [u8], WriteOnly>,
}

impl FrameBuffer {
	pub fn new(frame_buffer: &'static mut bootloader_api::info::FrameBuffer) -> Self {
		Self {
			info: frame_buffer.info(),
			buffer: Volatile::new_write_only(frame_buffer.buffer_mut()),
		}
	}

	pub fn clear(&mut self) {
		self.buffer.fill(0);
	}
}

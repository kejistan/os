use noto_sans_mono_bitmap::{get_raster, get_raster_width, FontWeight, RasterHeight};

use crate::vga::{Pixel, Vga};

const BACKUP_CHAR: char = '?';
const STYLE: FontWeight = FontWeight::Regular;
const SIZE: RasterHeight = RasterHeight::Size16;
const LINE_SPACING: usize = 2;

pub struct VgaText {
	vga: Vga,
	position: (usize, usize),

	margins: (usize, usize),
	max_lines: usize,
	max_cols: usize,
}

impl VgaText {
	pub fn new(vga: Vga) -> Self {
		let line_height = LINE_SPACING + SIZE as usize;
		let max_lines = vga.height() / line_height;
		let extra_vertical_space = vga.height() % line_height;

		let character_width = get_raster_width(STYLE, SIZE);
		let max_cols = vga.width() / character_width;
		let extra_horizontal_space = vga.width() % character_width;

		let margins = (extra_horizontal_space / 2, extra_vertical_space / 2);

		Self {
			vga,
			position: (0, 0),

			margins,
			max_lines,
			max_cols,
		}
	}

	pub fn write_char(&mut self, character: char) {
		match character {
			'\n' => self.newline(),
			c => {
				self.position.0 += 1;
				if self.position.0 == self.max_cols {
					self.newline();
				}

				let pixel_x = self.position.0 * self.character_width() + self.margins.0;
				let pixel_y = self.position.1 * (LINE_SPACING + SIZE as usize) + self.margins.1;

				self.draw_char_at(c, pixel_x, pixel_y);
			}
		}
	}

	fn newline(&mut self) {
		self.position.0 = 0;
		self.position.1 += 1;
		if self.position.1 == self.max_lines {
			self.scroll_one_line();
			self.position.1 -= 1;
		}
	}

	fn character_width(&self) -> usize {
		get_raster_width(STYLE, SIZE)
	}

	fn draw_char_at(&mut self, character: char, x: usize, y: usize) {
		let raster = get_raster(character, STYLE, SIZE).unwrap_or_else(|| {
			get_raster(BACKUP_CHAR, STYLE, SIZE).expect("Failed to rasterize backup character")
		});
		for (y_offset, row) in raster.raster().iter().enumerate() {
			for (x_offset, pixel) in row.iter().enumerate() {
				self.vga.draw_pixel_at(
					&Pixel {
						r: *pixel,
						g: *pixel,
						b: *pixel,
					},
					x + x_offset,
					y + y_offset,
				);
			}
		}
	}

	fn scroll_one_line(&mut self) {
		self.vga.shift_y(-(LINE_SPACING as isize + SIZE as isize));
	}
}

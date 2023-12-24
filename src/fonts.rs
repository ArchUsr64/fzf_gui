use anyhow::{anyhow, Error, Result};

pub struct Font {
	pub width: usize,
	pub height: usize,
	pub glyphs: [[Vec<u8>; 32]; 3],
}

impl Font {
	/// Parses a pbm image file as the font atlas.
	/// The atlas should be `32` glyphs wide and `3` glyphs tall, starting at
	/// Space (' ') in standard ASCII ordering
	pub fn from_pbm(bytes: &[u8]) -> Result<Self> {
		let (image_width, image_height, pixel_data) = Self::parse_pbm(bytes)?;
		if image_width % 32 != 0 {
			return Err(anyhow!("Font atlas width is invalid: {}", image_width));
		}
		if image_height % 3 != 0 {
			return Err(anyhow!("Font atlas height is invalid: {}", image_height));
		}
		let (glyph_width, glyph_height) = (image_width / 32, image_height / 3);
		println!("{glyph_width}, {glyph_height}, {image_width}, {image_height}");
		// TODO: Pre allocate glyph bitmaps here
		let mut glyphs: [[Vec<u8>; 32]; 3] = Default::default();
		for (j, row) in glyphs.iter_mut().enumerate() {
			for (i, cell) in row.iter_mut().enumerate() {
				let top_left = i as usize * glyph_width + j as usize * glyph_height * image_width;
				for j in 0..glyph_height {
					for i in 0..glyph_width {
						cell.push(pixel_data[top_left + i + j * image_width]);
					}
				}
			}
		}
		Ok(Self {
			width: glyph_width,
			height: glyph_height,
			glyphs,
		})
	}

	fn parse_pbm(bytes: &[u8]) -> Result<(usize, usize, Vec<u8>)> {
		let mut i = 0;
		let mut bitmap_offset = 0;
		let mut result = (
			Err(Error::msg("width")),
			Err(Error::msg("height")),
			Err(Error::msg("bitmap")),
		);
		for data in bytes.split(|i| *i == b'\n') {
			if i < 2 {
				// + 1 to account for the '\n' in between
				bitmap_offset += data.len() + 1;
			}
			if let Some(b'#') = data.first() {
				continue;
			}
			match i {
				0 => {
					if data != b"P4" {
						Err(anyhow!("Magic Word 'P4' not found"))?
					}
				}
				1 => {
					for (i, value) in data.split(|i| *i == b' ').enumerate() {
						if i >= 2 {
							Err(anyhow!("Failed to parse image width and height"))?;
						}
						let value: usize = std::str::from_utf8(value)?.parse()?;
						if i == 0 {
							result.0 = Ok(value);
						}
						if i == 1 {
							result.1 = Ok(value);
						}
					}
				}
				_ => break,
			}
			i += 1;
		}
		let bitmap = &bytes[bitmap_offset..];
		let bitmap_decompressed = bitmap
			.iter()
			.flat_map(|i| {
				[
					i & 0x80,
					i & 0x40,
					i & 0x20,
					i & 0x10,
					i & 8,
					i & 4,
					i & 2,
					i & 1,
				]
			})
			.map(|i| if i != 0 { 255 } else { 0 })
			.collect();
		// TODO: Handle the case when number of pixels isn't multiple of 8
		result.2 = Ok(bitmap_decompressed);
		Ok((result.0?, result.1?, result.2?))
	}
}

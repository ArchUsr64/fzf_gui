use crate::events::{Event, Keycode, Modifiers};
use crate::fonts::Font;
use crate::picker::Picker;
use log::{debug, log_enabled, Level};

/// Responsible for event handling and drawing to screen
pub struct App {
	// Some internal state
	picker: Picker,
	font: Font,
	running: bool,
}

impl App {
	pub fn new(font: Font, options: Vec<String>) -> Self {
		App {
			font,
			picker: Picker::new(options),
			running: true,
		}
	}
	pub fn handle_events(&mut self, event: Event) {
		debug!("{:?}", event);
		match event {
			Event::Focused(false) => {
				if !log_enabled!(Level::Debug) {
					self.close();
				}
			}
			Event::Keyboard {
				keycode: Keycode::Escape,
				..
			}
			| Event::Keyboard {
				keycode: Keycode::c,
				modifiers: Modifiers { ctrl: true, .. },
				..
			} => self.close(),
			Event::Keyboard {
				modifiers,
				keycode,
				utf8,
			} => {
				let search = &mut self.picker.search;
				let special_modifiers = modifiers.ctrl | modifiers.alt | modifiers.logo;
				if let Some(Some(ch)) = utf8.map(|i| i.chars().last()) {
					if !special_modifiers && (' '..='~').contains(&ch) {
						search.insert(ch);
					}
				}
				// Ctrl + <T> keycodes
				if modifiers.ctrl {
					match keycode {
						Keycode::a => search.cursor_to_start(),
						Keycode::e => search.cursor_to_end(),
						Keycode::w => search.delete_word(),
						Keycode::u => search.delete_till_start(),
						Keycode::k => search.delete_till_end(),
						Keycode::b => search.cursor_left(),
						Keycode::f => search.cursor_right(),
						_ => (),
					}
				}
				match keycode {
					Keycode::BackSpace => search.delete(),
					Keycode::Left => search.cursor_left(),
					Keycode::Right => search.cursor_right(),
					_ => (),
				}
			}
			_ => (),
		}
	}
	pub fn draw(&mut self, canvas: &mut [u8], width: u32, height: u32) {
		let line_count = height as usize / self.font.height;
		// 24-bit colors in ARGB format
		const BACKGROUND: u32 = 0xffffffff;
		canvas.chunks_exact_mut(4).for_each(|chunk| {
			let array: &mut [u8; 4] = chunk.try_into().unwrap();
			*array = BACKGROUND.to_le_bytes();
		});
		let mut draw_line = |index, text: &str| {
			let top_line = index * width as usize * self.font.height * 4;
			for (i, symbol) in text.char_indices() {
				let glyph = self.font.get_glyph(symbol).expect("Symbol is not ASCII");
				let top_left = top_line + i * self.font.width * 4;
				for j in 0..self.font.height {
					for i in 0..self.font.width {
						let index = top_left + 4 * (i + j * width as usize);
						let pixel_value = glyph[i + j * self.font.width];
						canvas[index] = pixel_value;
						canvas[index + 1] = pixel_value;
						canvas[index + 2] = pixel_value;
					}
				}
			}
		};
		draw_line(0, self.picker.query());
		// TODO: Handle text and cursor rendering when the text width is greater than canvas width
		self.picker.update();
		// -1 since one line is taken by search
		self.picker
			.get_matches(line_count - 1)
			.enumerate()
			.for_each(|(i, r#match)| draw_line(i + 1, r#match));
		// Render the cursor
		for i in 0..self.font.height {
			let index = 4 * (self.picker.cursor() * self.font.width + i * width as usize);
			canvas[index] = 0x00;
			canvas[index + 1] = 0x00;
			canvas[index + 2] = 0x00;
		}
	}
	pub fn running(&self) -> bool {
		self.running
	}
	pub fn close(&mut self) {
		self.running = false;
	}
}

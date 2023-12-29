use crate::events::{Event, Keycode, Modifiers};
use crate::fonts::Font;

/// Responsible for event handling and drawing to screen
pub struct App {
	// Some internal state
	font: Font,
	query: String,
	cursor: usize,
	running: bool,
}

impl App {
	pub fn new(font: Font) -> Self {
		App {
			font,
			query: "".to_string(),
			cursor: 0,
			running: true,
		}
	}
	pub fn handle_events(&mut self, event: Event) {
		println!("{:?}", event);
		match event {
			Event::Focused(false)
			| Event::Keyboard {
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
				let special_modifiers = modifiers.ctrl | modifiers.alt | modifiers.logo;
				if let Some(Some(text)) = utf8.map(|i| i.chars().last()) {
					if !special_modifiers && (' '..='~').contains(&text) {
						let (left, right) = self.query.split_at(self.cursor);
						let mut new_query = String::from(left);
						new_query.push(text);
						new_query.push_str(right);
						self.query = new_query;
						self.cursor += 1;
					}
				}
				// Ctrl + <T> keycodes
				if modifiers.ctrl {
					match keycode {
						Keycode::a => self.cursor = 0,
						Keycode::e => self.cursor = self.query.len(),
						// TODO: Fix deletion when multiple consecutive space
						Keycode::w => loop {
							if self.cursor == 0 {
								break;
							}
							if let Some(' ') = self.query.pop() {
								self.cursor -= 1;
								break;
							}
							self.cursor -= 1;
						},
						Keycode::u => {
							let (_, right) = self.query.split_at(self.cursor);
							self.query = right.to_string();
							self.cursor = 0;
						}
						Keycode::k => {
							let (left, _) = self.query.split_at(self.cursor);
							self.query = left.to_string();
						}
						Keycode::l => {
							self.query.clear();
							self.cursor = 0;
						}
						_ => (),
					}
				}
				match keycode {
					Keycode::BackSpace => {
						if self.cursor > 0 {
							self.cursor -= 1;
							let (left, right) = self.query.split_at(self.cursor);
							self.query = format!("{left}{}", &right[1..]);
						}
					}
					Keycode::Left => self.cursor = self.cursor.saturating_sub(1),
					Keycode::Right => {
						self.cursor += 1;
						if self.cursor > self.query.len() {
							self.cursor = self.query.len();
						}
					}
					_ => (),
				}
			}
			_ => (),
		}
		println!("{}, {}", self.query, self.cursor);
	}
	pub fn draw(&mut self, canvas: &mut [u8], width: u32, _height: u32) {
		// 24-bit colors in ARGB format
		const BACKGROUND: u32 = 0xffffffff;
		canvas.chunks_exact_mut(4).for_each(|chunk| {
			let array: &mut [u8; 4] = chunk.try_into().unwrap();
			*array = BACKGROUND.to_le_bytes();
		});
		// TODO: Handle text and cursor rendering when the text width is greater than canvas width
		for (i, symbol) in self.query.char_indices() {
			let glyph = self.font.get_glyph(symbol).expect("Symbol is not ASCII");
			let top_left = i * self.font.width * 4;
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
		// Render the cursor
		for i in 0..self.font.height {
			let index = 4 * (self.cursor * self.font.width + i * width as usize);
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

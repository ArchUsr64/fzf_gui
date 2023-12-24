use crate::events::{Event, Keycode, Modifiers};

/// Responsible for event handling and drawing to screen
pub struct App {
	// Some internal state
	query: String,
	cursor: usize,
	running: bool,
	tick: u32,
}

impl App {
	pub fn new() -> Self {
		App {
			query: "".to_string(),
			cursor: 0,
			running: true,
			tick: 0,
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
				if let Some(text) = utf8 {
					if !special_modifiers && !text.is_empty() {
						let (left, right) = self.query.split_at(self.cursor);
						let mut new_query = String::from(left);
						new_query.push_str(&text);
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
	pub fn draw(&mut self, canvas: &mut [u8], width: u32, height: u32) {
		canvas
			.chunks_exact_mut(4)
			.enumerate()
			.for_each(|(index, chunk)| {
				let x = ((index as usize) % width as usize) as u32;
				let y = (index / width as usize) as u32;

				let a = self.tick % 0xFF;
				let r = u32::min(((width - x) * 0xFF) / width, ((height - y) * 0xFF) / height);
				let g = u32::min((x * 0xFF) / width, ((height - y) * 0xFF) / height);
				let b = u32::min(((width - x) * 0xFF) / width, (y * 0xFF) / height);
				let color = (a << 24) + (r << 16) + (g << 8) + b;

				let array: &mut [u8; 4] = chunk.try_into().unwrap();
				*array = color.to_le_bytes();
			});
		self.tick += 1;
	}
	pub fn running(&self) -> bool {
		self.running
	}
	pub fn close(&mut self) {
		self.running = false;
	}
}

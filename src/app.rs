use crate::events::{Event, Keycode};

/// Responsible for event handling and drawing to screen
pub struct App {
	// Some internal state
	running: bool,
	tick: u32,
}

impl App {
	pub fn new() -> Self {
		App {
			running: true,
			tick: 0,
		}
	}
	pub fn handle_events(&mut self, event: Event) {
		println!("{event:?}");
		if let Event::Keyboard {
			keycode: Keycode::Escape,
			..
		} = event
		{
			self.running = false
		}
		if let Event::Keyboard {
			keycode: Keycode::space,
			..
		} = event
		{
			self.tick = 0
		}
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

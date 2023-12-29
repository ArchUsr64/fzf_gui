mod app;
mod events;
mod fonts;
mod picker;
mod window;
use app::App;
use atty::Stream;
use fonts::Font;
use window::Window;

use log::{debug, log_enabled, Level};

/// The height of the glyphs in pixels
const FONT_SIZE: usize = 20;
/// Size of the window in number of glyphs
const WINDOW_SIZE: (usize, usize) = (80, 20);

fn main() {
	env_logger::init();

	let mut options = Vec::new();

	if atty::is(Stream::Stdin) {
		let dir_reader = std::fs::read_dir(".").unwrap();
		for i in dir_reader {
			if let Ok(Ok(dir)) = i.map(|dir| dir.file_name().into_string()) {
				options.push(dir);
			}
		}
	} else {
		let stdin = std::io::stdin();
		for line in stdin.lines() {
			if let Ok(line) = line {
				options.push(line);
			}
		}
	}

	let font = Font::from_pbm(include_bytes!("res/font_atlas.pbm"), FONT_SIZE).unwrap();

	if log_enabled!(Level::Debug) {
		for ch in ' '..='~' {
			let i = ch as usize;
			let g = &font.glyphs[(i >> 5) - 1][i & 0x1f];
			let mut buf = String::new();
			for j in 0..font.height {
				for i in 0..font.width {
					if g[i + j * font.width] != 0xff {
						buf += format!("{:02x}", g[i + j * font.width]).as_str();
					} else {
						buf += "  ";
					}
				}
				buf += "\n";
			}
			debug!("Symbol: {ch}");
			debug!("\n{buf}");
		}
	} // We don't draw immediately, the configure will notify us when to first draw.
	let (mut window, mut event_queue) = Window::new(
		(WINDOW_SIZE.0 * FONT_SIZE) as u32,
		// +2 for rendering the top and bottom borders (1px each)
		(WINDOW_SIZE.1 * FONT_SIZE) as u32 + 2,
		App::new(font, options),
	);

	loop {
		event_queue.blocking_dispatch(&mut window).unwrap();

		if !window.app.running() {
			debug!("exiting example");
			break;
		}
	}
}

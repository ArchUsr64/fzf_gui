mod app;
mod events;
mod fonts;
mod picker;
mod window;
use app::App;
use atty::Stream;
use fonts::Font;
use window::Window;

/// The height of the glyphs in pixels
const FONT_SIZE: usize = 20;
/// Size of the window in number of glyphs
const WINDOW_SIZE: (usize, usize) = (80, 20);

fn main() {
	env_logger::init();

	let mut buf = Vec::new();

	if atty::is(Stream::Stdin) {
		let dir_reader = std::fs::read_dir(".").unwrap();
		for i in dir_reader {
			if let Ok(Ok(dir)) = i.map(|dir| dir.file_name().into_string()) {
				buf.push(dir);
			}
		}
	} else {
		let stdin = std::io::stdin();
		for line in stdin.lines() {
			if let Ok(line) = line {
				buf.push(line);
			}
		}
	}
	println!("Buff: {buf:?}");

	let font = Font::from_pbm(include_bytes!("res/font_atlas.pbm"), FONT_SIZE).unwrap();

	for i in ' '..='~' {
		let i = i as usize;
		let g = &font.glyphs[(i >> 5) - 1][i & 0x1f];
		for j in 0..font.height {
			for i in 0..font.width {
				if g[i + j * font.width] != 0 {
					print!("{:02x}", g[i + j * font.width]);
				} else {
					print!("  ");
				}
			}
			println!();
		}
	}

	// We don't draw immediately, the configure will notify us when to first draw.
	let (mut window, mut event_queue) = Window::new(
		(WINDOW_SIZE.0 * FONT_SIZE) as u32,
		(WINDOW_SIZE.1 * FONT_SIZE) as u32 + 2,
		App::new(font),
	);

	loop {
		event_queue.blocking_dispatch(&mut window).unwrap();

		if !window.app.running() {
			println!("exiting example");
			break;
		}
	}
}

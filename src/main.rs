mod app;
mod events;
mod fonts;
mod window;
use app::App;
use fonts::Font;
use window::Window;

fn main() {
	env_logger::init();

	let font = Font::from_pbm(include_bytes!("res/font_atlas.pbm"), 20).unwrap();

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
	let (mut window, mut event_queue) = Window::new(320, 240, App::new());

	loop {
		event_queue.blocking_dispatch(&mut window).unwrap();

		if !window.app.running() {
			println!("exiting example");
			break;
		}
	}
}

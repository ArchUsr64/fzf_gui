//! This example is horrible. Please make a better one soon.
mod window;
use window::Window;

fn main() {
	env_logger::init();

	// We don't draw immediately, the configure will notify us when to first draw.
	let (mut window, mut event_queue) = Window::new(320, 240);

	loop {
		event_queue.blocking_dispatch(&mut window).unwrap();

		if window.exit {
			println!("exiting example");
			break;
		}
	}
}

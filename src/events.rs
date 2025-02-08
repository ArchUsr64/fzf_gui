pub use smithay_client_toolkit::seat::keyboard::{Keysym, Modifiers};

pub use Keysym as Keycode;

#[derive(Debug)]
pub enum Event {
	Focused(bool),
	Keyboard {
		modifiers: Modifiers,
		keycode: Keycode,
		utf8: Option<String>,
	},
}

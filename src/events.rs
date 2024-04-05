pub use smithay_client_toolkit::seat::{
	keyboard::{Keysym, Modifiers},
	pointer::PointerEventKind,
};

pub use Keysym as Keycode;
pub use PointerEventKind as MouseEvent;

#[derive(Debug)]
pub enum Event {
	Focused(bool),
	Keyboard {
		modifiers: Modifiers,
		keycode: Keycode,
		utf8: Option<String>,
	},
	Mouse(MouseEvent),
}

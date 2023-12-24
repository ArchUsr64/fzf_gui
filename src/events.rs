pub use smithay_client_toolkit::seat::{
	keyboard::{KeyEvent, KeyboardHandler, Keysym, Modifiers},
	pointer::{PointerEvent, PointerEventKind, PointerHandler},
	Capability, SeatHandler, SeatState,
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

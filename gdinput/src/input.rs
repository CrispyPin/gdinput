use enigo::{Enigo, Key, KeyboardControllable};
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct VirtualInput {
	enigo: Enigo,
	#[property]
	print_actions: bool,
}

#[methods]
impl VirtualInput {
	fn new(_owner: &Node) -> Self {
		Self {
			enigo: Enigo::new(),
			print_actions: false,
		}
	}

	#[method]
	fn press(&mut self, key_name: String) {
		if let Some(key) = name_to_key(&key_name) {
			self.enigo.key_click(key);
		}
	}

	#[method]
	fn key_down(&mut self, key_name: String) {
		if let Some(key) = name_to_key(&key_name) {
			self.enigo.key_down(key);
		}
	}

	#[method]
	fn key_up(&mut self, key_name: String) {
		if let Some(key) = name_to_key(&key_name) {
			self.enigo.key_up(key);
		}
	}
}

fn name_to_key(name: &str) -> Option<Key> {
	let name = name.to_lowercase();

	if name.len() == 1 {
		return Some(Key::Layout(name.chars().next().unwrap()));
	}

	match name.as_str() {
		"alt" => Some(Key::Alt),
		"backspace" => Some(Key::Backspace),
		"capslock" => Some(Key::CapsLock),
		"control" => Some(Key::Control),
		"delete" => Some(Key::Delete),
		"downarrow" | "down" => Some(Key::DownArrow),
		"end" => Some(Key::End),
		"escape" => Some(Key::Escape),
		"f1" => Some(Key::F1),
		"f2" => Some(Key::F2),
		"f3" => Some(Key::F3),
		"f4" => Some(Key::F4),
		"f5" => Some(Key::F5),
		"f6" => Some(Key::F6),
		"f7" => Some(Key::F7),
		"f8" => Some(Key::F8),
		"f9" => Some(Key::F9),
		"f10" => Some(Key::F10),
		"f11" => Some(Key::F11),
		"f12" => Some(Key::F12),
		"home" => Some(Key::Home),
		"leftarrow" | "left" => Some(Key::LeftArrow),
		"meta" | "super" | "win" => Some(Key::Meta),
		"option" => Some(Key::Option),
		"pagedown" => Some(Key::PageDown),
		"pageup" => Some(Key::PageUp),
		"return" | "enter" => Some(Key::Return),
		"rightarrow" | "right" => Some(Key::RightArrow),
		"shift" => Some(Key::Shift),
		"space" => Some(Key::Space),
		"tab" => Some(Key::Tab),
		"uparrow" | "up" => Some(Key::UpArrow),
		unknown => {
			godot_warn!("Unknown or unsupported key '{unknown}'");
			None
		}
	}
}

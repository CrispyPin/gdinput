use gdnative::prelude::*;

mod input;
use input::VirtualInput;

fn init(handle: InitHandle) {
	handle.add_class::<VirtualInput>();
}

godot_init!(init);

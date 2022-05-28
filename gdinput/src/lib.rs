use gdnative::prelude::*;

mod input;
use input::*;


fn init(handle: InitHandle) {
	handle.add_class::<VirtualInput>();
}

godot_init!(init);

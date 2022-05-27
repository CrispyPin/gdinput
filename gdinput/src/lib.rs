use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
struct ExampleThing {

}


#[methods]
impl ExampleThing {
    fn new(_owner: &Node) -> Self {
        Self {  }
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("hello world");
    }
}


fn init(handle: InitHandle) {
	handle.add_class::<ExampleThing>();
}

godot_init!(init);

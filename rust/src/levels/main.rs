use godot::{
    classes::{INode3D, Node3D},
    obj::Base,
    register::{GodotClass, godot_api},
};

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct Main {
    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for Main {
    fn init(base: Base<Self::Base>) -> Self {
        Main { base }
    }
}

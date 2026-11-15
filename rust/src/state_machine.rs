use std::{vec};

use godot::{
    classes::{
        INode, Node,
        class_macros::private::virtuals::{Xrvrs::Gd, ZipReader::Array},
        notify::NodeNotification,
    }, global::godot_print, obj::{Base, Inherits, OnEditor}, register::{GodotClass, godot_api},
};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct StateMachine {
    base: Base<Node>,
    #[export]
    #[var(pub)]
    current_state: OnEditor<Gd<Node>>,

    #[export]
    states: Array<Gd<Node>>,
}

#[godot_api]
impl INode for StateMachine {
    fn init(base: Base<Self::Base>) -> Self {
        StateMachine {
            base: base,
            current_state: OnEditor::default(),
            states: Array::new(),
        }
    }

    fn ready(&mut self) {
        self.current_state.notify(NodeNotification::Unknown(5001));
    }
}

impl StateMachine {
    pub fn switch_state<T: Inherits<Node>>(&mut self) {
        let mut new_state: Option<Gd<Node>> = None;

        self.states.iter_shared().for_each(|node| {
            let cast = node.try_cast::<T>();
            match cast {
                Ok(state) => new_state = Some(state.upcast()),
                Err(_) => {
                }
            }
        });

        match new_state {
            Some(state) => {
                if state.get_class().to_lower() == self.current_state.get_class().to_lower() {
                    return;
                }

                godot_print!("sending 5002 -> {:?}", self.current_state);
                self.current_state.notify(NodeNotification::Unknown(5002));

                self.set_current_state(state);

                godot_print!("sending 5001 -> {:?}", self.current_state);
                self.current_state.notify(NodeNotification::Unknown(5001));
            }
            None => return,
        }
    }
}

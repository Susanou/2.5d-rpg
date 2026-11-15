use godot::{
    classes::{
        INode, Node,
        class_macros::private::virtuals::{Xrvrs::Gd, ZipReader::Vector2},
        notify::NodeNotification,
    },
    global::godot_print,
    obj::{Base, OnReady, WithBaseField},
    register::{GodotClass, godot_api},
};

use crate::{
    player::{player::Player, player_move_state::PlayerMoveState},
    utils,
};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct PlayerIdleState {
    base: Base<Node>,
    character_node: OnReady<Gd<Player>>,
}

#[godot_api]
impl INode for PlayerIdleState {
    fn init(base: Base<Self::Base>) -> Self {
        PlayerIdleState {
            base: base,
            character_node: OnReady::manual(),
        }
    }

    fn ready(&mut self) {
        let node = self.base_mut().get_owner().unwrap().cast::<Player>();
        self.character_node.init(node.clone());
        self.base_mut().set_physics_process(false);
    }

    fn physics_process(&mut self, _delta: f64) {
        let direction = self.character_node.bind().direction;
        if direction != Vector2::ZERO {
            let mut machine = self.character_node.bind().state_machine_node.clone();
            godot_print!("Sending switch to MOVE");
            machine.bind_mut().switch_state::<PlayerMoveState>();
        }
    }

    fn on_notification(&mut self, what: NodeNotification) {
        match what {
            NodeNotification::Unknown(5001) => {
                godot_print!("Idle receiving 5001");

                let mut character_node = self.character_node.clone();

                {
                    let _guard = self.base_mut();

                    character_node
                        .bind_mut()
                        .animation_player_node
                        .play_ex()
                        .name(utils::ANIM_IDLE)
                        .done();
                }

                self.base_mut().set_physics_process(true);
            }
            NodeNotification::Unknown(5002) => {
                godot_print!("physics stop IDLE received 5002");
                self.base_mut().set_physics_process(false);
            }
            _ => {}
        }
    }
}

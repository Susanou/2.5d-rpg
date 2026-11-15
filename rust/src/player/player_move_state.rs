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
    player::{player::Player, player_idle_state::PlayerIdleState},
    utils,
};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct PlayerMoveState {
    base: Base<Node>,
    character_node: OnReady<Gd<Player>>,
}

#[godot_api]
impl INode for PlayerMoveState {
    fn init(base: Base<Self::Base>) -> Self {
        PlayerMoveState {
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
        if direction == Vector2::ZERO {
            let mut machine = self.character_node.bind().state_machine_node.clone();
            godot_print!("Sending switch to IDLE");
            machine.bind_mut().switch_state::<PlayerIdleState>();
        }
    }

    fn on_notification(&mut self, what: NodeNotification) {
        match what {
            NodeNotification::Unknown(5001) => {
                godot_print!("Move receiving 5001");

                let mut character_node = self.character_node.clone();

                {
                    let _guard = self.base_mut();

                    character_node
                        .bind_mut()
                        .animation_player_node
                        .play_ex()
                        .name(utils::ANIM_MOVE)
                        .done();
                }

                self.base_mut().set_physics_process(true);
            }
            NodeNotification::Unknown(5002) => {
                godot_print!("physics stop MOVE received 5002");
                self.base_mut().set_physics_process(false);
            }
            _ => {}
        }
    }
}

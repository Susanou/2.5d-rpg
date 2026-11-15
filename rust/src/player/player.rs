use std::ops::Mul;

use godot::{
    classes::{
        AnimationPlayer, CharacterBody3D, ICharacterBody3D, Input, InputEvent, Sprite3D, class_macros::private::virtuals::{Xrvrs::Gd, ZipReader::{Vector2, Vector3}},
    }, obj::{Base, OnEditor, Singleton, WithBaseField}, register::{GodotClass, godot_api},
};

use crate::{state_machine::StateMachine, utils};

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Player {
    base: Base<CharacterBody3D>,
    pub direction: Vector2,
    #[export_group(name="Required Nodes")]
    #[export]
    pub animation_player_node: OnEditor<Gd<AnimationPlayer>>,
    #[export]
    sprite_node: OnEditor<Gd<Sprite3D>>,
    #[export]
    pub state_machine_node: OnEditor<Gd<StateMachine>>,
}

#[godot_api]
impl ICharacterBody3D for Player {
    fn init(base: Base<Self::Base>) -> Self {
        Player {
            base: base,
            direction: Vector2::new(0.0, 0.0),
            animation_player_node: OnEditor::default(),
            sprite_node: OnEditor::default(),
            state_machine_node: OnEditor::default()
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        //godot_print!("Player physics process!");
        let dir = self.direction;
        self.base_mut().set_velocity(Vector3::new(dir.x, 0.0, dir.y).mul(5.0));

        self.base_mut().move_and_slide();
        self.flip();
    }

    fn input(&mut self, _event: Gd<InputEvent>) {
        let input = Input::singleton();
        self.direction = Input::get_vector(
            &input,
            utils::INPUT_MOVE_LEFT,
            utils::INPUT_MOVE_RIGHT,
            utils::INPUT_MOVE_FORWARD,
            utils::INPUT_MOVE_BACKWARD,
        );
    }
}

impl Player {
    fn flip(&mut self) {
        if self.base().get_velocity().x == 0.0 {
            return;
        }

        let is_moving_left = self.base().get_velocity().x < 0.0;
        self.sprite_node.set_flip_h(is_moving_left);
    }
}

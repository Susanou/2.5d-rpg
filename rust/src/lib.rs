#![feature(extend_one)]

use godot::prelude::*;

struct Rpg;

#[gdextension]
unsafe impl ExtensionLibrary for Rpg {}

pub mod levels;
pub mod player;
pub mod utils;
pub mod state_machine;
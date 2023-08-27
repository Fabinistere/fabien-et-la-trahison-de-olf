use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
// use bevy_retrograde::prelude::Velocity;

use crate::{
    animations::sprite_sheet_animation::{AnimationIndices, AnimationTimer, CharacterState},
    constants::TILE_SIZE,
};

#[derive(Component, Deref, DerefMut)]
pub struct Speed(pub f32);

impl Default for Speed {
    fn default() -> Self {
        Speed(50. * TILE_SIZE)
    }
}

#[derive(Default, Bundle)]
pub struct MovementBundle {
    pub speed: Speed,
    pub velocity: Velocity,
    pub state: CharacterState,
    pub animation_timer: AnimationTimer,
    pub animation_indices: AnimationIndices,
}

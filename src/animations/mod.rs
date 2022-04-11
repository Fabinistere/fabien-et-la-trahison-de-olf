pub mod fade;
pub mod sprite_sheet_animation;

use bevy::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(fade::fade_animations)
            .add_system(sprite_sheet_animation::animate_sprite_sheet);
    }
}

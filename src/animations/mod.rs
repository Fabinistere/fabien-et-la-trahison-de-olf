pub mod fade;
pub mod sprite_sheet_animation;

use bevy::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(fade::fade_animations.system())
            .add_system(sprite_sheet_animation::animate_sprite_sheet.system());
    }
}


pub mod fade;
pub mod functions;
pub mod slide;
pub mod sprite_sheet_animation;

use bevy::prelude::*;
pub use fade::{Fade, FadeType};
pub use slide::{Slide, UiSlide, UiSlideType};

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                fade::fade_animations,
                slide::slide_animations,
                slide::ui_slide_animations,
                sprite_sheet_animation::animate_sprite_sheet,
                sprite_sheet_animation::jump_frame_manor_lights_state,
                sprite_sheet_animation::tempo_animation_timer
                    .after(sprite_sheet_animation::jump_frame_manor_lights_state),
                sprite_sheet_animation::animate_manor_lights
                    .after(sprite_sheet_animation::tempo_animation_timer),
                sprite_sheet_animation::animate_ui_atlas,
            ),
        );
    }
}

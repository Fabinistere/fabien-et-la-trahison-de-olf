pub mod fade;
pub mod functions;
pub mod slide;
pub mod sprite_sheet_animation;

use bevy::prelude::*;
pub use fade::{Fade, FadeType};
pub use slide::{Slide, UiSlide, UiSlideType};

use crate::in_menu;

// DOC: create systemsLabel for `sprite_sheet_animation::tempo_animation_timer`

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                sprite_sheet_animation::jump_frame_manor_lights_state
                    .before(sprite_sheet_animation::tempo_animation_timer),
                sprite_sheet_animation::animate_manor_lights
                    .after(sprite_sheet_animation::tempo_animation_timer),
                sprite_sheet_animation::animate_ui_atlas
                    .after(sprite_sheet_animation::tempo_animation_timer),
                sprite_sheet_animation::flexing_title
                    .after(sprite_sheet_animation::tempo_animation_timer),
            )
                .run_if(in_menu),
        )
        .add_systems(
            PostUpdate,
            (
                fade::fade_animations,
                slide::slide_animations,
                slide::ui_slide_animations,
                sprite_sheet_animation::animate_sprite_sheet,
                sprite_sheet_animation::tempo_animation_timer,
            ),
        );
    }
}

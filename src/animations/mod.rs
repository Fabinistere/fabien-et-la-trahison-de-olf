pub mod fade;
pub mod functions;
pub mod slide;
pub mod sprite_sheet_animation;

use bevy::prelude::*;
pub use fade::{Fade, FadeType};
pub use slide::{Slide, UiSlide, UiSlideType};

use crate::{
    constants::character::{
        COLUMN_FRAME_IDLE_END, COLUMN_FRAME_IDLE_START, COLUMN_FRAME_RUN_END,
        SPRITESHEET_COLUMN_NUMBER, SPRITESHEET_LINE_NUMBER,
    },
    in_menu,
};

use self::sprite_sheet_animation::CharacterState;

// DOC: create systemsLabel for `sprite_sheet_animation::tempo_animation_timer`

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CharacterSpriteSheet>()
            .init_resource::<GlobalAnimationIndices>()
            .add_systems(
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
                    sprite_sheet_animation::jump_frame_character_state,
                    sprite_sheet_animation::tempo_animation_timer,
                    sprite_sheet_animation::animate_character,
                ),
            );
    }
}

#[derive(Deref, Clone, Resource)]
pub struct CharacterSpriteSheet {
    pub texture_atlas: Handle<TextureAtlas>,
}

impl FromWorld for CharacterSpriteSheet {
    fn from_world(world: &mut World) -> Self {
        let texture_handle = world
            .get_resource::<AssetServer>()
            .unwrap()
            .load("textures/characters/big_spritesheet_v6.png");
        let atlas = TextureAtlas::from_grid(texture_handle, Vec2::splat(34.), 6, 16, None, None);

        let atlas_handle = world
            .get_resource_mut::<Assets<TextureAtlas>>()
            .unwrap()
            .add(atlas);

        CharacterSpriteSheet {
            texture_atlas: atlas_handle,
        }
    }
}

#[derive(Deref, Clone, Resource)]
pub struct GlobalAnimationIndices(Vec<Vec<(usize, usize, CharacterState)>>);

impl FromWorld for GlobalAnimationIndices {
    fn from_world(_world: &mut World) -> Self {
        let mut global_animations_indices: Vec<Vec<(usize, usize, CharacterState)>> = Vec::new();
        for line in 0..SPRITESHEET_LINE_NUMBER {
            global_animations_indices.push(vec![
                // Run Indexes for each line
                (
                    line * SPRITESHEET_COLUMN_NUMBER,
                    line * SPRITESHEET_COLUMN_NUMBER + COLUMN_FRAME_RUN_END,
                    CharacterState::Idle,
                    // CharacterState::Run, ?
                ),
                // Idle Indexes for each line
                (
                    line * SPRITESHEET_COLUMN_NUMBER + COLUMN_FRAME_IDLE_START,
                    line * SPRITESHEET_COLUMN_NUMBER + COLUMN_FRAME_IDLE_END,
                    CharacterState::Idle,
                ),
            ]);
        }

        GlobalAnimationIndices(global_animations_indices)
    }
}

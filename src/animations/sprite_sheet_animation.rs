use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;

use crate::{
    constants::title_screen::MANOR_LIGHTS_PATTERN_INDEXES,
    menu::{ManorLightsPattern, ManorLightsTimer},
};

#[derive(Reflect, Component)]
pub struct SpriteSheetAnimation {
    pub start_index: usize,
    pub end_index: usize,
    pub timer: Timer,
    pub duration: AnimationDuration,
}

#[derive(Reflect, PartialEq, Eq, PartialOrd, Ord, Component)]
pub enum AnimationDuration {
    Infinite,
    Once,
}

#[derive(Deref, DerefMut, Reflect, Component)]
pub struct TempoAnimation(pub Timer);

pub fn animate_sprite_sheet(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut SpriteSheetAnimation, &mut TextureAtlasSprite)>,
) {
    for (entity, mut animation, mut sprite) in query.iter_mut() {
        animation.timer.tick(time.delta());

        if animation.timer.finished() {
            if sprite.index >= animation.end_index {
                if animation.duration == AnimationDuration::Once {
                    commands.entity(entity).remove::<SpriteSheetAnimation>();
                } else {
                    sprite.index = animation.start_index;
                }
            } else {
                sprite.index += 1;
            }
        }
    }
}

/// Jump directly to the correct frame when the state has changed.
/// - If the state is the default one: `ManorLightsPattern::FullLights`,
/// Then start a `TempoAnimation` Timer
/// which will be taken care of in `sprite_sheet_animation::tempo_animation_timer`.
/// - Else
pub fn jump_frame_manor_lights_state(
    mut commands: Commands,
    mut manor_lights_query: Query<
        (Entity, &mut UiTextureAtlasImage, &ManorLightsPattern),
        Changed<ManorLightsPattern>,
    >,
) {
    for (manor_lights, mut sprite, manor_lights_state) in &mut manor_lights_query {
        // info!("{manor_lights_state:#?}");
        sprite.index = MANOR_LIGHTS_PATTERN_INDEXES[(*manor_lights_state) as usize].0;

        match manor_lights_state {
            // when running each time the anim loops it triggers this match arm
            ManorLightsPattern::FullLights => {
                let tempo_secs = rand::thread_rng().gen_range(2..=10);
                commands
                    .entity(manor_lights)
                    .insert(TempoAnimation(Timer::new(
                        Duration::from_secs(tempo_secs),
                        TimerMode::Once,
                    )));
            }
            ManorLightsPattern::BotShutdown
            | ManorLightsPattern::TopShutdown
            | ManorLightsPattern::LeftShutdown => {
                commands
                    .entity(manor_lights)
                    .insert(TempoAnimation(Timer::new(
                        Duration::from_secs(rand::thread_rng().gen_range(3..=6)),
                        TimerMode::Once,
                    )));
            }
            _ => {}
        }
    }
}

/// Could be a character or a menu entity.
/// Decrease each frame the `TempoAnimation` timer.
pub fn tempo_animation_timer(
    time: Res<Time>,
    mut commands: Commands,
    mut temporized_query: Query<(Entity, &mut TempoAnimation)>,
) {
    for (entity, mut timer) in &mut temporized_query {
        timer.tick(time.delta());
        if timer.just_finished() {
            commands.entity(entity).remove::<TempoAnimation>();
        }
    }
}

/// Don't affect the manor lights.
pub fn animate_ui_atlas(
    mut commands: Commands,
    time: Res<Time>,
    mut atlas_images: Query<
        (Entity, &mut SpriteSheetAnimation, &mut UiTextureAtlasImage),
        (Without<TempoAnimation>, Without<ManorLightsPattern>),
    >,
) {
    for (entity, mut animation, mut atlas_image) in atlas_images.iter_mut() {
        animation.timer.tick(time.delta());

        if animation.timer.finished() {
            if atlas_image.index >= animation.end_index {
                if animation.duration == AnimationDuration::Once {
                    commands.entity(entity).remove::<SpriteSheetAnimation>();
                } else {
                    atlas_image.index = animation.start_index;
                }
            } else {
                atlas_image.index += 1;
            }
        }
    }
}

/// Only for the manor lights.
/// When the `TempoAnimation` is finished,
/// - Choose a random new lights pattern
/// (except of `ManorLightsPattern::FullLights`,
/// confers `Distribution<ManorLightsPattern>` custom implementation).
/// - Or animate the pattern.
pub fn animate_manor_lights(
    time: Res<Time>,
    mut manor_lights_query: Query<
        (
            &mut UiTextureAtlasImage,
            &mut ManorLightsTimer,
            &mut ManorLightsPattern,
        ),
        Without<TempoAnimation>,
    >,
) {
    for (mut atlas_image, mut manor_lights_timer, mut manor_lights_pattern) in
        &mut manor_lights_query
    {
        manor_lights_timer.tick(time.delta());

        if manor_lights_timer.finished() {
            match *manor_lights_pattern {
                ManorLightsPattern::FullLights => {
                    *manor_lights_pattern = rand::thread_rng().gen::<ManorLightsPattern>()
                }
                _ => {
                    // info!(
                    //     "atlas.index: {}/{}",
                    //     atlas_image.index,
                    //     MANOR_LIGHTS_PATTERN_INDEXES[(*manor_lights_pattern) as usize].1
                    // );

                    if atlas_image.index
                        >= MANOR_LIGHTS_PATTERN_INDEXES[(*manor_lights_pattern) as usize].1
                    {
                        *manor_lights_pattern = ManorLightsPattern::FullLights;
                    } else {
                        atlas_image.index += 1;
                    }
                }
            };
        }
    }
}

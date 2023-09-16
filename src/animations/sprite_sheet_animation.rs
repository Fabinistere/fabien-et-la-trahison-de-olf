use bevy::prelude::*;

use rand::Rng;
use std::{collections::HashMap, time::Duration};

use crate::{
    characters::{
        npcs::{OlfCat, NPC},
        player::Player,
    },
    constants::{
        title_screen::{
            MANOR_LIGHTS_PATTERN_INDEXES, TITLE_FLEX_BOT, TITLE_FLEX_BOT_DELTA_S, TITLE_FLEX_TOP,
            TITLE_FLEX_TOP_DELTA_S,
        },
        FRAME_TIME,
    },
    menu::{ManorLightsPattern, ManorLightsTimer, Smoke, Title, TitleState},
};

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Reflect, Component)]
pub enum CharacterState {
    #[default]
    Idle,
    Run,
}

#[derive(Deref, DerefMut, Component)]
pub struct AnimationTimer(pub Timer);

impl Default for AnimationTimer {
    fn default() -> Self {
        AnimationTimer(Timer::from_seconds(FRAME_TIME, TimerMode::Repeating))
    }
}

/// A CharacterState is linked to
///
/// - a start_index (first frame),
/// - a end_index (last frame),
/// - the next CharacterState (after the anim ended)
#[derive(Deref, DerefMut, Clone, Reflect, Default, Component)]
pub struct AnimationIndices(pub HashMap<CharacterState, (usize, usize, CharacterState)>);

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
pub fn jump_frame_character_state(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &AnimationIndices,
            &mut TextureAtlasSprite,
            &CharacterState,
        ),
        Changed<CharacterState>,
    >,
) {
    for (character, indices, mut sprite, character_state) in &mut query {
        // info!("{character_state:#?}",);
        let (first_indice, _, _) = &indices.get(character_state).unwrap();
        sprite.index = *first_indice;

        match character_state {
            // when running each time the anim loops it triggers this match arm
            CharacterState::Idle => {
                commands.entity(character).insert(TempoAnimation(Timer::new(
                    Duration::from_secs_f32(rand::thread_rng().gen_range(0.1..=5.)),
                    TimerMode::Once,
                )));
            }
            _ => {
                commands.entity(character).remove::<TempoAnimation>();
            }
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

pub fn animate_character(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut characters_query: Query<
        (
            Entity,
            &AnimationIndices,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
            &mut CharacterState,
            &Name,
        ),
        (
            Or<(With<Player>, With<NPC>, With<OlfCat>)>,
            Without<TempoAnimation>,
        ),
    >,
) {
    for (
        _character,
        indices,
        mut timer,
        mut sprite,
        texture_atlas_handle,
        mut character_state,
        name,
    ) in &mut characters_query
    {
        timer.tick(time.delta());

        if timer.just_finished() {
            let (_first_frame, last_frame, next_phase) = &indices.get(&character_state).unwrap();
            // info!(
            //     "({_first_frame}, {last_frame}, {next_phase:#?}): {}",
            //     sprite.index
            // );
            // eprintln!("{:#?}", sprite);

            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();

            if sprite.index == *last_frame {
                // update state
                sprite.index = indices.get(next_phase).unwrap().0;
                *character_state = *next_phase;
            } else if sprite.index + 1 < texture_atlas.textures.len() {
                sprite.index += 1;
            } else {
                error!("anim limit reached: {}", name);
                // commands.entity(character).remove::<AnimationTimer>();
                *character_state = *next_phase;
                sprite.index = indices.get(next_phase).unwrap().0;
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
                commands
                    .entity(manor_lights)
                    .insert(TempoAnimation(Timer::new(
                        Duration::from_secs(rand::thread_rng().gen_range(2..=10)),
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

/// Don't affect the manor lights.
pub fn animate_ui_atlas(
    mut commands: Commands,
    time: Res<Time>,
    mut atlas_images: Query<
        (Entity, &mut SpriteSheetAnimation, &mut UiTextureAtlasImage),
        (Without<TempoAnimation>, Without<ManorLightsPattern>),
    >,
    smoke_query: Query<Entity, With<Smoke>>,
) {
    for (entity, mut animation, mut atlas_image) in atlas_images.iter_mut() {
        animation.timer.tick(time.delta());

        if animation.timer.finished() {
            if atlas_image.index >= animation.end_index {
                if animation.duration == AnimationDuration::Once {
                    commands.entity(entity).remove::<SpriteSheetAnimation>();
                } else {
                    atlas_image.index = animation.start_index;
                    if smoke_query.get(entity).is_ok() {
                        commands.entity(entity).insert(TempoAnimation(Timer::new(
                            Duration::from_secs(rand::thread_rng().gen_range(6..=15)),
                            TimerMode::Once,
                        )));
                    }
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

pub fn flexing_title(
    mut commands: Commands,
    mut title_query: Query<
        (Entity, &mut Style, &mut TitleState),
        (With<Title>, Without<TempoAnimation>),
    >,
) {
    if let Ok((entity, mut style, mut state)) = title_query.get_single_mut() {
        match *state {
            TitleState::FlexTop => {
                *state = TitleState::FlexBot;
                style.bottom = Val::Px(TITLE_FLEX_BOT);
                commands.entity(entity).insert(TempoAnimation(Timer::new(
                    Duration::from_secs(TITLE_FLEX_BOT_DELTA_S),
                    TimerMode::Once,
                )));
            }
            TitleState::FlexBot => {
                *state = TitleState::FlexTop;
                style.bottom = Val::Px(TITLE_FLEX_TOP);
                commands.entity(entity).insert(TempoAnimation(Timer::new(
                    Duration::from_secs(TITLE_FLEX_TOP_DELTA_S),
                    TimerMode::Once,
                )));
            }
            _ => {}
        }
    }
}

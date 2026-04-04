use bevy::prelude::*;

use rand::RngExt;
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
    locations::temple::{
        secret_room::{FlowerPanel, FlowerPot},
        Flame,
    },
    menu::{ArtMenu, ManorLightsPattern, ManorLightsTimer, Smoke, Title, TitleState},
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

/// For non-ui objects, flames, plants
pub fn animate_objects(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<
        (Entity, &mut SpriteSheetAnimation, &mut TextureAtlas),
        (
            Without<TempoAnimation>,
            Or<(With<Flame>, With<FlowerPot>, With<FlowerPanel>)>,
        ),
    >,
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
            &mut TextureAtlas,
            &CharacterState,
        ),
        Changed<CharacterState>,
    >,
) {
    for (character, indices, mut atlas, character_state) in &mut query {
        // log::info!("{character_state:#?}",);
        let (start_anim, _, _) = &indices.get(character_state).unwrap();
        atlas.index = *start_anim;

        match character_state {
            // when running each time the anim loops, it's back to the Idle State
            CharacterState::Idle => {
                commands.entity(character).insert(TempoAnimation(Timer::new(
                    Duration::from_secs_f32(rand::rng().random_range(0.1..=5.)),
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
    texture_atlases: Res<Assets<TextureAtlasLayout>>,
    mut characters_query: Query<
        (
            Entity,
            &AnimationIndices,
            &mut AnimationTimer,
            &mut TextureAtlas,
            &mut CharacterState,
            &Name,
        ),
        (
            Or<(With<Player>, With<NPC>, With<OlfCat>)>,
            Without<TempoAnimation>,
        ),
    >,
) {
    for (_character, indices, mut timer, mut atlas, mut character_state, name) in
        &mut characters_query
    {
        timer.tick(time.delta());

        if timer.just_finished() {
            let (_first_frame, last_frame, next_phase) = &indices.get(&character_state).unwrap();
            // log::info!(
            //     "({_first_frame}, {last_frame}, {next_phase:#?}): {}",
            //     atlas.index
            // );
            // eprintln!("{:#?}", atlas);

            if let Some(layout) = texture_atlases.get(atlas.layout.clone()) {
                if atlas.index == *last_frame {
                    // update state
                    atlas.index = indices.get(next_phase).unwrap().0;
                    *character_state = *next_phase;
                } else if atlas.index + 1 < layout.textures.len() {
                    atlas.index += 1;
                } else {
                    log::error!(target: "Animation", "anim limit reached: {name}");
                    // commands.entity(character).remove::<AnimationTimer>();
                    *character_state = *next_phase;
                    atlas.index = indices.get(next_phase).unwrap().0;
                }
            } else {
                log::error!(target: "Animation", "this character doesn't have an atlas: {name}");
            }
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                           Title Screen Animation                           */
/* -------------------------------------------------------------------------- */

/// Jump directly to the correct frame when the state has changed.
/// - If the state is the default one: `ManorLightsPattern::FullLights`,
///   Then start a `TempoAnimation` Timer
///   which will be taken care of in `sprite_sheet_animation::tempo_animation_timer`.
/// - DOC: write the "Else" section
pub fn jump_frame_manor_lights_state(
    mut commands: Commands,
    mut manor_lights_query: Query<
        (Entity, &mut TextureAtlas, &ManorLightsPattern),
        Changed<ManorLightsPattern>,
    >,
) {
    for (manor_lights, mut atlas, manor_lights_state) in &mut manor_lights_query {
        // log::info!("{manor_lights_state:#?}");
        atlas.index = MANOR_LIGHTS_PATTERN_INDEXES[(*manor_lights_state) as usize].0;

        match manor_lights_state {
            // when running each time the anim loops it triggers this match arm
            ManorLightsPattern::FullLights => {
                commands
                    .entity(manor_lights)
                    .insert(TempoAnimation(Timer::new(
                        Duration::from_secs(rand::rng().random_range(2..=10)),
                        TimerMode::Once,
                    )));
            }
            ManorLightsPattern::BotShutdown
            | ManorLightsPattern::TopShutdown
            | ManorLightsPattern::LeftShutdown => {
                commands
                    .entity(manor_lights)
                    .insert(TempoAnimation(Timer::new(
                        Duration::from_secs(rand::rng().random_range(3..=6)),
                        TimerMode::Once,
                    )));
            }
            _ => {}
        }
    }
}

/// Don't affect the manor lights.
/// Clouds, Smoke
pub fn animate_ui_atlas(
    mut commands: Commands,
    time: Res<Time>,
    mut atlas_images: Query<
        (Entity, &mut SpriteSheetAnimation, &mut TextureAtlas),
        (
            Without<TempoAnimation>,
            Without<ManorLightsPattern>,
            Or<(With<Smoke>, With<ArtMenu>)>,
        ),
    >,
    smoke_query: Query<Entity, With<Smoke>>,
) {
    for (entity, mut animation, mut atlas) in atlas_images.iter_mut() {
        animation.timer.tick(time.delta());

        if animation.timer.finished() {
            if atlas.index >= animation.end_index {
                if animation.duration == AnimationDuration::Once {
                    commands.entity(entity).remove::<SpriteSheetAnimation>();
                } else {
                    atlas.index = animation.start_index;
                    if smoke_query.get(entity).is_ok() {
                        commands.entity(entity).insert(TempoAnimation(Timer::new(
                            Duration::from_secs(rand::rng().random_range(6..=15)),
                            TimerMode::Once,
                        )));
                    }
                }
            } else {
                atlas.index += 1;
            }
        }
    }
}

/// Only for the manor lights.
/// When the `TempoAnimation` is finished,
/// - Choose a random new lights pattern
///   (except of `ManorLightsPattern::FullLights`,
///   confers `Distribution<ManorLightsPattern>` custom implementation).
/// - Or animate the pattern.
pub fn animate_manor_lights(
    time: Res<Time>,
    mut manor_lights_query: Query<
        (
            &mut TextureAtlas,
            &mut ManorLightsTimer,
            &mut ManorLightsPattern,
        ),
        Without<TempoAnimation>,
    >,
) {
    for (mut atlas, mut manor_lights_timer, mut manor_lights_pattern) in &mut manor_lights_query {
        manor_lights_timer.tick(time.delta());

        if manor_lights_timer.finished() {
            match *manor_lights_pattern {
                ManorLightsPattern::FullLights => {
                    *manor_lights_pattern = rand::rng().random::<ManorLightsPattern>()
                }
                _ => {
                    // log::info!(
                    //     "atlas.index: {}/{}",
                    //     atlas.index,
                    //     MANOR_LIGHTS_PATTERN_INDEXES[(*manor_lights_pattern) as usize].1
                    // );

                    if atlas.index
                        >= MANOR_LIGHTS_PATTERN_INDEXES[(*manor_lights_pattern) as usize].1
                    {
                        *manor_lights_pattern = ManorLightsPattern::FullLights;
                    } else {
                        atlas.index += 1;
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

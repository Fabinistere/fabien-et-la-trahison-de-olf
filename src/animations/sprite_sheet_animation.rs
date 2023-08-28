use std::{collections::HashMap, time::Duration};

use bevy::prelude::*;
use rand::Rng;

use crate::{
    characters::{
        npcs::{OlfCat, NPC},
        player::Player,
    },
    constants::FRAME_TIME,
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

#[derive(Component)]
pub struct SpriteSheetAnimation {
    pub start_index: usize,
    pub end_index: usize,
    pub timer: Timer,
    pub duration: AnimationDuration,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Component)]
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
        let (first_indice, _, _) = &indices.get(&character_state).unwrap();
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

/// Could be a character or a menu entity
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
                *character_state = next_phase.clone();
            } else if sprite.index + 1 < texture_atlas.textures.len() {
                sprite.index = sprite.index + 1
            } else {
                error!("anim limit reached: {}", name);
                // commands.entity(character).remove::<AnimationTimer>();
                *character_state = next_phase.clone();
                sprite.index = indices.get(next_phase).unwrap().0;
            }
        }
    }
}

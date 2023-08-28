//! NPCs lockup

use std::collections::HashMap;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    animations::{
        sprite_sheet_animation::{AnimationIndices, CharacterState},
        CharacterSpriteSheet,
    },
    constants::{
        character::{
            npc::{
                BLACK_CAT_LINE, NPC_SCALE, OLF_CAT_HITBOX_OFFSET, OLF_CAT_POSITION, OLF_CAT_SCALE,
                SUPREME_GOD_LINE,
            },
            player::PLAYER_Z,
            *,
        },
        locations::main_room::THRONE_POSITION,
    },
};

use super::{movement::MovementBundle, CharacterHitbox};

#[derive(Default)]
pub struct NPCPlugin;

/**
 * NPC has hobbies
 *  - landwark
 *    - index in const, with free: bol
 *    - when talking to a npc in a landwark, include the other present
 *    -> rest
 *  - stroll
 *    - in a restricted zone -index in const-
 *    -> rest
 *  - rest
 *    -> stroll
 *    -> landwark
 *  - talking to MC
 *    - infite rest until the MC is leaving
 *    -> short rest
 *    or
 *    -> stroll
 *    -> landmark
 *    -> rest
 *
 * Reflexion
 *  - should npc avoid hit other entity
 *  - turn false the free param from a landmark position taken by the MC
 */
impl Plugin for NPCPlugin {
    fn build(&self, app: &mut App) {
        app
            // when an enemy npc catch the player or an ally attached to the group
            // initialize a Combat
            // Combat mean A lock dialogue : Talk or Fight
            .add_systems(Startup, (
                spawn_characters,
            ))
            // .add_systems(
            //     FixedUpdate,
            //     (
            //         movement::just_walk.in_set(NPCSystems::Stroll),
            //         idle::do_flexing
            //             .in_set(NPCSystems::Idle)
            //             .after(NPCSystems::Stroll),
            //         movement::follow.in_set(NPCSystems::Follow),
            //         aggression::add_detection_aura.before(NPCSystems::FindTargets),
            //         aggression::threat_detection.in_set(NPCSystems::FindTargets),
            //         aggression::add_pursuit_urge
            //             .before(NPCSystems::Chase)
            //             .after(NPCSystems::FindTargets),
            //         movement::pursue
            //             .in_set(NPCSystems::Chase)
            //             .after(NPCSystems::FindTargets),
            //         aggression::remove_pursuit_urge
            //             .in_set(NPCSystems::StopChase)
            //             .after(NPCSystems::Chase),
            //         aggression::fair_play_wait.after(NPCSystems::StopChase),
            //         aggression::add_detection_aura.after(NPCSystems::StopChase),
            //     ),
            // )
            ;
    }
}

#[derive(Component)]
pub struct NPC;

#[derive(Component)]
pub struct OlfCat;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum NPCSystems {
    Stroll,
    Follow,
    // FindLandmark,
    FindTargets,
    Chase,
    StopChase,
    // Talking,
    Idle,
    // Combat,
}

fn spawn_characters(mut commands: Commands, characters_spritesheet: Res<CharacterSpriteSheet>) {
    let mut global_animations_indices: Vec<Vec<(usize, usize, CharacterState)>> = Vec::new();
    for line in 0..16 {
        global_animations_indices.push(vec![
            // Run Indexes for each line
            (
                line * SPRITESHEET_COLUMN_NUMBER,
                line * SPRITESHEET_COLUMN_NUMBER + COLUMN_FRAME_RUN_END,
                CharacterState::Idle,
            ),
            // Idle Indexes for each line
            (
                line * SPRITESHEET_COLUMN_NUMBER + COLUMN_FRAME_IDLE_START,
                line * SPRITESHEET_COLUMN_NUMBER + COLUMN_FRAME_IDLE_END,
                CharacterState::Idle,
            ),
        ]);
    }

    /* -------------------------------------------------------------------------- */
    /*                                   Olf Cat                                  */
    /* -------------------------------------------------------------------------- */
    // TODO: Polish #visual - Cat like movement
    let mut cat_animation_indices = AnimationIndices(HashMap::new());
    cat_animation_indices.insert(
        CharacterState::Run,
        (
            BLACK_CAT_LINE * SPRITESHEET_COLUMN_NUMBER,
            BLACK_CAT_LINE * SPRITESHEET_COLUMN_NUMBER + COLUMN_FRAME_RUN_END,
            CharacterState::Idle,
        ),
    );
    cat_animation_indices.insert(
        CharacterState::Idle,
        (
            BLACK_CAT_LINE * SPRITESHEET_COLUMN_NUMBER + COLUMN_FRAME_IDLE_START,
            BLACK_CAT_LINE * SPRITESHEET_COLUMN_NUMBER + COLUMN_FRAME_IDLE_END,
            CharacterState::Idle,
        ),
    );

    // TEMP: Static Olf cat
    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: characters_spritesheet.texture_atlas.clone(),
                transform: Transform {
                    translation: OLF_CAT_POSITION.into(),
                    scale: Vec3::splat(OLF_CAT_SCALE),
                    ..Transform::default()
                },
                ..default()
            },
            Name::new("Olf Cat"),
            OlfCat,
            // OverlappingProps {
            //     layer: Layer::Fourth,
            //     switch_offset_y: CAT_SWITCH_Y_OFFSET,
            // },
            MovementBundle {
                animation_indices: cat_animation_indices,
                ..default()
            },
            // -- Hitbox --
            RigidBody::Fixed,
        ))
        .with_children(|parent| {
            parent.spawn((
                Collider::cuboid(2.5, 1.),
                Transform::from_translation(OLF_CAT_HITBOX_OFFSET.into()),
            ));
        });

    /* -------------------------------------------------------------------------- */
    /*                                 Supreme God                                */
    /* -------------------------------------------------------------------------- */

    let mut supreme_god_animation_indices = AnimationIndices(HashMap::new());
    supreme_god_animation_indices.insert(
        CharacterState::Run,
        global_animations_indices[SUPREME_GOD_LINE][0],
    );
    supreme_god_animation_indices.insert(
        CharacterState::Idle,
        global_animations_indices[SUPREME_GOD_LINE][1],
    );

    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: characters_spritesheet.texture_atlas.clone(),
                transform: Transform {
                    translation: Vec3::new(THRONE_POSITION.0, THRONE_POSITION.1, PLAYER_Z),
                    scale: Vec3::splat(NPC_SCALE),
                    ..default()
                },
                ..default()
            },
            Name::new("NPC Admiral"),
            NPC,
            MovementBundle {
                animation_indices: supreme_god_animation_indices,
                ..default()
            },
            // -- Hitbox --
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
        ))
        .with_children(|parent| {
            parent.spawn((
                Collider::cuboid(CHAR_HITBOX_WIDTH, CHAR_HITBOX_HEIGHT),
                Transform::from_xyz(0., CHAR_HITBOX_Y_OFFSET, 0.),
                CharacterHitbox,
                Name::new("Admiral Hitbox"),
            ));
        });
}

//! NPCs lockup

pub mod aggression;
pub mod idle;
pub mod movement;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use yml_dialog::DialogNode;

use std::collections::{BTreeMap, HashMap};

use crate::{
    animations::{
        sprite_sheet_animation::{AnimationIndices, CharacterState},
        CharacterSpriteSheet,
    },
    combat::{Recruted, Reputation},
    constants::character::{dialog::FABIEN_DIALOG, npc::*, player::PLAYER_SPAWN, *},
    interactions::{Interactible, InteractionSensor},
    locations::temple::OverlappingEntity,
    ui::dialog_systems::{CurrentInterlocutor, DialogMap},
    HUDState,
};

use self::movement::FollowupBehavior;

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
            .add_event::<CharacterInteractionEvent>()
            .add_event::<aggression::StopChaseEvent>()
            .add_event::<aggression::DetectionModeEvent>()
            .add_event::<aggression::EngagePursuitEvent>()
            .add_systems(Startup, (spawn_characters,))
            .add_systems(Update, supreme_god_interaction_event)
            .add_systems(
                FixedUpdate,
                (
                    movement::just_walk.in_set(NPCSystems::Stroll),
                    idle::do_flexing
                        .in_set(NPCSystems::Idle)
                        .after(NPCSystems::Stroll),
                    movement::follow.in_set(NPCSystems::Follow),
                    aggression::add_detection_aura.before(NPCSystems::FindTargets),
                    aggression::threat_detection.in_set(NPCSystems::FindTargets),
                    aggression::add_pursuit_urge
                        .before(NPCSystems::Chase)
                        .after(NPCSystems::FindTargets),
                    movement::pursue
                        .in_set(NPCSystems::Chase)
                        .after(NPCSystems::FindTargets),
                    movement::animation
                        .after(NPCSystems::Follow)
                        .after(NPCSystems::Stroll)
                        .after(NPCSystems::Chase)
                        .after(NPCSystems::Idle),
                    aggression::remove_pursuit_urge
                        .in_set(NPCSystems::StopChase)
                        .after(NPCSystems::Chase),
                    aggression::fair_play_wait.after(NPCSystems::StopChase),
                    aggression::add_detection_aura.after(NPCSystems::StopChase),
                ),
            );
    }
}

#[derive(Component)]
pub struct NPC;

#[derive(Component)]
pub struct OlfCat;

#[derive(Event)]
pub struct CharacterInteractionEvent(pub Entity);

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

pub fn supreme_god_interaction_event(
    mut supreme_god_interaction_events: EventReader<CharacterInteractionEvent>,

    mut current_interlocutor: ResMut<CurrentInterlocutor>,
    mut next_game_state: ResMut<NextState<HUDState>>,
) {
    for CharacterInteractionEvent(character) in supreme_god_interaction_events.iter() {
        // info!("CharacterInteractionEvent({:#?})", character);
        current_interlocutor.interlocutor = Some(*character);
        next_game_state.set(HUDState::DialogWall);
    }
}

fn spawn_characters(
    mut commands: Commands,
    characters_spritesheet: Res<CharacterSpriteSheet>,
    mut dialogs: ResMut<DialogMap>,
) {
    let mut global_animations_indices: Vec<Vec<(usize, usize, CharacterState)>> = Vec::new();
    for line in 0..16 {
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
            MovementBundle {
                animation_indices: cat_animation_indices,
                overlapping_entity: OverlappingEntity::new(CAT_SWITCH_Z_OFFSET),
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

    let supreme_god = commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: characters_spritesheet.texture_atlas.clone(),
                transform: Transform {
                    translation: SUPREME_GOD_SPAWN_POSITION.into(),
                    scale: Vec3::splat(NPC_SCALE),
                    ..default()
                },
                ..default()
            },
            Name::new("NPC Supreme God"),
            NPC,
            MovementBundle {
                animation_indices: supreme_god_animation_indices,
                ..default()
            },
            // -- Social --
            Interactible::new(
                CHARACTER_INTERACT_BUTTON_POSITION.into(),
                SUPREME_GOD_INTERACTION_ID,
            ),
            Reputation::new(100, 0),
            // -- Hitbox --
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
        ))
        .with_children(|parent| {
            parent.spawn((
                Collider::ball(15.),
                Transform::IDENTITY,
                Sensor,
                InteractionSensor,
                Name::new("Supreme God InteractionSensor"),
            ));

            parent.spawn((
                Collider::cuboid(CHAR_HITBOX_WIDTH, CHAR_HITBOX_HEIGHT),
                Transform::from_xyz(0., CHAR_HITBOX_Y_OFFSET, 0.),
                CharacterHitbox,
                Name::new("Supreme God Hitbox"),
            ));
        })
        .id();

    let supreme_god_deserialized_map: BTreeMap<usize, DialogNode> =
        serde_yaml::from_str(FABIEN_DIALOG).unwrap();
    dialogs.insert(
        supreme_god,
        (
            *supreme_god_deserialized_map.first_key_value().unwrap().0,
            supreme_god_deserialized_map,
        ),
    );

    /* -------------------------------------------------------------------------- */
    /*                                    HUGO                                    */
    /* -------------------------------------------------------------------------- */

    let mut hugo_animation_indices = AnimationIndices(HashMap::new());
    hugo_animation_indices.insert(
        CharacterState::Run,
        global_animations_indices[HEALER_V2_LINE][0],
    );
    hugo_animation_indices.insert(
        CharacterState::Idle,
        global_animations_indices[HEALER_V2_LINE][1],
    );

    let hugo = commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: characters_spritesheet.texture_atlas.clone(),
                transform: Transform {
                    translation: PLAYER_SPAWN.into(),
                    scale: Vec3::splat(NPC_SCALE),
                    ..default()
                },
                ..default()
            },
            Name::new("NPC Hugo"),
            NPC,
            MovementBundle {
                animation_indices: hugo_animation_indices,
                ..default()
            },
            // -- Social --
            Reputation::new(100, 0),
            Recruted,
            FollowupBehavior,
            Interactible::new(
                CHARACTER_INTERACT_BUTTON_POSITION.into(),
                HUGO_INTERACTION_ID,
            ),
            // -- Hitbox --
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
        ))
        .with_children(|parent| {
            parent.spawn((
                Collider::ball(15.),
                Transform::IDENTITY,
                Sensor,
                InteractionSensor,
                Name::new("Hugo InteractionSensor"),
            ));

            parent.spawn((
                Collider::cuboid(CHAR_HITBOX_WIDTH, CHAR_HITBOX_HEIGHT),
                Transform::from_xyz(0., CHAR_HITBOX_Y_OFFSET, 0.),
                CharacterHitbox,
                Name::new("Hugo Hitbox"),
            ));
        })
        .id();

    let hugo_deserialized_map: BTreeMap<usize, DialogNode> =
        serde_yaml::from_str(FABIEN_DIALOG).unwrap();
    dialogs.insert(
        hugo,
        (
            *hugo_deserialized_map.first_key_value().unwrap().0,
            hugo_deserialized_map,
        ),
    );
}

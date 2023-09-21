//! NPCs lockup

pub mod aggression;
pub mod idle;
pub mod movement;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::{BTreeMap, HashMap};
use yml_dialog::DialogNode;

use crate::{
    animations::{
        sprite_sheet_animation::{AnimationIndices, CharacterState},
        CharacterSpriteSheet, GlobalAnimationIndices,
    },
    characters::{movement::MovementBundle, npcs::movement::NPCBehavior, CharacterHitbox},
    combat::Reputation,
    constants::{
        character::{npcs::*, player::PLAYER_SPAWN, *},
        interactions::INTERACT_BUTTON_SCALE,
    },
    hud_opened,
    interactions::{InteractIcon, Interactible, InteractionResources, InteractionSensor},
    locations::{
        landmarks::{reserved_random_free_landmark, Landmark},
        temple::{Location, OverlappingEntity},
    },
    ui::dialog_systems::{CurrentInterlocutor, DialogMap},
    GameState, HUDState,
};

use self::{
    aggression::{DetectionRangeSensor, PursuitRangeSensor},
    movement::{FollowRangeSensor, TargetSeeker, TargetType},
};

use super::player::Player;

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
            .add_event::<movement::FollowEvent>()
            .add_event::<aggression::StopChaseEvent>()
            .add_event::<aggression::EngagePursuitEvent>()
            .add_systems(
                OnEnter(GameState::Playing),
                (spawn_characters, spawn_vilains, spawn_cat),
            )
            .add_systems(
                Update,
                (
                    character_interaction_event,
                    movement::follow_event,
                    movement::npc_behavior_change,
                    movement::chase_management.in_set(NPCSystems::Collision),
                    aggression::activate_pursuit_urge.after(NPCSystems::Collision),
                    aggression::deactivate_pursuit_urge.after(NPCSystems::Collision),
                    idle::flexing_timer
                        .in_set(NPCSystems::Idle)
                        .after(NPCSystems::Movement),
                    freeze_player_in_dialog.run_if(hud_opened),
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                FixedUpdate,
                movement::npc_movement
                    .in_set(NPCSystems::Movement)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                PostUpdate,
                movement::animation.run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component)]
pub struct NPC;

// /// Global Direction TODO: or StareAt Entity
// #[derive(Deref, Component)]
// pub struct StareAt(pub Entity);

#[derive(Component)]
pub struct OlfCat;

#[derive(Event)]
pub struct CharacterInteractionEvent(pub Entity);

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum NPCSystems {
    Movement,
    Collision,
    // --- OLD ---
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

pub fn character_interaction_event(
    mut character_interaction_events: EventReader<CharacterInteractionEvent>,

    mut current_interlocutor: ResMut<CurrentInterlocutor>,
    mut next_game_state: ResMut<NextState<HUDState>>,
) {
    for CharacterInteractionEvent(character) in character_interaction_events.iter() {
        // info!("CharacterInteractionEvent({:#?})", character);
        current_interlocutor.interlocutor = Some(*character);
        next_game_state.set(HUDState::DialogWall);
    }
}

/// FIXME: Freeze ALL character in Dialog
fn freeze_player_in_dialog(
    // talker_query: Query<&mut Velocity, With<InDialog>>,
    mut player_query: Query<(&mut Velocity, &mut CharacterState), With<Player>>,
) {
    let (mut rb_vel, mut character_state) = player_query.single_mut();
    rb_vel.linvel.x = 0.;
    rb_vel.linvel.y = 0.;
    *character_state = CharacterState::Idle;
}

/// Cats and all friendly npc
fn spawn_characters(
    mut commands: Commands,
    characters_spritesheet: Res<CharacterSpriteSheet>,
    mut dialogs: ResMut<DialogMap>,
    global_animations_indices: Res<GlobalAnimationIndices>,
    interaction_resources: Res<InteractionResources>,
    mut landmark_sensor_query: Query<(Entity, &mut Landmark), With<Sensor>>,
) {
    /* -------------------------------------------------------------------------- */
    /*                                    NPCs                                    */
    /* -------------------------------------------------------------------------- */

    let fabien_dialog_path = "data/fabien_dialog.yml";
    let supreme_god_dialog_path = "data/supreme_god_dialog.yml";
    let hugo_dialog_path = "data/hugo_dialog.yml";
    // let olf_dialog_path = "data/olf_dialog.yml";

    let mut npcs_infos = vec![
        (
            "Supreme God".to_string(),
            SUPREME_GOD_LINE,
            SUPREME_GOD_SPAWN_POSITION,
            Reputation::new(100, 0),
            NPCBehavior::Camping,
            Location::Temple,
            supreme_god_dialog_path,
        ),
        (
            "Hugo".to_string(),
            HEALER_V2_LINE,
            PLAYER_SPAWN,
            Reputation::new(100, 0),
            NPCBehavior::Camping,
            Location::SecretRoom,
            hugo_dialog_path,
        ),
        (
            "Vampire".to_string(),
            VAMPIRE_LINE,
            VAMPIRE_SPAWN_POSITION,
            Reputation::new(100, 0),
            NPCBehavior::LandmarkSeeking(
                // match if there is none
                reserved_random_free_landmark(&mut landmark_sensor_query, Location::Temple)
                    .unwrap(),
                Location::Temple,
            ),
            Location::Temple,
            fabien_dialog_path,
        ),
    ];
    for i in 0..5 {
        npcs_infos.push((
            format!("Fabien {}", i),
            FABIEN_LOYAL_LINE,
            FABIEN_SPAWN_POSITION,
            Reputation::new(0, 0),
            NPCBehavior::LandmarkSeeking(
                // match if there is none
                reserved_random_free_landmark(&mut landmark_sensor_query, Location::Temple)
                    .unwrap(),
                Location::Temple,
            ),
            Location::Temple,
            fabien_dialog_path,
        ));
    }

    for (name, spritesheet_line, spawn_position, reputation, behavior, location, dialog_path) in
        npcs_infos
    {
        let mut npc_animation_indices = AnimationIndices(HashMap::new());
        npc_animation_indices.insert(
            CharacterState::Run,
            global_animations_indices[spritesheet_line][0],
        );
        npc_animation_indices.insert(
            CharacterState::Idle,
            global_animations_indices[spritesheet_line][1],
        );

        let interactible = Interactible::new_npc();

        let npc = commands
            .spawn((
                SpriteSheetBundle {
                    texture_atlas: characters_spritesheet.texture_atlas.clone(),
                    transform: Transform {
                        translation: spawn_position.into(),
                        scale: Vec3::splat(NPC_SCALE),
                        ..default()
                    },
                    ..default()
                },
                Name::new(format!("NPC {}", name)),
                NPC,
                // -- Movement --
                behavior,
                MovementBundle {
                    animation_indices: npc_animation_indices,
                    ..default()
                },
                location,
                // -- Social --
                interactible,
                reputation,
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
                    Name::new(format!("{} Interaction Sensor", name)),
                ));

                parent.spawn((
                    SpriteBundle {
                        texture: interaction_resources.interact_button.clone(),
                        transform: Transform {
                            translation: interactible.icon_translation,
                            scale: Vec3::splat(INTERACT_BUTTON_SCALE),
                            ..default()
                        },
                        visibility: Visibility::Hidden,
                        ..default()
                    },
                    InteractIcon,
                ));

                parent.spawn((
                    Collider::cuboid(CHAR_HITBOX_WIDTH, CHAR_HITBOX_HEIGHT),
                    Transform::from_xyz(0., CHAR_HITBOX_Y_OFFSET, 0.),
                    CharacterHitbox,
                    Name::new(format!("{} Hitbox", name)),
                ));

                // REFACTOR: Spawn all sensor (but with a component "InactiveSensor" or query With<ActiveEvents>)

                // parent.spawn((
                //     Collider::ball(10.),
                //     Sensor,
                //     ActiveEvents::COLLISION_EVENTS,
                //     ActiveCollisionTypes::STATIC_STATIC,
                //     CharacterCloseSensor,
                //     Name::new(format!("{} Close Sensor", name)),
                // ));

                // parent.spawn((
                //     Collider::ball(60.),
                //     // ActiveEvents::COLLISION_EVENTS,
                //     Sensor,
                //     PursuitRangeSensor,
                //     Name::new(format!("{} Pursuit Range", name)),
                // ));

                // parent.spawn((
                //     Collider::ball(40.),
                //     // ActiveEvents::COLLISION_EVENTS,
                //     Sensor,
                //     DetectionRangeSensor,
                //     Name::new(format!("{} Detection Range", name)),
                // ));

                parent.spawn((
                    Collider::ball(20.),
                    // ActiveEvents::COLLISION_EVENTS,
                    Sensor,
                    FollowRangeSensor,
                    Name::new(format!("{} Follow Range", name)),
                ));
            })
            .id();

        let dialog_file = std::fs::File::open(dialog_path).unwrap();
        let npc_deserialized_map: BTreeMap<usize, DialogNode> =
            serde_yaml::from_reader(dialog_file).unwrap();
        dialogs.insert(
            npc,
            (
                *npc_deserialized_map.first_key_value().unwrap().0,
                npc_deserialized_map,
            ),
        );
    }
}

/// All vilain npc
///
/// Merge all spawn npcs function
fn spawn_vilains(
    mut commands: Commands,
    characters_spritesheet: Res<CharacterSpriteSheet>,
    mut dialogs: ResMut<DialogMap>,
    global_animations_indices: Res<GlobalAnimationIndices>,
    interaction_resources: Res<InteractionResources>,
    mut landmark_sensor_query: Query<(Entity, &mut Landmark), With<Sensor>>,
) {
    /* -------------------------------------------------------------------------- */
    /*                                   Vilains                                  */
    /* -------------------------------------------------------------------------- */

    let olf_dialog_path = "data/olf_dialog.yml";

    let npcs_infos = vec![(
        "Olf",
        OLF_LINE,
        OLF_SPAWN_POSITION,
        Reputation::new(0, 100),
        NPCBehavior::LandmarkSeeking(
            // match if there is none
            reserved_random_free_landmark(&mut landmark_sensor_query, Location::SecretRoom)
                .unwrap(),
            Location::SecretRoom,
        ),
        olf_dialog_path,
    )];

    for (name, spritesheet_line, spawn_position, reputation, behavior, dialog_path) in npcs_infos {
        let mut npc_animation_indices = AnimationIndices(HashMap::new());
        npc_animation_indices.insert(
            CharacterState::Run,
            global_animations_indices[spritesheet_line][0],
        );
        npc_animation_indices.insert(
            CharacterState::Idle,
            global_animations_indices[spritesheet_line][1],
        );

        let interactible = Interactible::new_npc();

        let npc = commands
            .spawn((
                SpriteSheetBundle {
                    texture_atlas: characters_spritesheet.texture_atlas.clone(),
                    transform: Transform {
                        translation: spawn_position.into(),
                        scale: Vec3::splat(NPC_SCALE),
                        ..default()
                    },
                    ..default()
                },
                Name::new(format!("NPC {}", name)),
                NPC,
                // -- Movement --
                behavior,
                MovementBundle {
                    animation_indices: npc_animation_indices,
                    ..default()
                },
                Location::SecretRoom,
                // -- Social --
                interactible,
                reputation,
                TargetSeeker(TargetType::Player),
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
                    Name::new(format!("{} Interaction Sensor", name)),
                ));

                parent.spawn((
                    SpriteBundle {
                        texture: interaction_resources.interact_button.clone(),
                        transform: Transform {
                            translation: interactible.icon_translation,
                            scale: Vec3::splat(INTERACT_BUTTON_SCALE),
                            ..default()
                        },
                        visibility: Visibility::Hidden,
                        ..default()
                    },
                    InteractIcon,
                ));

                parent.spawn((
                    Collider::cuboid(CHAR_HITBOX_WIDTH, CHAR_HITBOX_HEIGHT),
                    Transform::from_xyz(0., CHAR_HITBOX_Y_OFFSET, 0.),
                    CharacterHitbox,
                    Name::new(format!("{} Hitbox", name)),
                ));

                // parent.spawn((
                //     Collider::ball(10.),
                //     Sensor,
                //     ActiveEvents::COLLISION_EVENTS,
                //     ActiveCollisionTypes::STATIC_STATIC,
                //     CharacterCloseSensor,
                //     Name::new(format!("{} Close Sensor", name)),
                // ));

                parent.spawn((
                    Collider::ball(60.),
                    // ActiveEvents::COLLISION_EVENTS,
                    Sensor,
                    PursuitRangeSensor,
                    Name::new(format!("{} Pursuit Range", name)),
                ));

                parent.spawn((
                    Collider::ball(40.),
                    ActiveEvents::COLLISION_EVENTS,
                    Sensor,
                    DetectionRangeSensor,
                    Name::new(format!("{} Detection Range", name)),
                ));

                // parent.spawn((
                //     Collider::ball(20.),
                //     // ActiveEvents::COLLISION_EVENTS,
                //     Sensor,
                //     FollowRangeSensor,
                //     Name::new(format!("{} Follow Range", name)),
                // ));
            })
            .id();

        let dialog_file = std::fs::File::open(dialog_path).unwrap();
        let npc_deserialized_map: BTreeMap<usize, DialogNode> =
            serde_yaml::from_reader(dialog_file).unwrap();
        dialogs.insert(
            npc,
            (
                *npc_deserialized_map.first_key_value().unwrap().0,
                npc_deserialized_map,
            ),
        );
    }
}

fn spawn_cat(mut commands: Commands, characters_spritesheet: Res<CharacterSpriteSheet>) {
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
}

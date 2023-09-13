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
        CharacterSpriteSheet,
    },
    characters::{movement::MovementBundle, npcs::movement::NPCBehavior, CharacterHitbox},
    combat::Reputation,
    constants::character::{npcs::*, player::PLAYER_SPAWN, *},
    interactions::{Interactible, InteractionSensor},
    locations::{
        landmarks::{reserved_random_free_landmark, Landmark, LandmarkStatus},
        temple::OverlappingEntity,
    },
    ui::dialog_systems::{CurrentInterlocutor, DialogMap},
    GameState, HUDState,
};

use self::{
    aggression::{DetectionRangeSensor, PursuitRangeSensor},
    movement::FollowRangeSensor,
};

use super::movement::CharacterCloseSensor;

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
            .add_systems(OnEnter(GameState::Playing), spawn_characters)
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
                    aggression::fair_play_wait.after(NPCSystems::StopChase),
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

/// Cats and all friendly npc
fn spawn_characters(
    mut commands: Commands,
    characters_spritesheet: Res<CharacterSpriteSheet>,
    mut dialogs: ResMut<DialogMap>,
    mut landmark_sensor_query: Query<(Entity, &mut Landmark), With<Sensor>>,
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
    /*                                    NPCs                                    */
    /* -------------------------------------------------------------------------- */

    let supreme_god_dialog_path = "data/supreme_god_dialog.yml";
    // let fabien_dialog_path = "data/fabien_dialog.yml";
    // let olf_dialog_path = "data/olf_dialog.yml";

    let npcs_infos = vec![
        (
            "Supreme God",
            SUPREME_GOD_LINE,
            SUPREME_GOD_SPAWN_POSITION,
            Reputation::new(100, 0),
            supreme_god_dialog_path,
        ),
        // (
        //     "Hugo",
        //     HEALER_V2_LINE,
        //     PLAYER_SPAWN,
        //     Reputation::new(100, 0),
        //     fabien_dialog_path,
        // ),
        // (
        //     "Vampire",
        //     VAMPIRE_LINE,
        //     PLAYER_SPAWN,
        //     Reputation::new(100, 0),
        //     fabien_dialog_path,
        // ),
        // (
        //     "Olf",
        //     OLF_LINE,
        //     OLF_SPAWN_POSITION,
        //     Reputation::new(0, 100),
        //     olf_dialog_path,
        // ),
    ];

    for info in npcs_infos {
        let mut npc_animation_indices = AnimationIndices(HashMap::new());
        npc_animation_indices.insert(CharacterState::Run, global_animations_indices[info.1][0]);
        npc_animation_indices.insert(CharacterState::Idle, global_animations_indices[info.1][1]);

        // match if there is none
        // only check the landmark in their zone
        let free_random_landmark =
            reserved_random_free_landmark(&mut landmark_sensor_query).unwrap();

        let npc = commands
            .spawn((
                SpriteSheetBundle {
                    texture_atlas: characters_spritesheet.texture_atlas.clone(),
                    transform: Transform {
                        translation: info.2.into(),
                        scale: Vec3::splat(NPC_SCALE),
                        ..default()
                    },
                    ..default()
                },
                Name::new(format!("NPC {}", info.0)),
                NPC,
                // -- Movement --
                NPCBehavior::Camping,
                // NPCBehavior::LandmarkSeeking(free_random_landmark),
                MovementBundle {
                    animation_indices: npc_animation_indices,
                    ..default()
                },
                // -- Social --
                Interactible::new_npc(),
                info.3,
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
                    Name::new(format!("{} Interaction Sensor", info.0)),
                ));

                parent.spawn((
                    Collider::cuboid(CHAR_HITBOX_WIDTH, CHAR_HITBOX_HEIGHT),
                    Transform::from_xyz(0., CHAR_HITBOX_Y_OFFSET, 0.),
                    CharacterHitbox,
                    Name::new(format!("{} Hitbox", info.0)),
                ));

                // parent.spawn((
                //     Collider::ball(10.),
                //     Sensor,
                //     ActiveEvents::COLLISION_EVENTS,
                //     ActiveCollisionTypes::STATIC_STATIC,
                //     CharacterCloseSensor,
                //     Name::new(format!("{} Close Sensor", info.0)),
                // ));

                // parent.spawn((
                //     Collider::ball(60.),
                //     // ActiveEvents::COLLISION_EVENTS,
                //     Sensor,
                //     PursuitRangeSensor,
                //     Name::new(format!("{} Pursuit Range", info.0)),
                // ));

                // parent.spawn((
                //     Collider::ball(40.),
                //     // ActiveEvents::COLLISION_EVENTS,
                //     Sensor,
                //     DetectionRangeSensor,
                //     Name::new(format!("{} Detection Range", info.0)),
                // ));

                parent.spawn((
                    Collider::ball(20.),
                    // ActiveEvents::COLLISION_EVENTS,
                    Sensor,
                    FollowRangeSensor,
                    Name::new(format!("{} Follow Range", info.0)),
                ));
            })
            .id();

        let dialog_file = std::fs::File::open(info.4).unwrap();
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

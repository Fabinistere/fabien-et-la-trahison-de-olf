//! Specific zone where npcs and player can gather to be in a group
//! or occupy certain location.
//!
//! For example, it could be the throne side, statue admiration, etc.

use bevy::prelude::*;
use bevy_ecs::query::{ReadOnlyWorldQuery, WorldQuery};
use bevy_rapier2d::prelude::{Collider, CollisionEvent, Sensor};
use rand::seq::IteratorRandom;
use std::time::Duration;

use crate::{
    characters::{
        npcs::{idle::RestTime, movement::NPCBehavior, NPC},
        player::Player,
        CharacterHitbox,
    },
    collisions::CollisionEventExt,
    constants::{character::npcs::movement::REST_TIMER, locations::main_room::landmarks::*},
};

use super::temple::PlayerLocation;

pub struct LandmarkPlugin;

impl Plugin for LandmarkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            // OnEnter(GameState::Playing),
            Startup,
            spawn_landmarks,
        )
        .add_systems(Update, landmark_arrival);
    }
}

/// Npcs will try with the `NPCBehavior::LandmarkSeeking`
/// to occupy one of the free landmark.
/// The npc will reserved the landmark, only the player can occupy a reserved landmark.
/// A occupied landmark can only be freed by the occupant.
#[derive(PartialEq, Eq, Default, Component)]
pub enum LandmarkStatus {
    #[default]
    Free,
    Reserved,
    Occupied,
}

/// TEMP: if there is only the field `status` just use the enum instead.
#[derive(Component)]
pub struct Landmark {
    pub status: LandmarkStatus,
    /// DOC: ambiguous name
    pub location: PlayerLocation,
}

impl Landmark {
    pub fn new(landmark_location: PlayerLocation) -> Self {
        Landmark {
            status: LandmarkStatus::default(),
            location: landmark_location,
        }
    }
}

#[derive(Debug)]
pub enum LandmarkReservationError {
    EmptyQuery,
}

/// TODO: Create an impl to automatictly Reserved a free landmark
///
/// LandmarkReservationError
pub fn reserved_random_free_landmark(
    landmark_sensor_query: &mut Query<(Entity, &mut Landmark), With<Sensor>>,
) -> Result<Entity, LandmarkReservationError> {
    match (*landmark_sensor_query)
        .iter_mut()
        .filter(|(_, landmark)| landmark.status == LandmarkStatus::Free)
        .choose(&mut rand::thread_rng())
    {
        // create custom error
        None => Err(LandmarkReservationError::EmptyQuery),
        Some((free_random_landmark, mut landmark)) => {
            landmark.status = LandmarkStatus::Reserved;
            Ok(free_random_landmark)
        }
    }
}

#[derive(Component)]
pub struct LandmarkGroup;

/* -------------------------------------------------------------------------- */
/*                             Landmark Collision                             */
/* -------------------------------------------------------------------------- */

fn landmark_arrival(
    mut collision_events: EventReader<CollisionEvent>,
    mut commands: Commands,

    character_hitbox_query: Query<(Entity, &Parent, &Name), With<CharacterHitbox>>,
    mut npc_query: Query<(&mut NPCBehavior, &Name), With<NPC>>,
    player_query: Query<Entity, With<Player>>,

    mut landmark_sensor_query: Query<(Entity, &mut Landmark), With<Sensor>>,
    // parent_query: Query<&Parent>,
) {
    for collision_event in collision_events.iter() {
        let entity_1 = collision_event.entities().0;
        let entity_2 = collision_event.entities().1;

        match (
            character_hitbox_query.get(entity_1),
            character_hitbox_query.get(entity_2),
        ) {
            (Err(_), Ok((character_hitbox, character_parent, name)))
            | (Ok((character_hitbox, character_parent, name)), Err(_)) => {
                let potential_landmark = if character_hitbox == entity_1 {
                    entity_2
                } else {
                    entity_1
                };
                if let Ok((landmark_entity, mut landmark)) =
                    landmark_sensor_query.get_mut(potential_landmark)
                {
                    if let Ok((mut behavior, _npc_name)) = npc_query.get_mut(**character_parent) {
                        if let NPCBehavior::LandmarkSeeking(landmark_destination) = *behavior {
                            if landmark_destination == landmark_entity {
                                match landmark.status {
                                    LandmarkStatus::Occupied => {
                                        // send an event to redirect the npc
                                        warn!("This landmark {:?} was claimed before the NPC {:?} arrived", landmark_entity, **character_parent)
                                    }
                                    _ => {
                                        landmark.status = LandmarkStatus::Occupied;
                                        // TODO: Or start dialog with the other
                                        info!(target: "Start Rest", "{:?}, {}", **character_parent, name);
                                        commands.entity(**character_parent).insert(RestTime {
                                            timer: Timer::new(
                                                Duration::from_secs(REST_TIMER),
                                                TimerMode::Once,
                                            ),
                                        });
                                        let next_destination = reserved_random_free_landmark(
                                            &mut landmark_sensor_query,
                                        )
                                        .unwrap();

                                        *behavior = NPCBehavior::LandmarkSeeking(next_destination);
                                    }
                                }
                            }
                        }
                    } else if player_query.get(**character_parent).is_ok() {
                        match landmark.status {
                            LandmarkStatus::Occupied => {}
                            LandmarkStatus::Free | LandmarkStatus::Reserved => {
                                landmark.status = LandmarkStatus::Occupied
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                               Spawn Landmarks                              */
/* -------------------------------------------------------------------------- */

fn spawn_landmarks(mut commands: Commands) {
    commands
        .spawn(TransformBundle::default())
        .with_children(|parent| {
            parent
                .spawn((
                    LandmarkGroup,
                    TransformBundle::default(),
                    Name::new("Cat Statue Discussion Group"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Landmark::new(PlayerLocation::Temple),
                        TransformBundle::from_transform(Transform::from_translation(
                            LANDMARK_CAT_STATUE_LEFT.into(),
                        )),
                        Collider::ball(5.),
                        Sensor,
                        Name::new("Landmark Cat Statue Left"),
                    ));
                    parent.spawn((
                        Landmark::new(PlayerLocation::Temple),
                        TransformBundle::from_transform(Transform::from_translation(
                            LANDMARK_CAT_STATUE_MIDDLE.into(),
                        )),
                        Collider::ball(5.),
                        Sensor,
                        Name::new("Landmark Cat Statue Middle"),
                    ));
                    parent.spawn((
                        Landmark::new(PlayerLocation::Temple),
                        TransformBundle::from_transform(Transform::from_translation(
                            LANDMARK_CAT_STATUE_RIGHT.into(),
                        )),
                        Collider::ball(5.),
                        Sensor,
                        Name::new("Landmark Cat Statue Right"),
                    ));
                });
            parent
                .spawn((
                    LandmarkGroup,
                    TransformBundle::default(),
                    Name::new("Fabien Statue Discussion Group"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Landmark::new(PlayerLocation::Temple),
                        TransformBundle::from_transform(Transform::from_translation(
                            LANDMARK_FABIEN_STATUE_LEFT.into(),
                        )),
                        Collider::ball(5.),
                        Sensor,
                        Name::new("Landmark Fabien Statue Left"),
                    ));
                    parent.spawn((
                        Landmark::new(PlayerLocation::Temple),
                        TransformBundle::from_transform(Transform::from_translation(
                            LANDMARK_FABIEN_STATUE_MIDDLE.into(),
                        )),
                        Collider::ball(5.),
                        Sensor,
                        Name::new("Landmark Fabien Statue Middle"),
                    ));
                    parent.spawn((
                        Landmark::new(PlayerLocation::Temple),
                        TransformBundle::from_transform(Transform::from_translation(
                            LANDMARK_FABIEN_STATUE_RIGHT.into(),
                        )),
                        Collider::ball(5.),
                        Sensor,
                        Name::new("Landmark Fabien Statue Right"),
                    ));
                });
        });
}

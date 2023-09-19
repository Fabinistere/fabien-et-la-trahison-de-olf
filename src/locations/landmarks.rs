//! Specific zone where npcs and player can gather to be in a group
//! or occupy certain location.
//!
//! For example, it could be the throne side, statue admiration, etc.

use bevy::prelude::*;
use bevy_rapier2d::prelude::{ActiveEvents, Collider, CollisionEvent, Sensor};
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

use super::temple::Location;

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
#[derive(PartialEq, Eq, Debug, Reflect, Default, Component)]
pub enum LandmarkStatus {
    #[default]
    Free,
    Reserved,
    OccupiedBy(Entity),
}

#[derive(Clone, Copy, Reflect, Debug, Component)]
pub enum Direction {
    Right,
    Left,
}

impl From<Direction> for bool {
    fn from(direction: Direction) -> bool {
        match direction {
            Direction::Left => true,
            Direction::Right => false,
        }
    }
}

#[derive(Reflect, Debug, Component)]
pub struct Landmark {
    pub status: LandmarkStatus,
    pub location: Location,
    /// Used for `flip_x`.
    pub direction: Option<Direction>,
}

impl Landmark {
    pub fn new(landmark_location: Location, direction: Option<Direction>) -> Self {
        Landmark {
            status: LandmarkStatus::default(),
            location: landmark_location,
            direction,
        }
    }
}

#[derive(Debug)]
pub enum LandmarkReservationError {
    NoFreeLandmarks,
}

/// TODO: Create an impl to automatictly Reserved a free landmark
///
/// LandmarkReservationError
pub fn reserved_random_free_landmark(
    landmark_sensor_query: &mut Query<(Entity, &mut Landmark), With<Sensor>>,
    location: Location,
) -> Result<Entity, LandmarkReservationError> {
    match (*landmark_sensor_query)
        .iter_mut()
        .filter(|(_, landmark)| {
            landmark.status == LandmarkStatus::Free && landmark.location == location
        })
        .choose(&mut rand::thread_rng())
    {
        None => Err(LandmarkReservationError::NoFreeLandmarks),
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

/// NOTE: To force NPCs to one particular spot of the landmark, create/use a `CharacterPrecisePosition` with is a sensor limited to point
fn landmark_arrival(
    mut collision_events: EventReader<CollisionEvent>,
    mut commands: Commands,

    character_hitbox_query: Query<(Entity, &Parent, &Name), With<CharacterHitbox>>,
    mut npc_query: Query<(Entity, &mut NPCBehavior, &Name), With<NPC>>,
    player_query: Query<Entity, With<Player>>,

    mut landmark_sensor_query: Query<(Entity, &mut Landmark), With<Sensor>>,
    // parent_query: Query<&Parent>,
) {
    for collision_event in collision_events.iter() {
        // info!("{:#?}", collision_event);
        let (entity_1, entity_2) = collision_event.entities();

        if let (Err(_), Ok((character_hitbox, character_parent, _name)))
        | (Ok((character_hitbox, character_parent, _name)), Err(_)) = (
            character_hitbox_query.get(entity_1),
            character_hitbox_query.get(entity_2),
        ) {
            let potential_landmark = if character_hitbox == entity_1 {
                entity_2
            } else {
                entity_1
            };
            if let Ok((landmark_entity, mut landmark)) =
                landmark_sensor_query.get_mut(potential_landmark)
            {
                if let Ok((npc, mut behavior, _npc_name)) = npc_query.get_mut(**character_parent) {
                    if let NPCBehavior::LandmarkSeeking(landmark_destination, location) = *behavior
                    {
                        // A npc enters/exits a landmark sensor
                        match landmark.status {
                            LandmarkStatus::OccupiedBy(occupant) => {
                                if collision_event.is_started()
                                    && landmark_destination == landmark_entity
                                {
                                    // info!("This landmark {:?} was claimed before the NPC {:?} arrived", landmark_entity, **character_parent)
                                    let next_destination = reserved_random_free_landmark(
                                        &mut landmark_sensor_query,
                                        location,
                                    )
                                    .unwrap();
                                    *behavior =
                                        NPCBehavior::LandmarkSeeking(next_destination, location);
                                } else if collision_event.is_stopped() && occupant == npc {
                                    landmark.status = LandmarkStatus::Free;
                                    commands.entity(**character_parent).remove::<Direction>();
                                }
                            }
                            _ => {
                                if collision_event.is_started()
                                    && landmark_destination == landmark_entity
                                {
                                    landmark.status = LandmarkStatus::OccupiedBy(npc);

                                    if let Some(forced_direction) = landmark.direction {
                                        info!("Forced Direction for {npc:?}: {forced_direction:?}",);
                                        commands
                                            .entity(**character_parent)
                                            .insert(forced_direction);
                                    }
                                    // TODO: Or start dialog with the other
                                    // info!(target: "Start Rest", "{:?}, {}", **character_parent, _name);
                                    commands.entity(**character_parent).insert(RestTime {
                                        timer: Timer::new(
                                            Duration::from_secs(REST_TIMER),
                                            TimerMode::Once,
                                        ),
                                    });
                                    let next_destination = reserved_random_free_landmark(
                                        &mut landmark_sensor_query,
                                        location,
                                    )
                                    .unwrap();
                                    *behavior =
                                        NPCBehavior::LandmarkSeeking(next_destination, location);
                                }
                            }
                        }
                    }
                } else if player_query.get(**character_parent).is_ok() {
                    // The player enters/exits a landmark
                    match landmark.status {
                        LandmarkStatus::OccupiedBy(occupant) => {
                            if collision_event.is_stopped() && occupant == **character_parent {
                                landmark.status = LandmarkStatus::Free;
                            }
                        }
                        LandmarkStatus::Free | LandmarkStatus::Reserved => {
                            if collision_event.is_started() {
                                landmark.status = LandmarkStatus::OccupiedBy(**character_parent)
                            }
                        }
                    }
                }
            }
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                               Spawn Landmarks                              */
/* -------------------------------------------------------------------------- */

fn spawn_landmarks(mut commands: Commands) {
    commands
        .spawn((TransformBundle::default(), Name::new("Landmarks")))
        .with_children(|parent| {
            let landmark_sensor = (
                Collider::ball(LANDMARK_SENSOR_SIZE),
                ActiveEvents::COLLISION_EVENTS,
                Sensor,
            );

            /* -------------------------------------------------------------------------- */
            /*                                   Groups                                   */
            /* -------------------------------------------------------------------------- */

            for (group, group_name) in LANDMARK_GROUPS {
                parent
                    .spawn((
                        LandmarkGroup,
                        TransformBundle::default(),
                        Name::new(format!("{group_name} Discussion Group")),
                    ))
                    .with_children(|parent| {
                        for (position, landmark_name, landmark_direction) in group {
                            parent.spawn((
                                Landmark::new(Location::Temple, landmark_direction),
                                TransformBundle::from_transform(Transform::from_translation(
                                    position.into(),
                                )),
                                Name::new(format!("Landmark {group_name} {landmark_name}")),
                                landmark_sensor.clone(),
                            ));
                        }
                    });
            }

            /* -------------------------------------------------------------------------- */
            /*                                 Singletons                                 */
            /* -------------------------------------------------------------------------- */

            for (position, landmark_name, landmark_direction) in LANDMARK_SINGLETONS {
                parent.spawn((
                    Landmark::new(Location::Temple, landmark_direction),
                    TransformBundle::from_transform(Transform::from_translation(position.into())),
                    Name::new(format!("Landmark {landmark_name}")),
                    landmark_sensor.clone(),
                ));
            }

            for (landmarks, pillar_name) in LANDMARK_PILLARS {
                parent
                    .spawn((
                        TransformBundle::default(),
                        Name::new(format!("{pillar_name}'s landmarks")),
                    ))
                    .with_children(|parent| {
                        for (position, landmark_name, landmark_direction) in landmarks {
                            parent.spawn((
                                Landmark::new(Location::Temple, landmark_direction),
                                TransformBundle::from_transform(Transform::from_translation(
                                    position.into(),
                                )),
                                Name::new(format!("Landmark {pillar_name} {landmark_name}")),
                                landmark_sensor.clone(),
                            ));
                        }
                    });
            }
        });
}

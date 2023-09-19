//! Implements Npc for moving and steering entities.

use bevy::prelude::*;
use bevy_ecs::query::QueryEntityError;
use bevy_rapier2d::prelude::{
    ActiveEvents, Collider, CollisionEvent, RapierContext, Sensor, Velocity,
};

use crate::{
    animations::sprite_sheet_animation::CharacterState,
    // GameState,
    characters::{
        movement::{CharacterCloseSensor, Speed},
        npcs::{
            aggression::{
                DetectionRangeSensor, EngagePursuitEvent, PursuitRangeSensor, StopChaseEvent,
            },
            idle::RestTime,
            NPC,
        },
        player::Player,
        CharacterHitbox,
    },
    collisions::CollisionEventExt,
    combat::{CombatEvent, FairPlayTimer, Reputation},
    constants::character::CHAR_HITBOX_Y_OFFSET,
    locations::{
        landmarks::{reserved_random_free_landmark, Direction, Landmark, LandmarkStatus},
        temple::Location,
    },
};

// pub const PROXIMITY_RADIUS: f32 = 64.;

/* -------------------------------------------------------------------------- */
/*                                 Components                                 */
/* -------------------------------------------------------------------------- */

#[derive(PartialEq, Clone, Copy, Reflect, Component)]
pub enum NPCBehavior {
    /// The entity runs to a specify location and occupy this zone.
    /// Tourist butterfly.
    LandmarkSeeking(Entity, Location),
    Camping,
    Follow {
        target: Entity,
        /// DOC: Modify by sensor collisions in `???`
        close: bool,
    },
}

impl NPCBehavior {
    pub fn follow(target: Entity, close: bool) -> Self {
        NPCBehavior::Follow { target, close }
    }
}

impl Default for NPCBehavior {
    fn default() -> Self {
        Self::Camping
    }
}

#[derive(Reflect, Component)]
pub struct TargetSeeker(pub TargetType);

#[derive(Reflect)]
pub enum TargetType {
    /// can be merge with `TargetType::Special(Entity)`
    Player,
    Ally,
    Enemy,
    Special(Entity),
}

#[derive(Debug, Reflect, Component)]
pub struct Chaser {
    pub target: Entity,
    /// DOC: Modify by sensor collisions in `???`
    pub close: bool,
}

impl Chaser {
    pub fn new(target: Entity) -> Self {
        Chaser {
            target,
            close: false,
        }
    }
}

#[derive(Component)]
pub struct FollowRangeSensor;

/* -------------------------------------------------------------------------- */
/*                                   Events                                   */
/* -------------------------------------------------------------------------- */

#[derive(Event)]
pub struct FollowEvent {
    /// In many situation, could be the interlocutor
    pub npc: Entity,
    pub target: Entity,
}

/* -------------------------------------------------------------------------- */
/*                                   Systems                                  */
/* -------------------------------------------------------------------------- */

// REFACTOR: The whole movement plugin ([x] close sensor, smooth movement)

/// Detect any change from the npcs about their behavior
pub fn npc_behavior_change(
    mut commands: Commands,
    npc_query: Query<(&NPCBehavior, &Children), (Changed<NPCBehavior>, With<NPC>)>,
    follow_sensor_query: Query<Entity, (With<FollowRangeSensor>, With<Collider>, With<Sensor>)>,
) {
    for (behavior, children) in &npc_query {
        match behavior {
            NPCBehavior::Follow { .. } => {
                for child in children {
                    if let Ok(follow_range) = follow_sensor_query.get(*child) {
                        commands
                            .entity(follow_range)
                            .insert(ActiveEvents::COLLISION_EVENTS);
                        break;
                    }
                }
            }
            _ => {
                for child in children {
                    if let Ok(follow_range) = follow_sensor_query.get(*child) {
                        commands.entity(follow_range).remove::<ActiveEvents>();
                        break;
                    }
                }
            }
        }
    }
}

pub fn follow_event(
    mut follow_event: EventReader<FollowEvent>,
    rapier_context: Res<RapierContext>,

    mut npc_query: Query<(&mut NPCBehavior, &Children), With<NPC>>,
    follow_sensor_query: Query<Entity, (With<FollowRangeSensor>, With<Collider>, With<Sensor>)>,
    target_children_query: Query<&Children>,
    character_hitbox_query: Query<Entity, With<CharacterHitbox>>,
) {
    for FollowEvent { npc, target } in follow_event.iter() {
        let (mut behavior, npc_children) = npc_query.get_mut(*npc).unwrap();
        let mut npc_follow_range: Result<Entity, _> = Err(QueryEntityError::NoSuchEntity(*npc));
        // FIXME: throw the correct error not this
        for child in npc_children {
            if follow_sensor_query.get(*child).is_ok() {
                npc_follow_range = Ok(*child);
                break;
            }
        }

        let target_children = target_children_query.get(*target).unwrap();
        // FIXME: throw the correct error not this
        let mut target_hitbox: Result<Entity, _> = Err(QueryEntityError::NoSuchEntity(*target));
        for child in target_children {
            if character_hitbox_query.get(*child).is_ok() {
                target_hitbox = Ok(*child);
                break;
            }
        }
        *behavior = NPCBehavior::follow(
            *target,
            rapier_context.intersection_pair(npc_follow_range.unwrap(), target_hitbox.unwrap())
                == Some(true),
        );
    }
}

pub fn animation(
    mut npc_query: Query<
        (
            &Velocity,
            &mut CharacterState,
            &mut TextureAtlasSprite,
            Option<&Direction>,
        ),
        (Or<(Changed<Velocity>, Changed<Direction>)>, With<NPC>),
    >,
) {
    for (rb_vel, mut npc_state, mut texture_atlas_sprite, potential_forced_direction) in
        &mut npc_query
    {
        /* -------------------------------------------------------------------------- */
        /*                                  Animation                                 */
        /* -------------------------------------------------------------------------- */

        // if there is any movement
        if rb_vel.linvel.x != 0. && rb_vel.linvel.y != 0. && *npc_state != CharacterState::Run {
            *npc_state = CharacterState::Run;
        } else if rb_vel.linvel.x == 0.
            && rb_vel.linvel.y == 0.
            && *npc_state == CharacterState::Run
            && *npc_state != CharacterState::Idle
        {
            // IDEA: Polish #visual - When we reach max speed (one full run loop), whenever you stop there is a smoke anim (sudden braking)
            *npc_state = CharacterState::Idle;
        }

        /* -------------------------------------------------------------------------- */
        /*                                  Direction                                 */
        /* -------------------------------------------------------------------------- */

        match potential_forced_direction {
            None => {
                if rb_vel.linvel.x > 0. {
                    texture_atlas_sprite.flip_x = false;
                } else if rb_vel.linvel.x < 0. {
                    texture_atlas_sprite.flip_x = true;
                }
            }
            Some(direction) => texture_atlas_sprite.flip_x = (*direction).into(),
        }
    }
}

// TODO: feature - use ColliderType::Sensor to delimiter zone

pub fn npc_movement(
    mut npc_query: Query<
        (
            &mut NPCBehavior,
            Option<&Chaser>,
            &Transform,
            &Speed,
            &mut Velocity,
        ),
        Without<RestTime>,
    >,
    mut landmark_sensor_query: Query<(Entity, &mut Landmark), With<Sensor>>,
    pos_query: Query<&GlobalTransform>,
) {
    for (mut behavior, potential_chaser, transform, speed, mut rb_vel) in &mut npc_query {
        let (vel_x, vel_y) = match potential_chaser {
            None => match *behavior {
                NPCBehavior::Camping => (0., 0.),
                NPCBehavior::LandmarkSeeking(destination, location) => {
                    let (_, landmark) = landmark_sensor_query.get(destination).unwrap();
                    let landmark_transform = pos_query.get(destination).unwrap();
                    match landmark.status {
                        LandmarkStatus::OccupiedBy(_) => {
                            // FIXME: Match the LandmarkReservationError
                            let next_destination =
                                reserved_random_free_landmark(&mut landmark_sensor_query, location)
                                    .unwrap();
                            *behavior = NPCBehavior::LandmarkSeeking(next_destination, location);
                            let next_transform = pos_query.get(next_destination).unwrap();
                            move_to(next_transform, false, transform, speed)
                        }
                        _ => move_to(landmark_transform, false, transform, speed),
                    }
                }
                NPCBehavior::Follow { target, close } => {
                    if close {
                        (0., 0.)
                    } else {
                        let target_transform = pos_query.get(target).unwrap();
                        move_to(target_transform, true, transform, speed)
                    }
                }
            },
            Some(Chaser { target, close }) => {
                if *close {
                    (0., 0.)
                } else {
                    let target_transform = pos_query.get(*target).unwrap();
                    move_to(target_transform, true, transform, speed)
                }
            }
        };

        rb_vel.linvel.x = vel_x;
        rb_vel.linvel.y = vel_y;
    }
}

/// # Chase Management
///
/// - Follow Close Update.
///   If a character enters/exits the npc's `FollowRangeSensor`,
///   it will update the `close` field in the npc's behavior.
/// - Chase Failed.
///   If a character exits the npc's `PursuitRangeSensor`,
///   and is the target of the npc, the npc will disengage with them.
/// - Chase Completed.
///   If a npc enters the `CharacterCloseSensor` of their target,
///   it will trigger the `CombatEvent`.
/// - Target Detection.
///   If a character enters the `DetectionRangeSensor` of an enemy npc,
///   seeking of targets, the npc will engage with the character.
pub fn chase_management(
    mut collision_events: EventReader<CollisionEvent>,
    // rapier_context: Res<RapierContext>,
    character_hitbox_query: Query<(&Parent, &Name), With<CharacterHitbox>>,
    player_query: Query<Entity, With<Player>>,
    mut npc_query: Query<(&mut NPCBehavior, Option<&mut Chaser>, &Name), With<NPC>>,
    target_seeker_query: Query<&TargetSeeker>,
    fair_play_timer_query: Query<Entity, With<FairPlayTimer>>,
    reputation_query: Query<&Reputation>,

    follow_sensor_query: Query<
        (Entity, &Parent),
        (
            With<FollowRangeSensor>,
            Without<PursuitRangeSensor>,
            Without<DetectionRangeSensor>,
        ),
    >,
    pursuit_sensor_query: Query<
        (Entity, &Parent),
        (
            With<PursuitRangeSensor>,
            Without<FollowRangeSensor>,
            Without<DetectionRangeSensor>,
        ),
    >,
    detection_sensor_query: Query<
        (Entity, &Parent),
        (
            With<DetectionRangeSensor>,
            Without<PursuitRangeSensor>,
            Without<FollowRangeSensor>,
        ),
    >,
    close_sensor_query: Query<(Entity, &Parent), With<CharacterCloseSensor>>,

    mut ev_engage_pursuit: EventWriter<EngagePursuitEvent>,
    mut ev_combat: EventWriter<CombatEvent>,
    mut ev_stop_chase: EventWriter<StopChaseEvent>,
) {
    for collision_event in collision_events.iter() {
        // info!("{:#?}", collision_event);
        let (entity_1, entity_2) = collision_event.entities();

        // if rapier_context.intersection_pair(entity_1, entity_2) == Some(true) {
        //     info!("Some(true) with {:#?}, {:#?}", entity_1, entity_2);
        // } else if rapier_context.intersection_pair(entity_1, entity_2) == Some(false) {
        //     info!("Some(false) with {:#?}, {:#?}", entity_1, entity_2);
        // }

        match (
            character_hitbox_query.get(entity_1),
            character_hitbox_query.get(entity_2),
        ) {
            (Err(_), Ok((character, character_name)))
            | (Ok((character, character_name)), Err(_)) => {
                match npc_query.get_mut(**character) {
                    Err(_) => {
                        /* -------------------------------------------------------------------------- */
                        /*                             Follow Close Update                            */
                        /* -------------------------------------------------------------------------- */
                        match (
                            follow_sensor_query.get(entity_1),
                            follow_sensor_query.get(entity_2),
                        ) {
                            (Ok((_follow_sensor, npc)), Err(_))
                            | (Err(_), Ok((_follow_sensor, npc))) => {
                                let (mut behavior, potential_chaser, _npc_name) =
                                    npc_query.get_mut(**npc).unwrap();
                                if potential_chaser.is_none() {
                                    if let NPCBehavior::Follow { target, close: _ } = *behavior {
                                        if target == **character {
                                            // The npc has their target entering their FollowRangeSensor
                                            *behavior = NPCBehavior::Follow {
                                                target,
                                                close: collision_event.is_started(),
                                            };
                                            // info!(
                                            //     "Follow Behavior: {}",
                                            //     collision_event.is_started()
                                            // );
                                        }
                                    }
                                }
                                continue;
                            }
                            _ => {}
                        }

                        /* -------------------------------------------------------------------------- */
                        /*                               Or Chase Failed                              */
                        /* -------------------------------------------------------------------------- */
                        if collision_event.is_stopped() {
                            match (
                                pursuit_sensor_query.get(entity_1),
                                pursuit_sensor_query.get(entity_2),
                            ) {
                                (Ok((_pursuit_sensor, npc)), Err(_))
                                | (Err(_), Ok((_pursuit_sensor, npc))) => {
                                    let (_, potential_chaser, npc_name) =
                                        npc_query.get_mut(**npc).unwrap();
                                    match potential_chaser {
                                        None => {}
                                        Some(chaser) => {
                                            if chaser.target == **character {
                                                // The npc has their target leaving their `PursuitRangeSensor`
                                                ev_stop_chase
                                                    .send(StopChaseEvent { npc_entity: **npc });
                                                info!(
                                                    "{} outran {}: chase canceled",
                                                    character_name, npc_name
                                                );
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        /* -------------------------------------------------------------------------- */
                        /*                           Or Detection Triggered                           */
                        /* -------------------------------------------------------------------------- */
                        else if collision_event.is_started() {
                            match (
                                detection_sensor_query.get(entity_1),
                                detection_sensor_query.get(entity_2),
                            ) {
                                (Ok((_detection_sensor, npc)), Err(_))
                                | (Err(_), Ok((_detection_sensor, npc))) => {
                                    let (_, potential_chaser, npc_name) =
                                        npc_query.get(**npc).unwrap();
                                    if potential_chaser.is_none()
                                        && target_seeker_query.get(**npc).is_ok()
                                        && fair_play_timer_query.get(**npc).is_err()
                                    {
                                        let TargetSeeker(target_type) =
                                            target_seeker_query.get(**npc).unwrap();
                                        // The npc has a potential target entering their DetectionRangeSensor
                                        let [npc_reputation, character_reputation] =
                                            reputation_query
                                                .get_many([**npc, **character])
                                                .unwrap();
                                        // DOC: new name ?
                                        let is_target_type = match target_type {
                                            TargetType::Player => {
                                                player_query.get(**character).is_ok()
                                            }
                                            TargetType::Enemy => !npc_reputation
                                                .in_the_same_team(character_reputation),
                                            TargetType::Ally => npc_reputation
                                                .in_the_same_team(character_reputation),
                                            TargetType::Special(target) => *target == **character,
                                        };

                                        if is_target_type {
                                            ev_engage_pursuit.send(EngagePursuitEvent {
                                                npc_entity: **npc,
                                                target_entity: **character,
                                            });
                                            info!(
                                                "{} detected {}: chase initialized",
                                                npc_name, character_name
                                            );
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    Ok((_, potential_chaser, npc_name)) => {
                        /* -------------------------------------------------------------------------- */
                        /*                               Chase Completed                              */
                        /* -------------------------------------------------------------------------- */

                        match potential_chaser {
                            None => {}
                            Some(mut chaser) => {
                                match (
                                    close_sensor_query.get(entity_1),
                                    close_sensor_query.get(entity_2),
                                ) {
                                    (Ok((_close_sensor, closed_character)), Err(_))
                                    | (Err(_), Ok((_close_sensor, closed_character))) => {
                                        if chaser.target == **closed_character
                                            && collision_event.is_started()
                                        {
                                            // The npc entered the close sensor of their target
                                            chaser.close = true;
                                            ev_combat.send(CombatEvent {
                                                entity: **character,
                                            });
                                            ev_stop_chase.send(StopChaseEvent {
                                                npc_entity: **character,
                                            });
                                            info!(
                                                "Target Caught in 4K by {:?} {}",
                                                character, npc_name
                                            );

                                            // handle flee when pressing o or moving ? (timer on npc before rechase)
                                            // 'global' timer (on player)
                                            // atm just pressing o won't make you free cause still in the DetectionRangeSensor
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

/// Give velocity x and y value to move forward a certain target
fn move_to(
    target_transform: &GlobalTransform,
    target_is_a_character: bool,
    transform: &Transform,
    speed: &Speed,
) -> (f32, f32) {
    // REFACTOR: use the max_step possible and see if the difference can be lowered.
    let target_y_offset = if target_is_a_character {
        CHAR_HITBOX_Y_OFFSET
    } else {
        0.
    };

    let up = target_transform.translation().y + target_y_offset
        > transform.translation.y + CHAR_HITBOX_Y_OFFSET;
    let down = target_transform.translation().y + target_y_offset
        < transform.translation.y + CHAR_HITBOX_Y_OFFSET;
    let left = target_transform.translation().x < transform.translation.x;
    let right = target_transform.translation().x > transform.translation.x;

    let x_axis = -(left as i8) + right as i8;
    let y_axis = -(down as i8) + up as i8;

    // println!("x: {}, y: {}", x_axis, y_axis);

    let mut vel_x = x_axis as f32 * **speed;
    let mut vel_y = y_axis as f32 * **speed;

    if x_axis != 0 && y_axis != 0 {
        vel_x *= (std::f32::consts::PI / 4.).cos();
        vel_y *= (std::f32::consts::PI / 4.).cos();
    }

    (vel_x, vel_y)
}

use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
// use log::info;

use crate::{
    characters::{
        npcs::{
            movement::{DetectionBehavior, PursuitBehavior, Target},
            NPC,
        },
        player::Player,
        CharacterHitbox,
    },
    collisions::CollisionEventExt,
    combat::{FairPlayTimer, InCombat, Reputation},
    constants::character::npc::movement::EVASION_TIMER,
};

#[derive(Component)]
pub struct DetectionSensor;

#[derive(Component)]
pub struct PursuitSensor;

/// Happens when:
///   - npc::movement::pursue
///     - target is not found/exist
///     - target is reach
/// Read in npc::aggression::remove_pursuit_urge
#[derive(Event)]
pub struct StopChaseEvent {
    pub npc_entity: Entity,
}

/// Happens when:
///   - npc::aggression::remove_pursuit_urge
///     - restablish -*dominance*- the detection behavior
///       over the pursuit beh
/// Read in
///   - npc::aggression::add_detection_aura
///     - creates THE DetectionSensor in the entity
#[derive(Event)]
pub struct DetectionModeEvent {
    pub entity: Entity,
}

/// Happens when:
///   - npc::aggression::threat_detection
///     - An npc detected a enemy
/// Read in
///   - npc::aggression::add_pursuit_urge
///     - remove DetectionBehavior from the entity
///     - insert PursuitBehavior into the entity
///     - insert the Target into the entity
#[derive(Event)]
pub struct EngagePursuitEvent {
    npc_entity: Entity,
    detection_sensor_entity: Entity,
    target_entity: Entity,
}

/// Pursuit Management
///
///   - Engagement
///     - targeting after the detection event
///       - Even if the FairPlayTimer ended:
///         As the entity doesn't stop the collision to restart it,
///         (quit n enter the detection circle)
///         with themself and the DetectionSensor,
///         the npc won't start pursue/chase
///         (wait for you to hide)
///   - Disengagement
///     - If the target outran the chaser remove
pub fn threat_detection(
    mut ev_engage_pursuit: EventWriter<EngagePursuitEvent>,
    mut ev_stop_pursuit: EventWriter<StopChaseEvent>,

    rapier_context: Res<RapierContext>,

    mut collision_events: EventReader<CollisionEvent>,
    collider_detection_sensor_query: Query<
        (Entity, &Parent),
        (With<Collider>, With<Sensor>, With<DetectionSensor>),
    >,
    collider_pursuit_sensor_query: Query<
        (Entity, &Parent),
        (With<Collider>, With<Sensor>, With<PursuitSensor>),
    >,
    collider_query: Query<(Entity, &Parent), (With<Collider>, With<CharacterHitbox>)>,

    target_query: Query<
        (Entity, &Reputation, &Name),
        (Or<(With<Player>, With<NPC>)>, Without<InCombat>),
    >,
    leader_query: Query<
        (Entity, &Reputation, &Name),
        (
            With<NPC>,
            With<DetectionBehavior>,
            Without<PursuitBehavior>,
            Without<FairPlayTimer>,
        ),
    >,
    pursuit_npc_query: Query<
        (Entity, &Reputation, &Name),
        (
            With<NPC>,
            With<PursuitBehavior>,
            Without<DetectionBehavior>,
            Without<FairPlayTimer>,
        ),
    >,
) {
    for collision_event in collision_events.iter() {
        let entity_1 = collision_event.entities().0;
        let entity_2 = collision_event.entities().1;

        // REFACTOR: find a nicer solution instead of this "copy paste"
        // it's pbly opti cause (TODO: complexity calc) enter in the if or the else if

        // one of these two colliders is a sensor && are in collision
        if rapier_context.intersection_pair(entity_1, entity_2) == Some(true) {
            // DEBUG: info!(target: "Collision Event with a sensor involved", "{:?} and {:?}", entity_1, entity_2);

            // check if the sensor is a DetectionSensor
            match (
                collider_detection_sensor_query.get(entity_1),
                collider_detection_sensor_query.get(entity_2),
                collider_query.get(entity_1),
                collider_query.get(entity_2),
            ) {
                // only one of them contains DetectionSensor: sensor_potential_npc
                // and the other one is a hitbox_potential_threat
                (Ok(sensor_potential_npc), Err(_e1), Err(_e2), Ok(hitbox_potential_threat))
                | (Err(_e1), Ok(sensor_potential_npc), Ok(hitbox_potential_threat), Err(_e2)) => {
                    // DEBUG: info!(target: "Collision with a sensor and a hitbox", "{:?} and {:?}", sensor_potential_npc, hitbox_potential_threat);

                    // [sensor_potential_npc, hitbox_potential_threat].1 returns the Parent Entity

                    // from the collider get their parent
                    match (
                        leader_query.get(sensor_potential_npc.1.get()),
                        target_query.get(hitbox_potential_threat.1.get()),
                    ) {
                        (
                            Ok((npc_entity, npc_reputation, npc_name)),
                            Ok((target_entity, target_reputation, target_name)),
                        ) => {
                            // DEBUG: info!(target: "Collision with a npc and a character", "{:?} and {:?}", npc.0, target.0);

                            // add the potential_threat as a target if not in the same team
                            if npc_reputation.in_the_same_team(target_reputation) {
                                info!("{} detected {}: chase initialized", npc_name, target_name);

                                // turn off npc detection sensor
                                // turn on npc pursuit sensor
                                // insert the new target into the npc
                                ev_engage_pursuit.send(EngagePursuitEvent {
                                    npc_entity: npc_entity,
                                    detection_sensor_entity: sensor_potential_npc.0,
                                    target_entity: target_entity,
                                });
                            }
                            // else {
                            //     info!("{} detected {}: same team", npc.2, target.2);
                            //     continue
                            // }
                        }

                        // not our manners (not a npc OR not a potential target)
                        // (Err(e), _) => warn!(target: "Not an NPC", "err: {:?}", e),
                        // (_, Err(e)) => warn!(target: "Not an Targeable Entity", "err: {:?}", e),
                        _ => continue,
                    }
                }
                // two are sensors
                // two are errors
                _ => continue,
            }
        }
        // these two colliders have stopped their collision
        else if collision_event.is_stopped() {
            // check if the sensor is a PursuitSensor
            match (
                collider_pursuit_sensor_query.get(entity_1),
                collider_pursuit_sensor_query.get(entity_2),
                collider_query.get(entity_1),
                collider_query.get(entity_2),
            ) {
                // only one of them contains PursuitSensor: sensor_potential_npc
                // and the other one is a hitbox_potential_threat
                (Ok(sensor_potential_npc), Err(_e1), Err(_e2), Ok(hitbox_potential_threat))
                | (Err(_e1), Ok(sensor_potential_npc), Ok(hitbox_potential_threat), Err(_e2)) => {
                    // DEBUG: info!(target: "Collision with a sensor and a hitbox", "{:?} and {:?}", sensor_potential_npc, hitbox_potential_threat);

                    // [sensor_potential_npc, hitbox_potential_threat].1 returns the Parent Entity

                    // from the collider get their parent
                    match (
                        pursuit_npc_query.get(sensor_potential_npc.1.get()),
                        target_query.get(hitbox_potential_threat.1.get()),
                    ) {
                        (
                            Ok((npc_entity, npc_team, npc_name)),
                            Ok((_target_entity, target_team, target_name)),
                        ) => {
                            // DEBUG: info!(target: "Collision with a npc and a character", "{:?} and {:?}", npc.0, target.0);

                            // [npc, target].0: Entity
                            // [npc, target].1: &Team
                            // [npc, target].2: &Name
                            // add the potential_threat as a target if not in the same team
                            if target_team != npc_team {
                                info!("{} outran {}: chase canceled", target_name, npc_name);

                                ev_stop_pursuit.send(StopChaseEvent { npc_entity });
                            }
                            // else {
                            //     info!("{} detected {}: same team", npc.2, target.2);
                            //     continue
                            // }
                        }

                        // not our manners (not a npc OR not a potential target)
                        // (Err(e), _) => warn!("Not the wanted NPC; err: {:?}", e),
                        // (_, Err(e)) => warn!("Not an Targeable Entity; err: {:?}", e),
                        _ => continue,
                    }
                }
                // two are sensors
                // two are errors
                _ => continue,
            }
        }

        // DEBUG: println!("Received collision event: {:?}", collision_event);
    }
}

/// Decrement the fair play Timer
/// while doing other things (don't **exclude** entity With<FairPlayTimer>)
/// remove the FairPlayTimer if the entity is in the player's team
pub fn fair_play_wait(
    mut commands: Commands,

    time: Res<Time>,
    mut npc_query: Query<
        (
            Entity,
            &mut FairPlayTimer,
            &mut Velocity,
            &Reputation,
            &Name,
        ),
        (
            With<NPC>,
            // Without<InCombat>
        ),
    >,
) {
    for (npc, mut fair_play_timer, mut _rb_vel, reputation, name) in npc_query.iter_mut() {
        fair_play_timer.timer.tick(time.delta());

        // not required to control velocity because it is managed elsewhere

        // REFACTOR: compare NPC's team with Player's Team instead of a global Player Team CST
        // query player to get his TEAM (it's the player who switch team not all npc)
        if fair_play_timer.timer.finished() || reputation.is_in_supreme_god_team() {
            info!("{:?}, {} can now aggro", npc, name);

            commands.entity(npc).remove::<FairPlayTimer>();
        }
    }
}

/// - turn off npc detection sensor
/// - turn on npc pursuit sensor
/// - insert the new target into the npc
/// match the ev's args in a query ? => security
pub fn add_pursuit_urge(
    mut commands: Commands,
    mut ev_engage_pursuit: EventReader<EngagePursuitEvent>,
    npc_query: Query<
        (Entity, &Name),
        (
            With<NPC>,
            With<DetectionBehavior>,
            Without<PursuitBehavior>,
            Without<FairPlayTimer>,
        ),
    >,
) {
    // let pursuit_sensor =
    //     commands
    //         .spawn()
    //         .insert(Collider::ball(80.))
    //         .insert(ActiveEvents::COLLISION_EVENTS)
    //         .insert(Sensor)
    //         .insert(PursuitSensor)
    //         .insert(Name::new("Pursuit Range"))
    //         .id();

    for ev in ev_engage_pursuit.iter() {
        match npc_query.get(ev.npc_entity) {
            Ok(npc) => {
                info!("add pursuit urge to {}", npc.1);

                // remove DetectionSensor
                commands.entity(ev.detection_sensor_entity).despawn();

                // remove DetectionBehavior
                commands.entity(npc.0).remove::<DetectionBehavior>();

                // turn on npc pursuit sensor
                commands
                    .entity(npc.0)
                    .insert(PursuitBehavior)
                    .with_children(|parent| {
                        parent.spawn((
                            Collider::ball(80.),
                            ActiveEvents::COLLISION_EVENTS,
                            Sensor,
                            PursuitSensor,
                            Name::new("Pursuit Range"),
                        ));
                    });

                // insert the new target into the npc
                commands
                    .entity(npc.0)
                    .insert(Target(Some(ev.target_entity)));
            }

            _ => continue,
        }
    }
}

/// remove target
/// remove PursuitBehavior
pub fn remove_pursuit_urge(
    mut commands: Commands,
    mut ev_stop_chase: EventReader<StopChaseEvent>,
    npc_query: Query<(Entity, &Children, &Name), (With<NPC>, With<PursuitBehavior>)>,
    pursuit_sensor_query: Query<Entity, (With<Collider>, With<Sensor>, With<PursuitSensor>)>,

    mut ev_detection_mode: EventWriter<DetectionModeEvent>,
) {
    for ev in ev_stop_chase.iter() {
        // remove PursuitSensor Collider (or turn it false, one day)
        match npc_query.get(ev.npc_entity) {
            Ok(npc) => {
                info!("remove pursuit urge to {}", npc.2);

                commands.entity(npc.0).remove::<PursuitBehavior>();

                commands.entity(npc.0).remove::<Target>();

                commands.entity(npc.0).insert(FairPlayTimer {
                    // create the non-repeating rest timer
                    timer: Timer::new(Duration::from_secs(EVASION_TIMER), TimerMode::Once),
                });

                // insert DetectionSensor into the Entity npc.0
                // insert DetectionBehavior
                ev_detection_mode.send(DetectionModeEvent { entity: npc.0 });

                // browse all colliders contained in within the npc
                for collider in npc.1.iter() {
                    // for all colliders matching with our query pursuit_sensor_query
                    // despawn it
                    match pursuit_sensor_query.get(*collider) {
                        // returned pursuit_sensor: Entity
                        Ok(_pursuit_sensor) => {
                            // FIXME: detach the child from their parent
                            commands.entity(*collider).despawn();
                        }

                        _ => continue,
                    }
                }
            }

            _ => continue,
        }
    }

    // send even to:
    // back to normal behavior
    // with prioritize behavior
}

/// Insert DetectionSensor
/// Insert DetectionBehavior
pub fn add_detection_aura(
    mut commands: Commands,
    mut ev_detection_mode: EventReader<DetectionModeEvent>,

    npc_query: Query<(Entity, &Name), (With<NPC>, Without<DetectionBehavior>)>,
) {
    // let detection_sensor =
    //     commands
    //         .spawn()
    //         .insert(Collider::ball(40.))
    //         .insert(ActiveEvents::COLLISION_EVENTS)
    //         .insert(Sensor)
    //         .insert(DetectionSensor)
    //         .insert(Name::new("Detection Range"))
    //         .id();

    // info!("enter add detection aura");

    for ev in ev_detection_mode.iter() {
        // DEBUG: info!("detection mode ev");

        // verify if this entity correspond with our query
        match npc_query.get(ev.entity) {
            Ok(npc) => {
                info!("add detection aura to {}", npc.1);

                commands
                    .entity(npc.0)
                    .insert(DetectionBehavior)
                    .with_children(|parent| {
                        parent.spawn((
                            Collider::ball(40.),
                            ActiveEvents::COLLISION_EVENTS,
                            Sensor,
                            DetectionSensor,
                            Name::new("Detection Range"),
                        ));
                    });
            }

            _ => continue,
        }
    }
}

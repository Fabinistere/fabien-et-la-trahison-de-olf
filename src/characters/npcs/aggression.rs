use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
// use log::info;

use crate::{
    characters::npcs::NPC,
    combat::{FairPlayTimer, Reputation},
    constants::character::npcs::movement::EVASION_TIMER,
};

use super::movement::{Chaser, NPCBehavior, TargetSeeker};

#[derive(Component)]
pub struct DetectionRangeSensor;

#[derive(Component)]
pub struct PursuitRangeSensor;

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
///   - npcs::movement::sensor_management
///     - An npc detected an enemy
///
/// Read in
///   - npc::aggression::add_pursuit_urge
///     - change the NPCBehavior to chase the target
#[derive(Event)]
pub struct EngagePursuitEvent {
    pub npc_entity: Entity,
    pub target_entity: Entity,
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
///
/// REFACTOR: Change Detection (Added)
pub fn activate_pursuit_urge(
    mut commands: Commands,
    mut ev_engage_pursuit: EventReader<EngagePursuitEvent>,
    npc_query: Query<
        (Entity, &Children, &Name),
        (With<NPC>, With<TargetSeeker>, Without<FairPlayTimer>),
    >,
    pursuit_sensor_query: Query<
        Entity,
        (
            With<PursuitRangeSensor>,
            With<Collider>,
            With<Sensor>,
            Without<DetectionRangeSensor>,
        ),
    >,
    dectection_sensor_query: Query<
        Entity,
        (
            With<DetectionRangeSensor>,
            With<Collider>,
            With<Sensor>,
            Without<PursuitRangeSensor>,
        ),
    >,
) {
    for EngagePursuitEvent {
        npc_entity,
        target_entity,
    } in ev_engage_pursuit.iter()
    {
        let (npc, children, name) = npc_query.get(*npc_entity).unwrap();
        info!("activate pursuit urge to {}", name);

        commands.entity(npc).insert(Chaser::new(*target_entity));

        // turn on npc pursuit sensor
        // turn off DetectionRangeSensor
        for collider in children {
            if pursuit_sensor_query.get(*collider).is_ok() {
                commands
                    .entity(*collider)
                    .insert(ActiveEvents::COLLISION_EVENTS);
            } else if dectection_sensor_query.get(*collider).is_ok() {
                commands.entity(*collider).remove::<ActiveEvents>();
            }
        }
    }
}

/// Change the npc behavior to `NPCBehavior::TargetSeeking`.
/// - Active Events of the collider DetectionRangeSensor
/// - Deactivate Events of the collider PursuitRangeSensor
///
/// REFACTOR: Remove Dectection
pub fn deactivate_pursuit_urge(
    mut commands: Commands,
    mut ev_stop_chase: EventReader<StopChaseEvent>,
    npc_query: Query<
        (Entity, &Children, &Name),
        (With<NPC>, With<TargetSeeker>, With<NPCBehavior>),
    >,
    pursuit_sensor_query: Query<
        Entity,
        (
            With<PursuitRangeSensor>,
            With<Collider>,
            With<Sensor>,
            Without<DetectionRangeSensor>,
        ),
    >,
    detection_sensor_query: Query<
        Entity,
        (
            With<DetectionRangeSensor>,
            With<Sensor>,
            With<Collider>,
            Without<PursuitRangeSensor>,
        ),
    >,
) {
    for StopChaseEvent { npc_entity } in ev_stop_chase.iter() {
        let (npc, children, name) = npc_query.get(*npc_entity).unwrap();
        info!("deactivate pursuit urge to {}", name);

        commands.entity(npc).insert(FairPlayTimer {
            timer: Timer::new(Duration::from_secs(EVASION_TIMER), TimerMode::Once),
        });

        commands.entity(npc).remove::<Chaser>();

        for collider in children {
            if pursuit_sensor_query.get(*collider).is_ok() {
                commands.entity(*collider).remove::<ActiveEvents>();
            } else if detection_sensor_query.get(*collider).is_ok() {
                commands
                    .entity(*collider)
                    .insert(ActiveEvents::COLLISION_EVENTS);
            }
        }
    }
}

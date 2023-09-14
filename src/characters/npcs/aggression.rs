use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
// use log::info;

use crate::{
    characters::npcs::{
        movement::{Chaser, NPCBehavior, TargetSeeker},
        NPC,
    },
    combat::FairPlayTimer,
    constants::character::npcs::movement::EVASION_TIMER,
};

/* -------------------------------------------------------------------------- */
/*                                 Components                                 */
/* -------------------------------------------------------------------------- */

#[derive(Component)]
pub struct DetectionRangeSensor;

#[derive(Component)]
pub struct PursuitRangeSensor;

/* -------------------------------------------------------------------------- */
/*                                   Events                                   */
/* -------------------------------------------------------------------------- */

/// Happens when:
///   - npc::movement::pursue
///     - target is not found/exist
///     - target is reach
///
/// Read in npc::aggression::deactivate_pursuit_urge
#[derive(Event)]
pub struct StopChaseEvent {
    pub npc_entity: Entity,
}

/// Happens when:
///   - npcs::movement::sensor_management
///     - An npc detected an enemy
///
/// Read in
///   - npc::aggression::activate_pursuit_urge
///     - change the NPCBehavior to chase the target
#[derive(Event)]
pub struct EngagePursuitEvent {
    pub npc_entity: Entity,
    pub target_entity: Entity,
}

/* -------------------------------------------------------------------------- */
/*                                   Systems                                  */
/* -------------------------------------------------------------------------- */

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

        commands
            .entity(npc)
            .insert(FairPlayTimer::new(EVASION_TIMER));

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

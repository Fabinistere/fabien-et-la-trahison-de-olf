use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

// use crate::combat::{InCombat, FairPlayTimer};

use super::{
    movement::{FollowupBehavior, PursuitBehavior},
    NPC,
};

#[derive(Component)]
pub struct IdleBehavior;

#[derive(Component)]
pub struct RestTime {
    /// track when the npc should stop rest (non-repeating timer)
    pub timer: Timer,
}

// TODO: Create a starting idleBehavior
// to avoid:
// - to give a direction in the spawn NPC
// - To give a RestTime in spwan

pub fn do_flexing(
    mut commands: Commands,
    time: Res<Time>,
    mut npc_query: Query<
        (Entity, &mut RestTime, &mut Velocity, &Name), // Option<&mut JustWalkBehavior>,
        (
            With<NPC>,
            With<IdleBehavior>,
            // Or<(JustWalkBehavior, ...)>,
            Without<FollowupBehavior>,
            Without<PursuitBehavior>,
        ),
    >,
) {
    for (npc, mut rest_timer, mut rb_vel, name) in npc_query.iter_mut() {
        rest_timer.timer.tick(time.delta());

        // prevent npcs from being launched by pushing them
        rb_vel.linvel.x = 0.;
        rb_vel.linvel.y = 0.;

        // flexing animation

        if rest_timer.timer.finished() {
            info!(target: "Stop Rest", "{:?}, {}", npc, name);

            // restart previous behavior or new one
            // TODO: feature - after the rest, npc will select a behavior
            commands.entity(npc).remove::<IdleBehavior>();
            commands.entity(npc).remove::<RestTime>();
        }
    }
}

// pub fn wait_leader(
//     mut commands: Commands,
//     mut npc_query: Query<
//         (Entity, &Name),
//         (With<IdleBehavior>, With<FollowupBehavior>)
//     >
// ) {
//     for (npc, name) in npc_query.iter_mut() {

//         // flexing animation

//         commands.entity(npc)
//                 .insert(
//                     FollowupBehavior);
//         commands.entity(npc)
//                 .remove::<IdleBehavior>();

//     }
// }

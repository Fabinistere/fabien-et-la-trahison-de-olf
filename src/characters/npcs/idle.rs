use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use super::{movement::NPCBehavior, NPC};

#[derive(Component)]
pub struct RestTime {
    /// track when the npc should stop rest (non-repeating timer)
    pub timer: Timer,
}

pub fn flexing_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut npc_query: Query<
        (Entity, &mut RestTime, &mut Velocity, &Name),
        (With<NPC>, With<NPCBehavior>),
    >,
) {
    for (npc, mut rest_timer, mut rb_vel, name) in npc_query.iter_mut() {
        rest_timer.timer.tick(time.delta());

        rb_vel.linvel.x = 0.;
        rb_vel.linvel.y = 0.;
        // info!("{:#?}", rest_timer.timer);
        if rest_timer.timer.finished() {
            // info!(target: "Stop Rest", "{:?}, {}", npc, name);

            // restart previous behavior
            commands.entity(npc).remove::<RestTime>();
        }
    }
}

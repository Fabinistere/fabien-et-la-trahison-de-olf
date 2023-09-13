use bevy::prelude::*;

use super::{movement::NPCBehavior, NPC};

#[derive(Component)]
pub struct RestTime {
    /// track when the npc should stop rest (non-repeating timer)
    pub timer: Timer,
}

pub fn flexing_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut npc_query: Query<(Entity, &mut RestTime, &Name), (With<NPC>, With<NPCBehavior>)>,
) {
    for (npc, mut rest_timer, name) in npc_query.iter_mut() {
        rest_timer.timer.tick(time.delta());
        if rest_timer.timer.finished() {
            info!(target: "Stop Rest", "{:?}, {}", npc, name);

            // restart previous behavior
            commands.entity(npc).remove::<RestTime>();
        }
    }
}

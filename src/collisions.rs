use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(collision_events);
    }
}

fn collision_events(mut collision_events: EventReader<CollisionEvent>) {
    for collision_event in collision_events.iter() {}
}

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub trait IsColliding {
    fn is_intersecting_with(&self, other: Entity, context: &Res<RapierContext>) -> bool;
}

impl IsColliding for Entity {
    fn is_intersecting_with(&self, other: Entity, context: &Res<RapierContext>) -> bool {
        for (e1, e2, is_intersecting) in context.intersections_with(*self) {
            if is_intersecting && ((e1 == *self && e2 == other) || (e1 == other && e2 == *self)) {
                return true;
            }
        }

        false
    }
}

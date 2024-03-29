pub mod landmarks;
pub mod temple;

use bevy::prelude::*;

pub struct LocationsPlugin;

impl Plugin for LocationsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((temple::TemplePlugin, landmarks::LandmarkPlugin));
    }
}

/*
pub fn spawn_child_collision_cuboid(
    parent: &mut ChildBuilder,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) {
    parent.spawn((
        RigidBodyBundle {
            body_type: RigidBodyType::Static,
            position: Vec2::new(x, y).into(),
            ..RigidBodyBundle::default()
        },
        ColliderBundle {
            shape: ColliderShape::cuboid(width, height),
            material: ColliderMaterial {
                friction: 0.,
                restitution: 0.,
                ..ColliderMaterial::default()
            },
            ..ColliderBundle::default()
        },
    ));
}
*/

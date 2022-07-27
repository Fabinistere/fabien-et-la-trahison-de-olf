pub mod temple;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum Location {
    Temple,
}

pub struct LocationsPlugin;

impl Plugin for LocationsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(temple::TemplePlugin)
            .add_state(Location::Temple);
    }
}

pub fn spawn_collision_cuboid(commands: &mut Commands, x: f32, y: f32, width: f32, height: f32) {
    commands
        .spawn()
        .insert(Collider::cuboid(width, height))
        .insert(Transform::from_xyz(x, y, 0.0))
        .insert(Friction::coefficient(0.0))
        .insert(Restitution::coefficient(0.0));
}

/*
pub fn spawn_child_collision_cuboid(
    parent: &mut ChildBuilder,
    x: f32, y: f32,
    width: f32, height: f32,
) {
    parent
        .spawn()
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static,
            position: Vec2::new(x, y).into(),
            ..RigidBodyBundle::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(width, height),
            material: ColliderMaterial {
                friction: 0.0,
                restitution: 0.0,
                ..ColliderMaterial::default()
            },
            ..ColliderBundle::default()
        });
}
*/

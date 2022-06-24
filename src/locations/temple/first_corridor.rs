use crate::{constants::locations::temple::FIRST_CORRIDOR_Z, interactions::Interactible};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn setup_first_corridor(mut commands: Commands, asset_server: Res<AssetServer>) {
    let first_corridor = asset_server.load("textures/temple/first_corridor/first_corridor.png");
    let props = asset_server.load("textures/temple/first_corridor/props.png");

    commands.spawn_bundle(SpriteBundle {
        texture: first_corridor,
        // transform: Transform::from_scale(Vec3::new(2.0, 2.0, 2.0)),
        transform: Transform::from_xyz(0.0, 0.0, FIRST_CORRIDOR_Z),
        ..SpriteBundle::default()
    });

    commands
        .spawn_bundle(SpriteBundle {
            texture: props,
            transform: Transform::from_xyz(0.0, 0.0, 2.5),
            ..SpriteBundle::default()
        })
        .with_children(|parent| {
            parent
                .spawn()
                .insert(Collider::cuboid(100.0, 105.0))
                .insert(Transform::from_xyz(-1210.0, -1430.0, 0.0));
        })
        .insert(Interactible {
            range: 100.0,
            icon_position: Vec3::new(0.0, 0.0, 0.0),
            interaction_id: 0,
        });
}

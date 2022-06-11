use crate::constants::locations::temple::FIRST_CORRIDOR_Z;
use bevy::prelude::*;

pub fn setup_first_corridor(mut commands: Commands, asset_server: Res<AssetServer>) {
    let first_corridor = asset_server.load("textures/temple/first_corridor/first_corridor.png");
    let props = asset_server.load("textures/temple/first_corridor/props.png");

    commands.spawn_bundle(SpriteBundle {
        texture: first_corridor,
        // transform: Transform::from_scale(Vec3::new(2.0, 2.0, 2.0)),
        transform: Transform::from_xyz(0.0, 0.0, FIRST_CORRIDOR_Z),
        ..SpriteBundle::default()
    });

    commands.spawn_bundle(SpriteBundle {
        texture: props,
        ..SpriteBundle::default()
    });
}

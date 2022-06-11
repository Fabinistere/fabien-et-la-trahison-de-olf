use crate::{constants::locations::temple::SECOND_CORRIDOR_Z, locations::spawn_collision_cuboid};
use bevy::prelude::*;

pub fn setup_second_corridor(mut commands: Commands, asset_server: Res<AssetServer>) {
    let second_corridor = asset_server.load("textures/temple/second_corridor/second_corridor.png");

    commands.spawn_bundle(SpriteBundle {
        texture: second_corridor,
        transform: Transform::from_xyz(0.0, 0.0, SECOND_CORRIDOR_Z),
        ..SpriteBundle::default()
    });

    // Left side of bottom wall
    spawn_collision_cuboid(&mut commands, -380.0, -1185.0, 930.0, 140.0);
    // Right side of bottom wall
    spawn_collision_cuboid(&mut commands, 760.0, -1185.0, 90.0, 140.0);
}

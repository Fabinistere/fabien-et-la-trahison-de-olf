use crate::{
    constants::{interactions::INTERACT_BUTTON_Z, locations::temple::second_corridor::*},
    interactions::Interactible,
    locations::spawn_collision_cuboid,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Door {
    open: bool,
}

#[derive(Component)]
pub struct DoorInteract {
    timer: Timer,
    opening: bool,
}

pub fn setup_second_corridor(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let second_corridor = asset_server.load("textures/temple/second_corridor/second_corridor.png");
    let door_spritesheet =
        asset_server.load("textures/temple/second_corridor/door_spritesheet.png");
    let door_texture_atlas =
        TextureAtlas::from_grid(door_spritesheet, Vec2::new(200.0, 300.0), 1, 8);

    // Left side of bottom wall
    spawn_collision_cuboid(&mut commands, -380.0, -1185.0, 930.0, 140.0);
    // Right side of bottom wall
    spawn_collision_cuboid(&mut commands, 760.0, -1185.0, 90.0, 140.0);

    commands.spawn_bundle(SpriteBundle {
        texture: second_corridor,
        transform: Transform::from_xyz(0.0, 0.0, SECOND_CORRIDOR_Z),
        ..SpriteBundle::default()
    });

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlases.add(door_texture_atlas),
            transform: Transform::from_translation(DOOR_POSITION.into()),
            ..SpriteSheetBundle::default()
        })
        .insert(Door { open: false })
        .insert(Interactible {
            icon_transform: Transform::from_xyz(-80.0, 10.0, INTERACT_BUTTON_Z),
            interaction_id: DOOR_INTERACTION_ID,
        })
        .with_children(|parent| {
            parent
                .spawn()
                .insert(Collider::cuboid(70.0, 35.0))
                .insert(Transform::from_xyz(
                    DOOR_POSITION.0,
                    DOOR_POSITION.1 - 60.0,
                    0.0,
                ))
                .insert(Sensor(true));

            parent
                .spawn()
                .insert(Collider::cuboid(30.0, 50.0))
                .insert(Transform::from_xyz(
                    DOOR_POSITION.0 - 30.0,
                    DOOR_POSITION.1,
                    0.0,
                ));

            parent
                .spawn()
                .insert(Collider::cuboid(30.0, 50.0))
                .insert(Transform::from_xyz(
                    DOOR_POSITION.0 + 30.0,
                    DOOR_POSITION.1,
                    0.0,
                ));
        });
}

use super::Temple;
use crate::{
    constants::locations::temple::{
        main_room::*,
        second_corridor::{
            DOOR_Z, DOOR_Z_IN_MAIN_ROOM, SECOND_CORRIDOR_Z, SECOND_CORRIDOR_Z_IN_MAIN_ROOM,
        },
    },
    constants::player::PLAYER_HITBOX_Y_OFFSET,
    locations::temple::second_corridor::{Door, SecondCorridor},
    player::Player,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Pillar;
#[derive(Component)]
pub struct Throne;
#[derive(Component)]
pub struct EnterMainRoomSensor;

pub struct EnterMainRoomEvent;

pub fn enter_main_room(
    mut enter_main_room_events: EventReader<EnterMainRoomEvent>,
    mut second_corridor_query: Query<&mut Transform, (With<SecondCorridor>, Without<Player>)>,
    mut door_query: Query<&mut Transform, (With<Door>, Without<Player>, Without<SecondCorridor>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    for EnterMainRoomEvent in enter_main_room_events.iter() {
        let player_transform = player_query.single();
        let mut corridor_transform = second_corridor_query.single_mut();
        let mut door_transform = door_query.single_mut();
        dbg!(
            &corridor_transform,
            &door_transform,
            &player_transform,
            MAIN_ROOM_ENTER_Y
        );

        if player_transform.translation.y >= MAIN_ROOM_ENTER_Y {
            corridor_transform.translation.z = SECOND_CORRIDOR_Z;
            door_transform.translation.z = DOOR_Z;
        } else {
            corridor_transform.translation.z = SECOND_CORRIDOR_Z_IN_MAIN_ROOM;
            door_transform.translation.z = DOOR_Z_IN_MAIN_ROOM;
        }
    }
}

pub fn pillars_position(
    player_query: Query<&GlobalTransform, With<Player>>,
    mut pillars_query: Query<&mut Transform, With<Pillar>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for mut pillar_transform in pillars_query.iter_mut() {
            if player_transform.translation.y + 60.0 > pillar_transform.translation.y {
                pillar_transform.translation.z = PILLARS_Z_FRONT;
            } else {
                pillar_transform.translation.z = PILLARS_Z_BACK;
            }
        }
    }
}

pub fn throne_position(
    player_query: Query<&GlobalTransform, With<Player>>,
    mut throne_query: Query<&mut Transform, With<Throne>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for mut throne_transform in throne_query.iter_mut() {
            if player_transform.translation.y > throne_transform.translation.y {
                throne_transform.translation.z = THRONE_Z_FRONT;
            } else {
                throne_transform.translation.z = THRONE_Z_BACK;
            }
        }
    }
}

// Spawns all entity related to the main room
pub fn setup_main_room(mut commands: Commands, asset_server: Res<AssetServer>) {
    let main_room = asset_server.load("textures/temple/main_room.png");
    let pillar = asset_server.load("textures/temple/pillar.png");
    let throne = asset_server.load("textures/temple/throne.png");

    commands
        .spawn_bundle(SpriteBundle {
            texture: main_room,
            transform: Transform::from_xyz(0.0, 0.0, MAIN_ROOM_Z),
            ..SpriteBundle::default()
        })
        .insert(Temple);

    commands
        .spawn()
        .insert(Collider::segment(
            Vect::new(-320.0, MAIN_ROOM_ENTER_Y),
            Vect::new(-140.0, MAIN_ROOM_ENTER_Y),
        ))
        .insert(Transform::default())
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(EnterMainRoomSensor)
        .insert(Sensor(true));

    commands
        .spawn_bundle(SpriteBundle {
            texture: throne,
            transform: Transform::from_translation(THRONE_POSITION.into()),
            ..SpriteBundle::default()
        })
        .insert(Throne);

    for pos in PILLAR_POSITIONS {
        commands
            .spawn_bundle(SpriteBundle {
                texture: pillar.clone(),
                transform: Transform::from_translation(pos.into()),
                ..SpriteBundle::default()
            })
            .insert(Pillar)
            .with_children(|parent| {
                parent
                    .spawn()
                    .insert(Collider::cuboid(60.0, 20.0))
                    .insert(Transform::from_xyz(pos.0, pos.1 - 110.0, 0.0));
            });
    }
}

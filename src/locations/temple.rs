use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::{
    player::Player,
    constants::locations::temple::*,
};
use super::{ Location, spawn_collision_cuboid };

pub struct TemplePlugin;

impl Plugin for TemplePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system_set(
                SystemSet::on_enter(Location::Temple)
                    .with_system(setup_temple.system())
                    .with_system(spawn_hitboxes.system())
            )
            .add_system(pillars_position.system())
            .add_system(curtains_animation.system())
            .add_system(secret_room_detection.system())
            .insert_resource(SecretRoomHidden(true));
    }
}

struct Temple;
struct Pillar;
struct SecretRoomHidden(bool);

fn pillars_position(
    player_query: Query<&GlobalTransform, With<Player>>,
    mut pillars_query: Query<(&mut Transform, &RigidBodyPosition), With<Pillar>>,
) {
    if let Ok(player_transform) = player_query.single() {
        for (mut pillar_transform, rb_pos) in pillars_query.iter_mut() {
            if player_transform.translation.y - 60.0 > rb_pos.position.translation.y {
                pillar_transform.translation.z = PILLARS_Z_FRONT;
            } else {
                pillar_transform.translation.z = PILLARS_Z_BACK;
            }
        }
    }
}

fn secret_room_detection(
    query: Query<&Transform, With<Player>>,
    mut secret_room_hidden: ResMut<SecretRoomHidden>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for transform in query.iter() {
        if transform.translation.y >= 610.0 {
            info!("hello");
            secret_room_hidden.0 = false;
        } else {
            info!("{}", transform.translation.y);
            secret_room_hidden.0 = true;
        } 
    }
}

fn spawn_secret_room(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let secret_room = asset_server.load("textures/temple/secret_room.png");

    commands.spawn_bundle(SpriteBundle {
        material: materials.add(secret_room.into()),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, BACKGROUND_Z)),
        ..SpriteBundle::default()
    }).insert(Temple);
}

fn despawn_secret_room(mut commands: &Commands) {

}

fn curtains_animation(

) {

}

fn setup_temple(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let background = asset_server.load("textures/temple/background.png");
    let main_room = asset_server.load("textures/temple/main_room.png");
    let pillar = asset_server.load("textures/temple/pillar_int.png");
    let stones = asset_server.load("textures/temple/stones.png");

    commands.spawn_bundle(SpriteBundle {
        material: materials.add(background.into()),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, BACKGROUND_Z)),
        ..SpriteBundle::default()
    }).insert(Temple);

    commands.spawn_bundle(SpriteBundle {
        material: materials.add(main_room.into()),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, TEMPLE_Z)),
        ..SpriteBundle::default()
    }).insert(Temple);

    commands.spawn_bundle(SpriteBundle {
        material: materials.add(stones.into()),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, STONES_Z)),
        ..SpriteBundle::default()
    }).insert(Temple);

    for pos in PILLAR_POSITIONS {
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(pillar.clone().into()),
                transform: Transform::from_translation(pos.into()),
                ..SpriteBundle::default()
            })
            .insert_bundle(RigidBodyBundle {
                body_type: RigidBodyType::Static,
                mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
                position: Vec2::new(pos.0, pos.1 - 110.0).into(),
                ..RigidBodyBundle::default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(ColliderBundle {
                    shape: ColliderShape::cuboid(60.0, 20.0),
                    position: Vec2::new(0.0, 0.0).into(),
                    material: ColliderMaterial {
                        friction: 0.0,
                        restitution: 0.0,
                        ..ColliderMaterial::default()
                    },
                    ..ColliderBundle::default()
                });
            })
            .insert(Pillar);
    }
}

fn spawn_hitboxes(mut commands: Commands) {
    // Left wall
    spawn_collision_cuboid(&mut commands, -1080.0, -240.0, 10.0, 810.0);
    // Right wall
    spawn_collision_cuboid(&mut commands, 1080.0, -240.0, 10.0, 810.0);
    // Left side of top wall
    spawn_collision_cuboid(&mut commands, -655.0, 580.0, 415.0, 10.0);
    // Right side of top wall
    spawn_collision_cuboid(&mut commands, 455.0, 580.0, 615.0, 10.0);
    // Left of hidden door
    spawn_collision_cuboid(&mut commands, -150.0, 610.0, 10.0, 20.0);
    // Right of hidden foor
    spawn_collision_cuboid(&mut commands, -250.0, 610.0, 10.0, 20.0);
}

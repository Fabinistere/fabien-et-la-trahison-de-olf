use bevy::{ prelude::*, utils::Duration };
use bevy_rapier2d::prelude::*;
use crate::{
    player::Player,
    constants::locations::temple::*,
    animations::*,
};
use super::{ Location, spawn_collision_cuboid };

pub struct TemplePlugin;

impl Plugin for TemplePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_state(PlayerLocation::Temple)
            .add_system_set(
                SystemSet::on_enter(Location::Temple)
                    .with_system(setup_temple.system())
                    .with_system(spawn_hitboxes.system())
            )
            .add_system_set(
                SystemSet::on_enter(PlayerLocation::SecretRoom)
                    .with_system(remove_secret_room_cover.system())
            )
            .add_system_set(
                SystemSet::on_exit(PlayerLocation::SecretRoom)
                    .with_system(add_secret_room_cover.system())
            )
            .add_system(pillars_position.system())
            .add_system(curtains_animation.system())
            .add_system(secret_room_enter.system());
    }
}

// Components
struct Temple;
struct Pillar;
struct SecretRoom;
struct SecretRoomCover;

// States
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum PlayerLocation {
    Temple,
    SecretRoom,
}

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

fn secret_room_enter(
    player_query: Query<&GlobalTransform, With<Player>>,
    mut player_location: ResMut<State<PlayerLocation>>,
) {
    if let Ok(transform) = player_query.single() {
        if transform.translation.y >= SECRET_ROOM_TRIGGER_Y
            && player_location.current() == &PlayerLocation::Temple
        {
            player_location.set(PlayerLocation::SecretRoom).unwrap();
        } else if transform.translation.y < SECRET_ROOM_TRIGGER_Y
            && player_location.current() == &PlayerLocation::SecretRoom
        {
            player_location.set(PlayerLocation::Temple).unwrap();
        }
    }
}

fn secret_room_cover_fade() {

}

fn remove_secret_room_cover(
    mut commands: Commands,
    mut temple_query: Query<&mut Transform, With<Temple>>,
    mut secret_room_cover_query: Query<(Entity, Option<&mut Fade>), With<SecretRoomCover>>,
) {
    if let Ok((cover_entity, fade_opt)) = secret_room_cover_query.single_mut() {
        if let Some(mut fade) = fade_opt {
            fade.invert();
        } else {
            commands.entity(cover_entity).insert(Fade {
                current_alpha: 1.0,
                fade_type: FadeType::FadeIn,
                total_duration: Duration::from_secs(1),
                animation_fn: ease_in_sine,
                ..Fade::default()
            });
        }
    }

    if let Ok(mut temple_transform) = temple_query.single_mut() {
        temple_transform.translation.z = TEMPLE_Z_WHEN_IN_SECRET_ROOM;
    }
}

fn add_secret_room_cover(
    mut commands: Commands,
    mut temple_query: Query<&mut Transform, With<Temple>>,
    mut secret_room_cover_query: Query<(Entity, Option<&mut Fade>), With<SecretRoomCover>>,
) {
    if let Ok((cover_entity, fade_opt)) = secret_room_cover_query.single_mut() {
        if let Some(mut fade) = fade_opt {
            fade.invert();
        } else {
            commands.entity(cover_entity).insert(Fade {
                current_alpha: 0.0,
                fade_type: FadeType::FadeOut,
                total_duration: Duration::from_secs(1),
                animation_fn: ease_out_sine,
                ..Fade::default()
            });
        }
    }

    if let Ok(mut temple_transform) = temple_query.single_mut() {
        temple_transform.translation.z = TEMPLE_Z;
    }
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
    let secret_room = asset_server.load("textures/temple/secret_room.png");
    let pillar = asset_server.load("textures/temple/pillar_int.png");
    let stones = asset_server.load("textures/temple/stones.png");

    commands.spawn_bundle(SpriteBundle {
        material: materials.add(background.into()),
        transform: Transform::from_xyz(0.0, 0.0, BACKGROUND_Z),
        ..SpriteBundle::default()
    });

    commands.spawn_bundle(SpriteBundle {
        material: materials.add(main_room.into()),
        transform: Transform::from_xyz(0.0, 0.0, TEMPLE_Z),
        ..SpriteBundle::default()
    }).insert(Temple);

    commands.spawn_bundle(SpriteBundle {
        material: materials.add(stones.into()),
        transform: Transform::from_xyz(0.0, 0.0, STONES_Z),
        ..SpriteBundle::default()
    });

    commands.spawn_bundle(SpriteBundle {
        material: materials.add(secret_room.into()),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, SECRET_ROOM_Z)),
        ..SpriteBundle::default()
    }).insert(SecretRoom);

    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::Rgba {
            red: 58.0 / 256.0,
            green: 36.0 / 246.0,
            blue: 48.0 / 256.0,
            alpha: 1.0,
        }.into()),
        transform: Transform::from_xyz(0.0, 925.0, SECRET_ROOM_COVER_Z),
        sprite: Sprite::new(Vec2::new(2420.0, 670.0)),
        ..SpriteBundle::default()
    }).insert(SecretRoomCover);

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
    spawn_collision_cuboid(&mut commands, -1080.0, -40.0, 10.0, 1010.0);
    // Right wall
    spawn_collision_cuboid(&mut commands, 1080.0, -40.0, 10.0, 1010.0);
    // Left side of top wall
    spawn_collision_cuboid(&mut commands, -655.0, 600.0, 415.0, 30.0);
    // Right side of top wall
    spawn_collision_cuboid(&mut commands, 455.0, 600.0, 615.0, 30.0);
    // Bottom wall
    spawn_collision_cuboid(&mut commands, 0.0, -1060.0, 1070.0, 10.0);
    // Top wall of secret room
    spawn_collision_cuboid(&mut commands, 0.0, 980.0, 1070.0, 10.0);
}

mod curtains;
mod secret_room;

use super::{spawn_collision_cuboid, Location};
use crate::{
    constants::{locations::temple::*, BACKGROUND_COLOR},
    player::Player,
    GameState,
};
use bevy::{ecs::schedule::ShouldRun, prelude::*};
use bevy_rapier2d::prelude::*;
use curtains::Curtain;
use secret_room::{SecretRoom, SecretRoomCover, SecretRoomSensor};

pub struct TemplePlugin;

impl Plugin for TemplePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(PlayerLocation::Temple)
            .add_state(curtains::PlayerCurtainsPosition::Below)
            .add_system_set(
                SystemSet::on_enter(Location::Temple)
                    .with_system(setup_temple)
                    .with_system(spawn_hitboxes),
            )
            .add_system_set(
                SystemSet::on_enter(PlayerLocation::SecretRoom)
                    .with_system(secret_room::remove_secret_room_cover),
            )
            .add_system_set(
                SystemSet::on_exit(PlayerLocation::SecretRoom)
                    .with_system(secret_room::add_secret_room_cover),
            )
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::new()
                    .with_run_criteria(run_if_in_temple)
                    .with_system(pillars_position)
                    .with_system(curtains::curtains_animation)
                    .with_system(curtains::curtains_z_position)
                    .with_system(secret_room::secret_room_enter)
                    .with_system(throne_position)
                    .with_system(olf_cat_animation),
            );
    }
}

#[derive(Component)]
pub struct Temple;
#[derive(Component)]
struct Pillar;
#[derive(Component)]
struct Throne;
#[derive(Component, Deref, DerefMut)]
pub struct ZPosition(f32);
#[derive(Component, Deref, DerefMut)]
struct OlfCatTimer(Timer);

// States
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum PlayerLocation {
    Temple,
    SecretRoom,
}

fn run_if_in_temple(
    location: Res<State<Location>>,
    game_state: Res<State<GameState>>,
) -> ShouldRun {
    if location.current() == &Location::Temple && game_state.current() == &GameState::Playing {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn pillars_position(
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

fn throne_position(
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

// Animation of smol black cat
fn olf_cat_animation(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut OlfCatTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index as usize + 1) % texture_atlas.textures.len();
        }
    }
}

// Spawns all entity related to the temple
fn setup_temple(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let background = asset_server.load("textures/temple/background.png");
    let main_room = asset_server.load("textures/temple/main_room.png");
    let secret_room = asset_server.load("textures/temple/secret_room.png");
    let pillar = asset_server.load("textures/temple/pillar.png");
    let throne = asset_server.load("textures/temple/throne.png");
    let curtains_spritesheet = asset_server.load("textures/temple/curtains_sprite_sheet.png");
    let ground = asset_server.load("textures/temple/ground.png");
    let olf_cat_spritesheet = asset_server.load("textures/temple/olf_cat_spritesheet.png");
    let left_curtains_texture_atlas =
        TextureAtlas::from_grid(curtains_spritesheet.clone(), Vec2::new(200.0, 360.0), 1, 10);
    let right_curtains_texture_atlas =
        TextureAtlas::from_grid(curtains_spritesheet, Vec2::new(200.0, 360.0), 1, 10);
    let olf_cat_texture_atlas =
        TextureAtlas::from_grid(olf_cat_spritesheet, Vec2::new(100.0, 110.0), 2, 1);

    // Sensors colliders
    // Secret door sensor
    commands
        .spawn()
        .insert(Collider::segment(
            Vect::new(-235.0, SECRET_ROOM_TRIGGER_Y),
            Vect::new(-165.0, SECRET_ROOM_TRIGGER_Y),
        ))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(SecretRoomSensor)
        .insert(Sensor(true));

    // All the temple sprites
    commands.spawn_bundle(SpriteBundle {
        texture: background,
        transform: Transform::from_xyz(0.0, 0.0, BACKGROUND_Z),
        ..SpriteBundle::default()
    });

    commands
        .spawn_bundle(SpriteBundle {
            texture: main_room,
            transform: Transform::from_xyz(0.0, 0.0, TEMPLE_Z),
            ..SpriteBundle::default()
        })
        .insert(Temple);

    commands.spawn_bundle(SpriteBundle {
        texture: ground,
        transform: Transform::from_xyz(0.0, 0.0, GROUND_Z),
        ..SpriteBundle::default()
    });

    commands
        .spawn_bundle(SpriteBundle {
            texture: secret_room,
            transform: Transform::from_xyz(0.0, 0.0, SECRET_ROOM_Z),
            ..SpriteBundle::default()
        })
        .insert(SecretRoom);

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(0.0, 925.0, SECRET_ROOM_COVER_Z),
            sprite: Sprite {
                custom_size: Some(Vec2::new(2420.0, 670.0)),
                color: BACKGROUND_COLOR,
                ..Sprite::default()
            },
            ..SpriteBundle::default()
        })
        .insert(SecretRoomCover);

    commands
        .spawn_bundle(SpriteBundle {
            texture: throne,
            transform: Transform::from_xyz(0.0, 450.0, THRONE_Z_BACK),
            ..SpriteBundle::default()
        })
        .insert(Throne);

    // Left curtain, with a sensor collider to detect when the player passes through it
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlases.add(left_curtains_texture_atlas),
            transform: Transform::from_xyz(-200.0, 630.0, CURTAINS_Z_BACK),
            ..SpriteSheetBundle::default()
        })
        .insert(Collider::segment(
            Vect::new(-30.0, CURTAINS_SENSOR_Y_OFFSET),
            Vect::new(30.0, CURTAINS_SENSOR_Y_OFFSET),
        ))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(ActiveCollisionTypes::STATIC_STATIC)
        .insert(Sensor(true))
        .insert(Curtain);

    // Right curtain
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlases.add(right_curtains_texture_atlas),
            transform: Transform::from_xyz(200.0, 630.0, CURTAINS_Z_BACK),
            ..SpriteSheetBundle::default()
        })
        .insert(Collider::segment(
            Vect::new(-30.0, CURTAINS_SENSOR_Y_OFFSET),
            Vect::new(30.0, CURTAINS_SENSOR_Y_OFFSET),
        ))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(ActiveCollisionTypes::STATIC_STATIC)
        .insert(Sensor(true))
        .insert(Curtain);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlases.add(olf_cat_texture_atlas),
            transform: Transform {
                translation: Vec3::new(-200.0, 960.0, OLF_CAT_Z),
                scale: Vec3::new(OLF_CAT_SCALE, OLF_CAT_SCALE, 1.0),
                ..Transform::default()
            },
            ..SpriteSheetBundle::default()
        })
        .insert(OlfCatTimer(Timer::from_seconds(
            OLF_CAT_ANIMATION_DELTA,
            true,
        )));

    for pos in PILLAR_POSITIONS {
        commands
            .spawn_bundle(SpriteBundle {
                texture: pillar.clone(),
                transform: Transform::from_translation(pos.into()),
                ..SpriteBundle::default()
            })
            .with_children(|parent| {
                parent
                    .spawn()
                    .insert(Collider::cuboid(60.0, 20.0))
                    .insert(Transform::from_xyz(pos.0, pos.1 - 110.0, 0.0));
            })
            .insert(Pillar);
    }
}

fn spawn_hitboxes(mut commands: Commands) {
    // Left wall
    spawn_collision_cuboid(&mut commands, -1080.0, -40.0, 10.0, 1080.0);
    // Right wall
    spawn_collision_cuboid(&mut commands, 1080.0, -40.0, 10.0, 1080.0);
    // Left side of top wall
    spawn_collision_cuboid(&mut commands, -655.0, 530.0, 415.0, 30.0);
    // Right side of top wall
    spawn_collision_cuboid(&mut commands, 455.0, 530.0, 615.0, 30.0);
    // Bottom wall
    spawn_collision_cuboid(&mut commands, 0.0, -1130.0, 1070.0, 10.0);
    // Top wall of secret room
    spawn_collision_cuboid(&mut commands, 0.0, 1050.0, 1070.0, 10.0);
    // Middle wall of secret room
    spawn_collision_cuboid(&mut commands, 160.0, 850.0, 140.0, 190.0);
    // Throne seat
    spawn_collision_cuboid(&mut commands, 0.0, 410.0, 70.0, 40.0);
    // Throne front of seat
    spawn_collision_cuboid(&mut commands, 0.0, 360.0, 50.0, 10.0);
    // Throne front of front of seat
    spawn_collision_cuboid(&mut commands, 0.0, 340.0, 30.0, 10.0);
    // Throne bump left 1
    spawn_collision_cuboid(&mut commands, -330.0, 440.0, 1.0, 60.0);
    // Throne bump right 1
    spawn_collision_cuboid(&mut commands, 330.0, 440.0, 1.0, 60.0);
    // Throne bump left 2
    spawn_collision_cuboid(&mut commands, -310.0, 350.0, 1.0, 30.0);
    // Throne bump right 2
    spawn_collision_cuboid(&mut commands, 310.0, 350.0, 1.0, 30.0);
    // Throne bump left 3
    spawn_collision_cuboid(&mut commands, -290.0, 290.0, 1.0, 30.0);
    // Throne bump right 3
    spawn_collision_cuboid(&mut commands, 290.0, 290.0, 1.0, 30.0);
    // Throne bump left 4
    spawn_collision_cuboid(&mut commands, -230.0, 215.0, 1.0, 45.0);
    // Throne bump right 4
    spawn_collision_cuboid(&mut commands, 230.0, 215.0, 1.0, 45.0);
}

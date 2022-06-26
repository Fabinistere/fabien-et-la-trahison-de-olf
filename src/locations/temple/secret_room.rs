use super::{PlayerLocation, Temple};
use crate::{
    animations::{
        functions::{ease_in_sine, ease_out_sine},
        Fade, FadeType,
    },
    constants::{
        locations::temple::{secret_room::*, TEMPLE_Z, TEMPLE_Z_WHEN_IN_SECRET_ROOM},
        player::{PLAYER_HITBOX_WIDTH, PLAYER_HITBOX_Y_OFFSET},
        BACKGROUND_COLOR,
    },
    locations::spawn_collision_cuboid,
    player::Player,
};
use bevy::{prelude::*, utils::Duration};
use bevy_rapier2d::prelude::*;

pub struct SecretRoomTriggerEvent {
    pub started: bool,
}

#[derive(Component)]
pub struct SecretRoomSensor;
#[derive(Component)]
pub struct SecretRoom;
#[derive(Component)]
pub struct SecretRoomCover;
#[derive(Component, Deref, DerefMut)]
pub struct OlfCatTimer(Timer);

pub fn setup_secret_room(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let secret_room = asset_server.load("textures/temple/secret_room/secret_room.png");
    let olf_cat_spritesheet =
        asset_server.load("textures/temple/secret_room/olf_cat_spritesheet.png");
    let olf_cat_texture_atlas =
        TextureAtlas::from_grid(olf_cat_spritesheet, Vec2::new(100.0, 110.0), 2, 1);

    commands
        .spawn_bundle(SpriteBundle {
            texture: secret_room,
            transform: Transform::from_xyz(0.0, 0.0, SECRET_ROOM_Z),
            ..SpriteBundle::default()
        })
        .insert(SecretRoom)
        .with_children(|parent| {
            parent
                .spawn()
                .insert(Collider::segment(
                    Vect::new(-480.0, SECRET_ROOM_TRIGGER_Y),
                    Vect::new(-400.0, SECRET_ROOM_TRIGGER_Y),
                ))
                .insert(Transform::default())
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(SecretRoomSensor)
                .insert(Sensor(true));
        });

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_translation(SECRET_ROOM_COVER_POSITION.into()),
            sprite: Sprite {
                custom_size: Some(SECRET_ROOM_COVER_SIZE.into()),
                color: BACKGROUND_COLOR,
                ..Sprite::default()
            },
            ..SpriteBundle::default()
        })
        .insert(SecretRoomCover);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlases.add(olf_cat_texture_atlas),
            transform: Transform {
                translation: OLF_CAT_POSITION.into(),
                scale: Vec3::new(OLF_CAT_SCALE, OLF_CAT_SCALE, 1.0),
                ..Transform::default()
            },
            ..SpriteSheetBundle::default()
        })
        .insert(OlfCatTimer(Timer::from_seconds(
            OLF_CAT_ANIMATION_DELTA,
            true,
        )))
        .with_children(|parent| {
            parent
                .spawn()
                .insert(Collider::cuboid(25.0, 25.0))
                .insert(Transform::from_translation(OLF_CAT_HITBOX_POSITION.into()));
        });

    // Top wall of secret room
    spawn_collision_cuboid(&mut commands, -230.0, 1485.0, 1080.0, 10.0);
    // Middle wall of secret room
    spawn_collision_cuboid(&mut commands, -80.0, 1285.0, 140.0, 190.0);
}

pub fn secret_room_trigger(
    // player_sensor_query: Query<Entity, With<PlayerSensorCollider>>,
    player_query: Query<&Transform, With<Player>>,
    sensor_query: Query<Entity, With<SecretRoomSensor>>,
    mut secret_room_trigger_events: EventReader<SecretRoomTriggerEvent>,
    mut player_location: ResMut<State<PlayerLocation>>,
) {
    for SecretRoomTriggerEvent { started } in secret_room_trigger_events.iter() {
        let transform = player_query.single();

        if *started {
            // When the player goes through the sensor collider, change its location
            // to the secret room or the temple
            if player_location.current() == &PlayerLocation::Temple {
                player_location.set(PlayerLocation::SecretRoom).unwrap();
            } else {
                player_location.set(PlayerLocation::Temple).unwrap();
            }
        } else {
            // If the player changes direction while the sensor is still in its collider,
            // check the top of its hitbox is in the temple or the secret room
            if transform.translation.y + PLAYER_HITBOX_Y_OFFSET + PLAYER_HITBOX_WIDTH / 2.0
                > SECRET_ROOM_TRIGGER_Y
                && player_location.current() == &PlayerLocation::Temple
            {
                player_location.set(PlayerLocation::SecretRoom).unwrap();
            } else if transform.translation.y + PLAYER_HITBOX_Y_OFFSET + PLAYER_HITBOX_WIDTH / 2.0
                < SECRET_ROOM_TRIGGER_Y
                && player_location.current() == &PlayerLocation::SecretRoom
            {
                player_location.set(PlayerLocation::Temple).unwrap();
            }
        }
    }
}

pub fn remove_secret_room_cover(
    mut commands: Commands,
    mut temple_query: Query<&mut Transform, With<Temple>>,
    mut secret_room_cover_query: Query<(Entity, Option<&mut Fade>), With<SecretRoomCover>>,
) {
    if let Ok((cover_entity, fade_opt)) = secret_room_cover_query.get_single_mut() {
        if let Some(mut fade) = fade_opt {
            fade.invert();
        } else {
            commands.entity(cover_entity).insert(Fade::new(
                FadeType::FadeIn,
                Duration::from_secs(1),
                ease_in_sine,
            ));
        }
    }

    if let Ok(mut temple_transform) = temple_query.get_single_mut() {
        temple_transform.translation.z = TEMPLE_Z_WHEN_IN_SECRET_ROOM;
    }
}

pub fn add_secret_room_cover(
    mut commands: Commands,
    mut temple_query: Query<&mut Transform, With<Temple>>,
    mut secret_room_cover_query: Query<(Entity, Option<&mut Fade>), With<SecretRoomCover>>,
) {
    if let Ok((cover_entity, fade_opt)) = secret_room_cover_query.get_single_mut() {
        if let Some(mut fade) = fade_opt {
            fade.invert();
        } else {
            commands.entity(cover_entity).insert(Fade::new(
                FadeType::FadeOut,
                Duration::from_secs(1),
                ease_out_sine,
            ));
        }
    }

    if let Ok(mut temple_transform) = temple_query.get_single_mut() {
        temple_transform.translation.z = TEMPLE_Z;
    }
}

// Animation of smol black cat
pub fn olf_cat_animation(
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

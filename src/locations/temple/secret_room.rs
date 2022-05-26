use super::{PlayerLocation, Temple};
use crate::{
    animations::fade::*,
    constants::{
        locations::temple::*,
        player::{PLAYER_HITBOX_WIDTH, PLAYER_HITBOX_Y_OFFSET},
    },
    player::Player,
};
use bevy::{prelude::*, utils::Duration};
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct SecretRoomSensor;
#[derive(Component)]
pub struct SecretRoom;
#[derive(Component)]
pub struct SecretRoomCover;

pub fn secret_room_enter(
    // player_sensor_query: Query<Entity, With<PlayerSensorCollider>>,
    player_query: Query<&Transform, With<Player>>,
    sensor_query: Query<Entity, With<SecretRoomSensor>>,
    mut collision_events: EventReader<CollisionEvent>,
    mut player_location: ResMut<State<PlayerLocation>>,
) {
    for collision_event in collision_events.iter() {
        if let Ok(transform) = player_query.get_single() {
            let sensor_e = sensor_query.single();

            match collision_event {
                // When the player goes through the sensor collider, change its location
                // to the secret room or the temple
                CollisionEvent::Started(e1, e2, _) if *e1 == sensor_e || *e2 == sensor_e => {
                    if player_location.current() == &PlayerLocation::Temple {
                        player_location.set(PlayerLocation::SecretRoom).unwrap();
                    } else {
                        player_location.set(PlayerLocation::Temple).unwrap();
                    }
                }
                CollisionEvent::Stopped(e1, e2, _) if *e1 == sensor_e || *e2 == sensor_e => {
                    // If the player changes direction while the sensor is still in its collider,
                    // check the top of its hitbox is in the temple or the secret room
                    if transform.translation.y + PLAYER_HITBOX_Y_OFFSET + PLAYER_HITBOX_WIDTH / 2.0
                        > SECRET_ROOM_TRIGGER_Y
                        && player_location.current() == &PlayerLocation::Temple
                    {
                        player_location.set(PlayerLocation::SecretRoom).unwrap();
                    } else if transform.translation.y
                        + PLAYER_HITBOX_Y_OFFSET
                        + PLAYER_HITBOX_WIDTH / 2.0
                        < SECRET_ROOM_TRIGGER_Y
                        && player_location.current() == &PlayerLocation::SecretRoom
                    {
                        player_location.set(PlayerLocation::Temple).unwrap();
                    }
                }
                _ => {}
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
            commands.entity(cover_entity).insert(Fade {
                current_alpha: 1.0,
                fade_type: FadeType::FadeIn,
                total_duration: Duration::from_secs(1),
                animation_fn: ease_in_sine,
                ..Fade::default()
            });
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
            commands.entity(cover_entity).insert(Fade {
                current_alpha: 0.0,
                fade_type: FadeType::FadeOut,
                total_duration: Duration::from_secs(1),
                animation_fn: ease_out_sine,
                ..Fade::default()
            });
        }
    }

    if let Ok(mut temple_transform) = temple_query.get_single_mut() {
        temple_transform.translation.z = TEMPLE_Z;
    }
}

use bevy::prelude::*;

use crate::{characters::player::Player, constants::character::player::CAMERA_INTERPOLATION};

#[derive(Component)]
struct PlayerCamera;

pub fn player_camera_follow(
    mut queries: ParamSet<(
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<PlayerCamera>>,
    )>,
) {
    if let Ok(t) = queries.p0().get_single() {
        let player_transform = *t;

        if let Ok(mut camera_transform) = queries.p1().get_single_mut() {
            camera_transform.translation = camera_transform.translation.lerp(
                Vec3::new(
                    player_transform.translation.x,
                    player_transform.translation.y,
                    camera_transform.translation.z,
                ),
                CAMERA_INTERPOLATION,
            );
        }
    }
}

pub fn cinematic_camera_follow(
    mut queries: ParamSet<(
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<PlayerCamera>>,
    )>,
) {
    if let Ok(t) = queries.p0().get_single() {
        let player_transform = *t;

        if let Ok(mut camera_transform) = queries.p1().get_single_mut() {
            camera_transform.translation = camera_transform.translation.lerp(
                Vec3::new(
                    player_transform.translation.x,
                    player_transform.translation.y,
                    camera_transform.translation.z,
                ),
                CAMERA_INTERPOLATION,
            );
        }
    }
}

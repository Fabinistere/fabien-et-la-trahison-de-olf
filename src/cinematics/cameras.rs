use bevy::prelude::*;

use crate::{characters::player::Player, constants::character::player::CAMERA_INTERPOLATION};

use super::CameraFocus;

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Component)]
pub struct CinematicCamera;

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
    mut cinematic_camera: Query<&mut Transform, With<CinematicCamera>>,
    camera_target: Query<&Transform, (With<CameraFocus>, Without<CinematicCamera>)>,
) {
    if let Ok(mut camera_transform) = cinematic_camera.get_single_mut() {
        let pop_star_transform = camera_target.single();
        camera_transform.translation = camera_transform.translation.lerp(
            Vec3::new(
                pop_star_transform.translation.x,
                pop_star_transform.translation.y,
                camera_transform.translation.z,
            ),
            CAMERA_INTERPOLATION,
        );
    }
}

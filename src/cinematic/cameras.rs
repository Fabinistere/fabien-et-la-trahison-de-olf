use bevy::prelude::*;
use rand::seq::IteratorRandom;

use crate::{
    characters::{player::Player, Character},
    constants::character::player::CAMERA_INTERPOLATION,
};

use super::{CameraFocus, PlayMode};

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Component)]
pub struct CinematicCamera;

pub fn follow_a_random_character(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,

    pop_star_query: Query<Entity, With<CameraFocus>>,
    characters_query: Query<(Entity, &Name), (With<Character>, Without<CameraFocus>)>,
    player_query: Query<Entity, With<Player>>,

    current_play_mode: Res<State<PlayMode>>,
    mut next_play_mode: ResMut<NextState<PlayMode>>,
) {
    if keyboard_input.just_pressed(KeyCode::C) {
        if let Ok(previously_pop_star) = pop_star_query.get_single() {
            commands.entity(previously_pop_star).remove::<CameraFocus>();
        }

        let mut rng = rand::thread_rng();
        let (random_pop_star, pop_star_name) = characters_query.iter().choose(&mut rng).unwrap();
        info!("{pop_star_name:?} is the new pop star");

        let player = player_query.single();
        if random_pop_star == player {
            next_play_mode.set(PlayMode::PlayerIsInControl);
        } else {
            commands
                .entity(random_pop_star)
                .insert(CameraFocus::default());

            if *current_play_mode != PlayMode::InCinematic {
                next_play_mode.set(PlayMode::InCinematic);
            }
        }
    }
}

pub fn reset_camera_to_player(
    mut cinematic_camera: Query<&mut Camera, (With<CinematicCamera>, Without<PlayerCamera>)>,
    mut player_camera: Query<
        (&mut Camera, &mut Transform),
        (With<PlayerCamera>, Without<CinematicCamera>),
    >,
) {
    // Remove current camera
    if let Ok(mut camera) = cinematic_camera.get_single_mut() {
        camera.is_active = false;
    }

    // Reactivate `PlayerCamera`
    if let Ok((mut camera, _camera_transform)) = player_camera.get_single_mut() {
        camera.is_active = true;
    }
}

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

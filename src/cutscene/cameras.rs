use bevy::prelude::*;
use rand::seq::IteratorRandom;

use crate::{
    characters::{player::Player, Character},
    constants::character::player::CAMERA_INTERPOLATION,
    cutscene::PlayerIsInControl,
};

use super::{CameraFocus, PlayMode};

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Component)]
pub struct CinematicCamera;

/* ---------- Startup, OnEnter and OnExit systems  ---------- */

pub fn spawn_cinematic_camera(mut commands: Commands) {
    let mut cinematic_camera = Camera2dBundle::default();
    cinematic_camera.projection.scale = 0.08;
    cinematic_camera.camera.is_active = false; // will be changed in perform_camera_swap
    cinematic_camera.camera.order = 1;
    commands.spawn((cinematic_camera, CinematicCamera));
}

pub fn perform_camera_swap(
    mut cinematic_camera_query: Query<&mut Camera, (With<CinematicCamera>, Without<PlayerCamera>)>,
    mut player_camera_query: Query<&mut Camera, With<PlayerCamera>>,
) {
    let mut cinematic_camera = cinematic_camera_query.single_mut();
    let mut player_camera = player_camera_query.single_mut();

    cinematic_camera.is_active = !cinematic_camera.is_active;
    player_camera.is_active = !player_camera.is_active;
}

/* --------------------- Update systems --------------------- */

pub fn follow_a_random_character(
    mut commands: Commands,

    camera_target_query: Query<Entity, With<CameraFocus>>,
    characters_query: Query<(Entity, &Name), (With<Character>, Without<CameraFocus>)>,
    player_query: Query<Entity, With<Player>>,

    mut player_is_in_control_resource: ResMut<PlayerIsInControl>,
    current_play_mode: Res<State<PlayMode>>,
    mut next_play_mode: ResMut<NextState<PlayMode>>,
) {
    if let Ok(previous_camera_target) = camera_target_query.get_single() {
        commands
            .entity(previous_camera_target)
            .remove::<CameraFocus>();
    }

    let mut rng = rand::thread_rng();
    let (random_pop_star, pop_star_name) = characters_query.iter().choose(&mut rng).unwrap();
    info!("{pop_star_name:?} is the new pop star");

    let player = player_query.single();
    if random_pop_star == player {
        commands.entity(player).insert(CameraFocus::default());

        if *current_play_mode != PlayMode::Improvisation {
            next_play_mode.set(PlayMode::Improvisation);
            info!(target:"States", "PlayMode::Improvisation sended");
        }
    } else {
        commands
            .entity(random_pop_star)
            .insert(CameraFocus::default());

        player_is_in_control_resource.0 = true;
        // FIXME: instant switch to PlayMode::Improvisation
        if *current_play_mode != PlayMode::InCinematic {
            next_play_mode.set(PlayMode::InCinematic);
            info!(target:"States", "PlayMode::InCinematic sended");
        }
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
        // println!("cinematic_camera_follow");
        if let Ok(pop_star_transform) = camera_target.get_single() {
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
}

use bevy::prelude::*;
use cameras::PlayerCamera;
pub mod cameras;

pub struct CinematicPlugin;

impl Plugin for CinematicPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<PlayMode>()
            .add_systems(
                Update,
                (
                    cameras::player_camera_follow,
                    cameras::follow_a_random_character,
                ),
            )
            .add_systems(OnEnter(PlayMode::InCinemmatic), perform_camera_swap)
            .add_systems(
                Update,
                cameras::cinematic_camera_follow.run_if(in_state(PlayMode::InCinemmatic)),
            )
            .add_systems(
                OnExit(PlayMode::InCinemmatic),
                cameras::reset_camera_to_player,
            );
    }
}

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone)]
pub enum PlayMode {
    #[default]
    PlayerIsInControl,
    InCinemmatic,
}

#[derive(Default)]
pub enum CameraFocusType {
    #[default]
    Normal,
    Shaky,
    DramaticZoom,
    SupriseZoom,
}

#[derive(Component, Default)]
pub struct CameraFocus {
    focus_type: CameraFocusType,
    finished: bool,
}

fn perform_camera_swap(
    mut commands: Commands,
    camera_target: Query<Entity, Added<CameraFocus>>,
    mut player_camera_query: Query<&mut Camera, With<PlayerCamera>>,
) {
    if let Ok(_pop_star) = camera_target.get_single() {
        let mut cinematic_camera = Camera2dBundle::default();
        cinematic_camera.projection.scale = 0.1;
        cinematic_camera.camera.is_active = true;
        cinematic_camera.camera.order = 1;
        commands.spawn((cinematic_camera, cameras::CinematicCamera));

        let mut player_camera = player_camera_query.single_mut();
        player_camera.is_active = false;
    } else {
        let mut player_camera = player_camera_query.single_mut();
        player_camera.is_active = true;
    }
}

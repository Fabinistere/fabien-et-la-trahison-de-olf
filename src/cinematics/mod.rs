use bevy::prelude::*;
pub mod cameras;

pub struct CinematicPlugin;

impl Plugin for CinematicPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<PlayMode>()
            .add_systems(Update, cameras::player_camera_follow)
            .add_systems(
                Update,
                (perform_camera_swap, cameras::cinematic_camera_follow)
                    .run_if(in_state(PlayMode::InCinemmatic)),
            )
            .add_systems(OnExit(PlayMode::InCinemmatic), reset_camera_to_player);
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

fn reset_camera_to_player() {
    // Remove current camera
    // Reactivate `PlayerCamera`
}

fn perform_camera_swap(mut commands: Commands, camera_target: Query<Entity, Added<CameraFocus>>) {
    if let Ok(_pop_star) = camera_target.get_single() {
        let mut camera = Camera2dBundle::default();
        camera.projection.scale = 0.1;
        commands.spawn((camera, cameras::CinematicCamera));
    }
}

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use serde::{Deserialize, Serialize};

use crate::GameState;

pub mod cameras;
mod staging;

pub struct CinematicPlugin;

impl Plugin for CinematicPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PlayMode>()
            .insert_resource(PlayerIsInControl::default())
            .add_systems(Startup, cameras::spawn_cinematic_camera)
            .add_systems(OnEnter(GameState::Playing), staging::spawn_cutscene)
            .add_systems(
                Update,
                (
                    cameras::player_camera_follow,
                    cameras::follow_a_random_character.run_if(input_just_pressed(KeyCode::KeyC)),
                    staging::run_cutscene.run_if(in_state(GameState::Playing)),
                    staging::frame_timer.run_if(in_state(GameState::Playing)),
                    staging::cutscene_characters_movement
                        .run_if(in_state(GameState::Playing))
                        .run_if(in_state(PlayMode::InCinematic)),
                ),
            )
            .add_systems(OnEnter(PlayMode::InCinematic), cameras::perform_camera_swap)
            .add_systems(
                Update,
                cameras::cinematic_camera_follow.run_if(in_state(PlayMode::InCinematic)),
            )
            .add_systems(OnExit(PlayMode::InCinematic), cameras::perform_camera_swap);
    }
}

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone, Reflect)]
pub enum PlayMode {
    /// Not in Cinematic
    #[default]
    Improvisation,
    /// Some entities has a precise control on where they should go and with specific camera settings.
    /// The `CinematicCamera` will be used in this state.
    /// Use the Resource `PlayerIsInControl` to let the player move freely or not.
    InCinematic,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum CameraFocusType {
    #[default]
    Normal,
    Shaky,
    DramaticZoom,
    SurpriseZoom,
}

#[derive(Component, Default)]
pub struct CameraFocus {
    pub _focus_type: CameraFocusType,
    /// REFACTOR: hum... why?
    pub _finished: bool,
}

#[derive(Resource)]
pub struct PlayerIsInControl(bool);

impl Default for PlayerIsInControl {
    fn default() -> Self {
        PlayerIsInControl(true)
    }
}

/* -------------------------------------------------------------------------- */
/*                                   Run If                                   */
/* -------------------------------------------------------------------------- */

pub fn player_is_in_control(player_is_in_control_resource: Res<PlayerIsInControl>) -> bool {
    player_is_in_control_resource.0
}

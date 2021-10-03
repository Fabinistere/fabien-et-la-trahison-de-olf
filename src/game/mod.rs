use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::GameState;

mod player;

pub struct GamePlugin;
pub struct PlayerCamera;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(player::PlayerPlugin)
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(setup.system())
            );
    }
}

fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.gravity = Vector::zeros();
    rapier_config.scale = 10.0;

    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(PlayerCamera);
}

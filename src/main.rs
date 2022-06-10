#![allow(clippy::type_complexity)]

pub mod animations;
pub mod collisions;
pub mod constants;
pub mod dialogs;
mod locations;
mod menu;
pub mod player;
mod ui;

use bevy::prelude::*;
// use bevy_prototype_debug_lines::*;
use bevy_rapier2d::prelude::*;

pub use crate::{
    constants::BACKGROUND_COLOR,
    dialogs::{DialogId, Dialogs, Language},
};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Menu,
    Playing,
}

#[derive(Component)]
struct PlayerCamera;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Fabien et le trahison de Olf".to_string(),
            // vsync: true,
            // mode: bevy::window::WindowMode::BorderlessFullscreen,
            ..WindowDescriptor::default()
        })
        .insert_resource(Msaa::default())
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierDebugRenderPlugin {
            depth_test: false,
            ..RapierDebugRenderPlugin::default()
        })
        .add_plugin(bevy_tweening::TweeningPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        .add_plugin(dialogs::DialogsPlugin)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(animations::AnimationPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(locations::LocationsPlugin)
        .add_plugin(ui::UiPlugin)
        .add_state(GameState::Playing)
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(game_setup))
        .run();
}

fn game_setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut windows: ResMut<Windows>,
) {
    windows.primary_mut().set_scale_factor_override(Some(1.0));
    rapier_config.gravity = Vect::ZERO;
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(PlayerCamera);
}

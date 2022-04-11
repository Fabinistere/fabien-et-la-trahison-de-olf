#![allow(clippy::type_complexity)]

pub mod animations;
pub mod constants;
// mod debug;
pub mod dialogs;
mod locations;
mod menu;
pub mod player;

use bevy::prelude::*;
// use bevy_prototype_debug_lines::*;
use bevy_rapier2d::prelude::*;

pub use crate::{
    constants::BACKGROUND_COLOR,
    dialogs::{language::Language, DialogId, Dialogs},
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
        // .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        // .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(DebugLinesPlugin)
        .add_plugin(dialogs::DialogsPlugin)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(animations::AnimationPlugin)
        .add_state(GameState::Playing)
        // .add_system(debug::collider_debug_lines_system)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(locations::LocationsPlugin)
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(game_setup))
        .run();
}

fn game_setup(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.gravity = Vector::zeros();
    rapier_config.scale = 1.0;

    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(PlayerCamera);

    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_lock_mode(true);
    window.set_cursor_visibility(false);
}

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
        .add_plugin(RapierDebugRenderPlugin {
            depth_test: false,
            ..RapierDebugRenderPlugin::default()
        })
        .insert_resource(Msaa::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugin(dialogs::DialogsPlugin)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(animations::AnimationPlugin)
        .add_state(GameState::Playing)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(locations::LocationsPlugin)
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(game_setup))
        .run();
}

fn game_setup(
    mut commands: Commands,
    // mut windows: ResMut<Windows>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.gravity = Vect::ZERO;
    // rapier_config.scale = 1.0;

    /*
    let mut camera = OrthographicCameraBundle::new_2d();
    info!("{:?}", camera.transform.translation);
    camera.transform.translation += Vec3::new(0.0, 0.0, -800.0);
    */

    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        // .spawn_bundle(camera)
        .insert(PlayerCamera);

    /*
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_lock_mode(true);
    window.set_cursor_visibility(false);
    */
}

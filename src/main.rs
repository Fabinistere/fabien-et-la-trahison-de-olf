pub mod dialogs;
mod menu;
mod game;
mod debug;

use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
use bevy_rapier2d::prelude::*;

pub use crate::{
    dialogs::{
        Dialogs,
        DialogId,
        language::Language,
    },
};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Menu,
    Playing,
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Fabien et le trahison de Olf".to_string(),
            // vsync: true,
            // mode: bevy::window::WindowMode::BorderlessFullscreen,
            ..WindowDescriptor::default()
        })
        // .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        // .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(DebugLinesPlugin)
        .add_plugin(dialogs::DialogsPlugin)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(game::GamePlugin)
        .add_state(GameState::Playing)
        .add_system(debug::collider_debug_lines_system.system())
        .run();
}

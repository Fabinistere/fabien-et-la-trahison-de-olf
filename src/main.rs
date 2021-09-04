mod menu;

use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Menu,
    Playing,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(menu::MenuPlugin)
        .add_state(GameState::Menu)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}

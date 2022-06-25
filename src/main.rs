#![allow(clippy::type_complexity)]

pub mod animations;
mod collisions;
pub mod constants;
pub mod controls;
pub mod dialogs;
pub mod interactions;
mod locations;
pub mod material;
mod menu;
pub mod player;
mod ui;

use bevy::{prelude::*, sprite::Material2dPlugin};
// use bevy_prototype_debug_lines::*;
use bevy_rapier2d::prelude::*;

pub use crate::{
    constants::BACKGROUND_COLOR,
    controls::Key,
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
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: "Fabien et le trahison de Olf".to_string(),
        // vsync: true,
        // mode: bevy::window::WindowMode::BorderlessFullscreen,
        ..WindowDescriptor::default()
    })
    .insert_resource(Msaa::default())
    .insert_resource(ClearColor(BACKGROUND_COLOR))
    .insert_resource(controls::KeyBindings {
        up: [Key(KeyCode::Z), Key(KeyCode::Up)],
        down: [Key(KeyCode::S), Key(KeyCode::Down)],
        right: [Key(KeyCode::D), Key(KeyCode::Right)],
        left: [Key(KeyCode::Q), Key(KeyCode::Left)],
        interact: [Key(KeyCode::E), Key(KeyCode::R)],
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(Material2dPlugin::<material::CustomMaterial>::default())
    .add_plugin(bevy_tweening::TweeningPlugin)
    .add_plugin(RapierDebugRenderPlugin {
        depth_test: false,
        ..RapierDebugRenderPlugin::default()
    })
    .add_state(GameState::Playing)
    .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(game_setup))
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
    .add_plugin(dialogs::DialogsPlugin)
    .add_plugin(menu::MenuPlugin)
    .add_plugin(animations::AnimationPlugin)
    .add_plugin(player::PlayerPlugin)
    .add_plugin(locations::LocationsPlugin)
    .add_plugin(interactions::InteractionsPlugin)
    .add_plugin(ui::UiPlugin);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_web_resizer::Plugin);

    app.run();
}

fn game_setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut windows: ResMut<Windows>,
    asset_server: Res<AssetServer>,
) {
    asset_server.watch_for_changes().unwrap();

    windows.primary_mut().set_scale_factor_override(Some(1.0));
    rapier_config.gravity = Vect::ZERO;
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(PlayerCamera);
}

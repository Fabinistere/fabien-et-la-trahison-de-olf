#![allow(clippy::type_complexity)]

pub mod animations;
mod collisions;
pub mod constants;
pub mod controls;
pub mod dialogs;
pub mod interactions;
mod locations;
mod menu;
pub mod player;
mod ui;

use bevy::{ecs::schedule::ScheduleBuildSettings, prelude::*};
use bevy_rapier2d::prelude::*;

pub use crate::{
    constants::BACKGROUND_COLOR,
    controls::Key,
    dialogs::{DialogId, Dialogs, Language},
};

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    Menu,
    #[default]
    Playing,
}

#[derive(Component)]
struct PlayerCamera;

fn main() {
    let mut app = App::new();
    app
        // .insert_resource(Msaa::default())
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(controls::KeyBindings {
            up: [Key(KeyCode::Z), Key(KeyCode::Up)],
            down: [Key(KeyCode::S), Key(KeyCode::Down)],
            right: [Key(KeyCode::D), Key(KeyCode::Right)],
            left: [Key(KeyCode::Q), Key(KeyCode::Left)],
            interact: [Key(KeyCode::E), Key(KeyCode::R)],
        })
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Fabien et le trahison de Olf".to_string(),
                        // vsync: true,
                        // mode: bevy::window::WindowMode::BorderlessFullscreen,
                        ..Window::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..default()
                }),
        )
        .add_plugin(bevy_tweening::TweeningPlugin)
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_state::<GameState>()
        .add_system(game_setup.in_schedule(OnEnter(GameState::Playing)))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        .add_plugin(dialogs::DialogsPlugin)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(animations::AnimationPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(locations::LocationsPlugin)
        .add_plugin(interactions::InteractionsPlugin)
        .add_plugin(ui::UiPlugin)
        .add_plugin(collisions::CollisionsPlugin);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_web_resizer::Plugin);

    app.edit_schedule(CoreSchedule::Main, |schedule| {
        schedule.set_build_settings(ScheduleBuildSettings {
            ambiguity_detection: bevy::ecs::schedule::LogLevel::Warn,
            ..default()
        });
    });

    app.run();
}

fn game_setup(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vect::ZERO;
    commands.spawn((Camera2dBundle::default(), PlayerCamera));
}

#![allow(clippy::type_complexity)]

pub mod animations;
pub mod characters;
mod collisions;
pub mod constants;
pub mod controls;
pub mod debug;
pub mod dialogs;
pub mod interactions;
mod locations;
mod menu;
mod ui;

use std::time::Duration;

use bevy::{asset::ChangeWatcher, ecs::schedule::ScheduleBuildSettings, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::{
    constants::{BACKGROUND_COLOR_INGAME, BACKGROUND_COLOR_INMENU},
    controls::Key,
    dialogs::{DialogId, Dialogs, Language},
};

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, Reflect, States)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
}

#[derive(Component)]
struct PlayerCamera;

fn main() {
    let mut app = App::new();
    app.insert_resource(Msaa::Off)
        .insert_resource(ClearColor(BACKGROUND_COLOR_INMENU))
        .insert_resource(controls::KeyBindings {
            up: [Key(KeyCode::Z), Key(KeyCode::Up)],
            down: [Key(KeyCode::S), Key(KeyCode::Down)],
            right: [Key(KeyCode::D), Key(KeyCode::Right)],
            left: [Key(KeyCode::Q), Key(KeyCode::Left)],
            interact: [Key(KeyCode::E), Key(KeyCode::R)],
        })
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Fabien et le trahison de Olf".to_string(),
                        // vsync: true,
                        mode: bevy::window::WindowMode::BorderlessFullscreen,
                        ..Window::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
                    ..default()
                }),
            bevy_tweening::TweeningPlugin,
            RapierDebugRenderPlugin::default(),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.),
            // ----- Our plugins -----
            animations::AnimationPlugin,
            dialogs::DialogsPlugin,
            debug::DebugPlugin,
            collisions::CollisionsPlugin,
            interactions::InteractionsPlugin,
            locations::LocationsPlugin,
            menu::MenuPlugin,
            characters::CharactersPlugin,
            ui::UiPlugin,
        ))
        .add_state::<GameState>()
        .add_systems(Startup, game_setup)
        .add_systems(OnEnter(GameState::Playing), setup_background_playing);

    app.edit_schedule(Main, |schedule| {
        schedule.set_build_settings(ScheduleBuildSettings {
            ambiguity_detection: bevy::ecs::schedule::LogLevel::Warn,
            ..default()
        });
    });

    app.run();
}

fn game_setup(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vect::ZERO;

    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.1;
    commands.spawn((camera, PlayerCamera));
}

fn setup_background_playing(mut clear_color: ResMut<ClearColor>) {
    clear_color.0 = BACKGROUND_COLOR_INGAME;
}

pub fn playing(game_state: Res<State<GameState>>) -> bool {
    game_state.get() == &GameState::Playing
}

pub fn in_menu(game_state: Res<State<GameState>>) -> bool {
    game_state.get() == &GameState::Menu
}

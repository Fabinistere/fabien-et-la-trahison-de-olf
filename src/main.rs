#![allow(
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::pedantic,
    clippy::nursery
)]
// #![warn(missing_docs)]

pub mod animations;
pub mod characters;
mod collisions;
pub mod combat;
pub mod constants;
pub mod controls;
mod cutscene;
pub mod debug;
pub mod dialogs;
pub mod interactions;
mod locations;
mod menu;
mod ui;

// use std::io::Write; // for infox!

use bevy::{ecs::schedule::ScheduleBuildSettings, prelude::*};
use bevy_rapier2d::prelude::*;
use cutscene::{cameras::PlayerCamera, PlayMode};
/* ------------------------ LOGGING  ------------------------ */
// use std::sync::OnceLock;
// use tracing_appender::{non_blocking::WorkerGuard, rolling, rolling::never};
// use tracing_subscriber::{layer::SubscriberExt, Registry, filter};
use bevy::log::LogPlugin;
// to use our custom logger
pub use log::{error, info, warn};
/* --------------------------------------------------------- */

use crate::{
    constants::{BACKGROUND_COLOR_INGAME, BACKGROUND_COLOR_INMENU},
    controls::Key,
    debug::setup_logging,
    dialogs::{DialogId, Dialogs, Language},
};

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, Reflect, States)]
pub enum GameState {
    #[default]
    Menu,
    /// Game without any HUD.
    /// Exploration.
    Playing,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, Reflect, States)]
pub enum HUDState {
    #[default]
    Closed,
    // /// is also the Team's Inventory
    CombatWall,
    // LogCave,
    DialogWall,
    OptionsWall,
}

// fn custom_layer(_app: &mut App) -> Option<BoxedLayer> {
//     let file_appender = rolling::daily("logs", "app.log");
//     let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
//     let _ = LOG_GUARD.set(guard);
//     Some(bevy::log::tracing_subscriber::fmt::layer()
//             .with_writer(non_blocking)
//             .with_file(true)
//             .with_line_number(true)
//             .boxed())
// }

fn main() {
    let mut app = App::new();

    // #[cg(debug_assertions)]
    // app.add_plugins(RapierDebugRenderPlugin::default());

    /* ------------------------ Logging  ------------------------ */
    let session_time = chrono::Local::now().format("%Y-%m-%dT%H-%M-%S");
    let log_dir = format!("logs/{session_time}");

    setup_logging(&log_dir).expect("Failed to init logging");
    /* ---------------------------------------------------------- */

    app.insert_resource(Msaa::Off)
        .insert_resource(ClearColor(BACKGROUND_COLOR_INMENU))
        .insert_resource(controls::KeyBindings {
            up: [
                Key(KeyCode::KeyW),
                Key(KeyCode::KeyZ),
                Key(KeyCode::ArrowUp),
            ],
            down: [Key(KeyCode::KeyS), Key(KeyCode::ArrowDown)],
            right: [Key(KeyCode::KeyD), Key(KeyCode::ArrowRight)],
            left: [
                Key(KeyCode::KeyA),
                Key(KeyCode::KeyQ),
                Key(KeyCode::ArrowLeft),
            ],
            interact: [Key(KeyCode::KeyE), Key(KeyCode::KeyR)],
        })
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Fabien et la Trahison de Olf".to_string(),
                        // vsync: true,
                        mode: bevy::window::WindowMode::Windowed,
                        // mode: bevy::window::WindowMode::BorderlessFullscreen,
                        ..Window::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                //,
                // .set(LogPlugin {
                //     update_subscriber: Some(|subscriber| {
                //         // File writer for the "Audio" target
                //         let file_appender = never(&log_dir, "Audio.log");
                //         let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
                //
                //         let audio_layer = tracing_subscriber::fmt::layer()
                //             .with_writer(non_blocking)
                //             .with_filter(
                //                 filter::Targets::new()
                //                     .with_target("Audio", Level::INFO)
                //             );
                //
                //         Box::new(subscriber.with(audio_layer))
                //     }),
                //     ..default()
                // })
                // .set(LogPlugin {
                //     custom_layer,
                //     ..default()
                // }),
                .disable::<LogPlugin>(),
            bevy_tweening::TweeningPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.),
            // ----- Our plugins -----
            animations::AnimationPlugin,
            dialogs::DialogsPlugin,
            debug::DebugPlugin,
            collisions::CollisionsPlugin,
            interactions::InteractionsPlugin,
            locations::LocationsPlugin,
            cutscene::CinematicPlugin,
            menu::MenuPlugin,
            characters::CharactersPlugin,
            combat::CombatPlugin,
            ui::UiPlugin,
        ))
        .init_state::<GameState>()
        .init_state::<HUDState>()
        .add_systems(Startup, (game_setup, music))
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
    // Higher order camera (UI is displayed onto this one)
    camera.camera.order = 2;
    commands.spawn((camera, PlayerCamera));
}

fn setup_background_playing(mut clear_color: ResMut<ClearColor>) {
    clear_color.0 = BACKGROUND_COLOR_INGAME;
}

/// Marker component for our music entity
#[derive(Component)]
struct CastleTheme;

fn music(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("sounds/FTO_Dracula_theme.ogg"),
            settings: PlaybackSettings::LOOP.with_volume(bevy::audio::Volume::new(0.10)),
        },
        CastleTheme,
    ));

    info!(target: "Audio", "audio playing...");
    // debug::infox!(&["Audio"], "audio playing...");
}

/* -------------------------------------------------------------------------- */
/*                                   Run If                                   */
/* -------------------------------------------------------------------------- */

// REFACTOR: use `in_state` instead of redefining the functions

pub fn playing(game_state: Res<State<GameState>>) -> bool {
    game_state.get() == &GameState::Playing
}

pub fn in_menu(game_state: Res<State<GameState>>) -> bool {
    game_state.get() == &GameState::Menu
}

pub fn in_cutscene(play_mode: Res<State<PlayMode>>) -> bool {
    play_mode.get() == &PlayMode::InCinematic
}

pub fn hud_closed(hud_state: Res<State<HUDState>>) -> bool {
    hud_state.get() == &HUDState::Closed
}

pub fn hud_opened(hud_state: Res<State<HUDState>>) -> bool {
    hud_state.get() != &HUDState::Closed
}

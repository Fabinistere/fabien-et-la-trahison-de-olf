#![allow(clippy::type_complexity)]
#![feature(trivial_bounds)]
// // ^^--- allow reflect on Vec<T>

pub mod animations;
pub mod characters;
mod collisions;
pub mod combat;
pub mod constants;
pub mod controls;
pub mod debug;
pub mod dialogs;
pub mod interactions;
mod locations;
mod menu;
mod ui;

use std::time::Duration;

use bevy::{
    asset::ChangeWatcher, audio::VolumeLevel, ecs::schedule::ScheduleBuildSettings, prelude::*,
};
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

/// We might want to do something like that but
/// it will created so much dualistic situation with `HUDState`
///
/// ```rust
/// pub enum PlayingState {
///     NonPlaying,
///     WalkSimulator,
///     Combat,
///     /// The state we need to distinguish with `PlayingState::Combat`
///     CombatPrep,
///     Dialog,
/// }
/// ```
///
/// # Notes
///
/// We still don't want to create another state of `HUDState`,
/// as not much chnage (just selectionSkill can't click skill anymore in `Preparation`).
/// And especially the  physical Wall is the same.
///
/// Creating new `CombatState`, as `ObserveSkill`, is still problematic
/// because we cannot distinguish in `SelectionCaster` in which state we are.
/// A simple `Resource`/`States` might be the solution to check at macro where we need.
///
/// NOTE: it can be a `Resource`
#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, Reflect, States)]
pub enum CombatWallStage {
    #[default]
    Closed,
    Preparation,
    InCombat,
}

/// DOC: explain the transition for State
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, Reflect, States)]
pub enum HUDState {
    #[default]
    Closed,
    /// is also the Team's Inventory
    CombatWall,
    /// Accessible from the CombatWall
    LogCave,
    DialogWall,
    OptionsWall,
}

#[derive(Component)]
struct PlayerCamera;

fn main() {
    let mut app = App::new();

    #[cfg(debug_assertions)]
    app.add_plugins(RapierDebugRenderPlugin::default());
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    app.insert_resource(Msaa::Off)
        .insert_resource(ClearColor(BACKGROUND_COLOR_INMENU))
        .insert_resource(controls::KeyBindings {
            up: [Key(KeyCode::W), Key(KeyCode::Z), Key(KeyCode::Up)],
            down: [Key(KeyCode::S), Key(KeyCode::Down)],
            right: [Key(KeyCode::D), Key(KeyCode::Right)],
            left: [Key(KeyCode::A), Key(KeyCode::Q), Key(KeyCode::Left)],
            interact: [Key(KeyCode::E), Key(KeyCode::R)],
        })
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Fabien et la Trahison de Olf".to_string(),
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
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.),
            // ----- Our plugins -----
            debug::DebugPlugin,
            animations::AnimationPlugin,
            menu::MenuPlugin,
            collisions::CollisionsPlugin,
            characters::CharactersPlugin,
            locations::LocationsPlugin,
            interactions::InteractionsPlugin,
            ui::UiPlugin,
            dialogs::DialogsPlugin,
            combat::CombatPlugin,
        ))
        .add_state::<GameState>()
        .add_state::<HUDState>()
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
    // NOTE: Why is this time, gravity settings are required ?
    rapier_config.gravity = Vect::ZERO;
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.1;
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
            settings: PlaybackSettings::LOOP
                .with_volume(bevy::audio::Volume::Relative(VolumeLevel::new(0.10))),
        },
        CastleTheme,
    ));

    info!(target: "misc", "audio playing...");
}

/* -------------------------------------------------------------------------- */
/*                                   Run If                                   */
/* -------------------------------------------------------------------------- */

pub fn playing(game_state: Res<State<GameState>>) -> bool {
    game_state.get() == &GameState::Playing
}

pub fn in_menu(game_state: Res<State<GameState>>) -> bool {
    game_state.get() == &GameState::Menu
}

pub fn hud_closed(hud_state: Res<State<HUDState>>) -> bool {
    hud_state.get() == &HUDState::Closed
}

pub fn hud_opened(hud_state: Res<State<HUDState>>) -> bool {
    hud_state.get() != &HUDState::Closed
}

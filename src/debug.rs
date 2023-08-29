use bevy::prelude::*;
use bevy_inspector_egui::quick::{StateInspectorPlugin, WorldInspectorPlugin};

use crate::{
    animations::sprite_sheet_animation::{AnimationIndices, CharacterState, TempoAnimation},
    collisions::{TesselatedCollider, TesselatedColliderConfig},
    locations::temple::{OverlappingEntity, PlayerLocation},
    GameState,
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugins((WorldInspectorPlugin::new(),))
                .register_type::<GameState>()
                .register_type::<PlayerLocation>()
                .add_plugins((
                    StateInspectorPlugin::<GameState>::default(),
                    StateInspectorPlugin::<PlayerLocation>::default(),
                ))
                /* -------------------------------------------------------------------------- */
                /*                              Global Animation                              */
                /* -------------------------------------------------------------------------- */
                .register_type::<TempoAnimation>()
                /* -------------------------------------------------------------------------- */
                /*                                  Character                                 */
                /* -------------------------------------------------------------------------- */
                .register_type::<AnimationIndices>()
                .register_type::<CharacterState>()
                /* -------------------------------------------------------------------------- */
                /*                                     Map                                    */
                /* -------------------------------------------------------------------------- */
                .register_type::<OverlappingEntity>()
                /* -------------------------------------------------------------------------- */
                /*                                   Hitbox                                   */
                /* -------------------------------------------------------------------------- */
                .register_type::<TesselatedCollider>()
                .register_type::<TesselatedColliderConfig>();
        }
    }
}

// TODO: Create debug log kind
// Combat Debug
// Movement Debug
// Dialog Debug
// ...

// make it clear in the global log (different files ?)
//   - global log file
//   - specific (Combat/Movement/Dialog) log file
// ask for sending logs and data to *me* when game crash

// TODO: Create Custom Lint Rule
// function using query not being added to a plugin
// event ...
// plugin ...

// TODO: Create Contribution Example
// for
// - fn
// - struct
//   - Component
//   - Event
//   - Plugin
// - Module

use bevy::prelude::*;
use bevy_inspector_egui::quick::{StateInspectorPlugin, WorldInspectorPlugin};

use crate::{
    animations::sprite_sheet_animation::{
        AnimationIndices, CharacterState, SpriteSheetAnimation, TempoAnimation,
    },
    characters::npcs::movement::{Chaser, NPCBehavior, TargetSeeker, TargetType},
    collisions::{TesselatedCollider, TesselatedColliderConfig},
    locations::temple::{OverlappingEntity, PlayerLocation},
    menu::{ManorLightsPattern, ManorLightsTimer},
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
                /*                                    Menu                                    */
                /* -------------------------------------------------------------------------- */
                .register_type::<ManorLightsTimer>()
                .register_type::<ManorLightsPattern>()
                .register_type::<SpriteSheetAnimation>()
                /* -------------------------------------------------------------------------- */
                /*                                  Character                                 */
                /* -------------------------------------------------------------------------- */
                .register_type::<AnimationIndices>()
                .register_type::<CharacterState>()
                .register_type::<NPCBehavior>()
                .register_type::<TargetSeeker>()
                .register_type::<TargetType>()
                .register_type::<Chaser>()
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

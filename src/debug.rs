use bevy::prelude::*;
use bevy_inspector_egui::quick::{ResourceInspectorPlugin, StateInspectorPlugin, WorldInspectorPlugin};

use crate::{
    animations::sprite_sheet_animation::{
        AnimationIndices, CharacterState, SpriteSheetAnimation, TempoAnimation,
    }, characters::npcs::movement::{Chaser, NPCBehavior, TargetSeeker, TargetType}, collisions::{TesselatedCollider, TesselatedColliderConfig}, combat::{alterations::{Alteration, AlterationAction}, skills::{SkillType, TargetOption}, stats::{Attack, AttackSpe, Defense, DefenseSpe, Hp, Initiative, Mana, Shield}, stuff::{Equipements, Job, MasteryTier, WeaponType}, ActionCount, CombatState, TacticalPlace}, locations::{
        landmarks::Landmark,
        temple::{Location, OverlappingEntity},
    }, menu::{ManorLightsPattern, ManorLightsTimer}, ui::combat::combat_system::{ActionHistory, ActionsLogs, LastTurnActionHistory}, GameState
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        // log4rs::init_file("data/logging_config.yaml", Default::default()).unwrap();

        // trace!("detailed tracing info");
        // debug!("debug info");
        // info!("relevant general info");
        // warn!("warning this program doesn't do much");
        // error!("error message here");

        // let level = log::LevelFilter::Info;
        // let file_path = "/home/olf/Code/fabien-et-la-trahison-de-olf/log/all.log";
        // // Build a stderr logger.
        // let stderr = ConsoleAppender::builder().target(Target::Stderr).build();
        // // Logging to log file.
        // let logfile = FileAppender::builder()
        //     // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        //     .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        //     .build(file_path)
        //     .unwrap();

        // // Log Trace level output to file where trace is the default level
        // // and the programmatically specified level to stderr.
        // let config = Config::builder()
        //     .appender(Appender::builder().build("logfile", Box::new(logfile)))
        //     .appender(
        //         Appender::builder()
        //             .filter(Box::new(ThresholdFilter::new(level)))
        //             .build("stderr", Box::new(stderr)),
        //     )
        //     .build(
        //         Root::builder()
        //             .appender("logfile")
        //             .appender("stderr")
        //             .build(log::LevelFilter::Trace),
        //     )
        //     .unwrap();

        // let _handle = log4rs::init_config(config).unwrap();

        if cfg!(debug_assertions) {
            app.add_plugins((WorldInspectorPlugin::new(),))
                .register_type::<GameState>()
                .register_type::<Location>()
                .add_plugins((StateInspectorPlugin::<GameState>::default(),))
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
                .register_type::<Location>()
                /* -------------------------------------------------------------------------- */
                /*                                     Map                                    */
                /* -------------------------------------------------------------------------- */
                .register_type::<OverlappingEntity>()
                .register_type::<Landmark>()
                /* -------------------------------------------------------------------------- */
                /*                                   Hitbox                                   */
                /* -------------------------------------------------------------------------- */
                .register_type::<TesselatedCollider>()
                .register_type::<TesselatedColliderConfig>()
                
                /* --------------------------------- Combat --------------------------------- */
                /* -------------------------------------------------------------------------- */
                /*                          --- Global Structure ---                          */
                /* -------------------------------------------------------------------------- */
                
                .register_type::<CombatState>()
                // .register_type::<CombatResources>()
                // .register_type::<Action>()
                
                .register_type::<ActionCount>()

                // .register_type::<TacticalPosition>()
                .register_type::<TacticalPlace>()
                
                /* -------------------------------------------------------------------------- */
                /*                       --- Skills and Alterations ---                       */
                /* -------------------------------------------------------------------------- */

                .register_type::<Alteration>()
                .register_type::<AlterationAction>()
                .register_type::<TargetOption>()
                
                // .register_type::<Skill>()
                .register_type::<SkillType>()
                
                /* -------------------------------------------------------------------------- */
                /*                               --- Weapons ---                              */
                /* -------------------------------------------------------------------------- */
                
                .register_type::<Equipements>()
                .register_type::<WeaponType>()
                
                .register_type::<Job>()
                .register_type::<MasteryTier>()
                // .register_type::<JobsMasteries>()
                
                /* -------------------------------------------------------------------------- */
                /*                                --- Stats ---                               */
                /* -------------------------------------------------------------------------- */
                
                .register_type::<Hp>()
                .register_type::<Mana>()
                .register_type::<Shield>()
                .register_type::<Initiative>()
                .register_type::<Attack>()
                .register_type::<AttackSpe>()
                .register_type::<Defense>()
                .register_type::<DefenseSpe>()

                /* -------------------------------------------------------------------------- */
                /*                                 --- UI ---                                 */
                /* -------------------------------------------------------------------------- */

                .register_type::<ActionHistory>()
                .register_type::<LastTurnActionHistory>()
                .register_type::<ActionsLogs>()

                .add_plugins((
                    ResourceInspectorPlugin::<ActionHistory>::default(),
                    ResourceInspectorPlugin::<LastTurnActionHistory>::default(),
                    ResourceInspectorPlugin::<ActionsLogs>::default(),
                ));
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

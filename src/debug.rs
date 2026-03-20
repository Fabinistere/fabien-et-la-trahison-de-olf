use bevy::prelude::*;
use bevy_inspector_egui::quick::{StateInspectorPlugin, WorldInspectorPlugin};
// use std::fs::File;
// use std::io::Write;

use crate::{
    animations::sprite_sheet_animation::{
        AnimationIndices, CharacterState, SpriteSheetAnimation, TempoAnimation,
    },
    characters::npcs::movement::{Chaser, NPCBehavior, TargetSeeker, TargetType},
    collisions::{TessellatedCollider, TessellatedColliderConfig},
    cutscene::PlayMode,
    locations::{
        landmarks::Landmark,
        temple::{Location, OverlappingEntity},
    },
    menu::{ManorLightsPattern, ManorLightsTimer},
    GameState, HUDState,
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugins((WorldInspectorPlugin::new(),))
                .register_type::<GameState>()
                .register_type::<HUDState>()
                .register_type::<PlayMode>()
                .add_plugins((
                    StateInspectorPlugin::<GameState>::default(),
                    StateInspectorPlugin::<HUDState>::default(),
                    StateInspectorPlugin::<PlayMode>::default(),
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
                .register_type::<Location>()
                /* -------------------------------------------------------------------------- */
                /*                                     Map                                    */
                /* -------------------------------------------------------------------------- */
                .register_type::<OverlappingEntity>()
                .register_type::<Landmark>()
                /* -------------------------------------------------------------------------- */
                /*                                   Hitbox                                   */
                /* -------------------------------------------------------------------------- */
                .register_type::<TessellatedCollider>()
                .register_type::<TessellatedColliderConfig>();
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                                   LOGGING                                  */
/* -------------------------------------------------------------------------- */

use colored::Colorize;
use log4rs::{
    append::console::ConsoleAppender,
    append::console::Target,
    append::file::FileAppender,
    config::{Appender, Config, Logger, Root},
    encode::{Encode, Write},
    // encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};

#[derive(Debug)]
struct ColoredEncoder;

impl Encode for ColoredEncoder {
    fn encode(&self, w: &mut dyn Write, record: &log::Record) -> Result<(), anyhow::Error> {
        let level = match record.level() {
            log::Level::Error => format!("{:>5}", record.level()).red().bold(),
            log::Level::Warn => format!("{:>5}", record.level()).yellow().bold(),
            log::Level::Info => format!("{:>5}", record.level()).green(),
            log::Level::Debug => format!("{:>5}", record.level()).blue(),
            log::Level::Trace => format!("{:>5}", record.level()).dimmed(),
        };

        let time = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%.6fZ");
        writeln!(
            w,
            "{}  {} {}: {}",
            time,
            level,
            record.target(),
            record.args()
        )?;
        Ok(())
    }
}

/// To handle new target in logs, we must add it to the list
///
/// ## Notes
///
/// You must use `log::info!("...")` for it to be catch by our custom logger
/// You can also import `use log::{debug, error, info, trace, warn};` after the prelude import to
/// avoid writing `log::` everywhere
pub fn setup_logging(log_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(log_dir)?;

    let targets = &[
        "Animation",
        "NPC",
        "Combat",
        "Movement",
        "Dialog",
        "States",
        "Cutscene",
        "Audio",
        "Video",
    ];

    let silenced = &[
        "wgpu",
        "wgpu_core",
        "wgpu_hal",
        "naga",
        "winit",
        "gilrs",
        "gilrs_core",
    ];

    /* ------------------------ Appenders ------------------------ */
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(ColoredEncoder))
        .target(Target::Stdout)
        .build();

    let mut config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(
            Appender::builder().build(
                "global_file",
                Box::new(
                    FileAppender::builder()
                        .encoder(Box::new(ColoredEncoder))
                        .build(format!("{log_dir}/Global.log"))?,
                ),
            ),
        )
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(log::LevelFilter::Warn)))
                .build(
                    "warn_file",
                    Box::new(
                        FileAppender::builder()
                            .encoder(Box::new(ColoredEncoder))
                            .build(format!("{log_dir}/Warn.log"))?,
                    ),
                ),
        )
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(log::LevelFilter::Error)))
                .build(
                    "error_file",
                    Box::new(
                        FileAppender::builder()
                            .encoder(Box::new(ColoredEncoder))
                            .build(format!("{log_dir}/Error.log"))?,
                    ),
                ),
        );

    /* -------- One appender + logger per target (generated) -------- */
    for target in targets {
        let appender_name = format!("{}_file", target.to_lowercase());
        let appender = FileAppender::builder()
            // .encoder(Box::new(PatternEncoder::new(
            //     "{d(%Y-%m-%dT%H:%M:%S%.6fZ)} [{l}] {m}\n",
            // )))
            // The log files will have the color (see the README to how to display these colors)
            .encoder(Box::new(ColoredEncoder))
            .build(format!("{log_dir}/{target}.log"))?;

        config = config
            .appender(Appender::builder().build(&appender_name, Box::new(appender)))
            .logger(
                Logger::builder()
                    .appender(&appender_name)
                    // .additive(false)
                    .build(*target, log::LevelFilter::Info),
            );
    }

    /* ------------------------ Silence ------------------------ */
    for silenced_target in silenced {
        config = config.logger(
            Logger::builder()
                .additive(false)
                .build(*silenced_target, log::LevelFilter::Error),
        );
    }

    /* ---------------------------------------------------------- */
    let config = config.build(
        Root::builder()
            .appender("stdout")
            .appender("global_file")
            .appender("warn_file")
            .appender("error_file")
            .build(log::LevelFilter::Info),
    )?;

    log4rs::init_config(config)?;
    Ok(())
}

/// Giving a set of file paths and a string + arguments (same as the format! or print! macro)
/// - create the file at the giving path
/// - write in the file
/// - write in the standard output (print)
///
/// ## Use
///
/// ```rust
/// let targets: Vec<&'static str> = vec![
///     "NPC",
///     "Aggression",
/// ];
///
/// // info!(target: "NPC", "{} change zone. {:?}: chase canceled", npc_name, *target);
/// // is now:
/// infox!(targets, "{} change zone. {:?}: chase canceled", npc_name, *target);
///
/// infox!(targets, "A nice log message with optional values: {}", optional_value);
/// ```
macro_rules! infox {
    ($targets:expr, $($arg:tt)*) => {

        let time = "2026-03-14-19:57:21";

        let mut targets: Vec<&str> = $targets.to_vec();
        targets.push("Global");
        let targets_path = targets.iter().map(|name| {
            format!("logs/{time}/{name}.log")
        })
        .collect::<Vec<_>>();

        for target in targets {
            info!(target: target, $($arg)*);
        }

        for file_path in targets_path {
            if let Some(parent) = std::path::Path::new(&file_path).parent() {
                std::fs::create_dir_all(parent)
                    .expect(&format!("Failed to create directory: {parent:?}"));
            }
            let mut file = std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(file_path.clone())
                .expect(&format!("Failed to open file: {file_path:?}", ));

            std::writeln!(file, $($arg)*)
                .expect(&format!("Failed to write to file: {:?}", file_path));

        }
    };
}
// pub(crate) use infox;

// IDEA: debug - log every state change

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

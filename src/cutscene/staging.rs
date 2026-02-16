//! This module is focused on the staging part of the cinematic.
//! What each character has to do, where to move, what dialogue to start.

use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use serde::{Deserialize, Serialize};

use crate::{
    characters::{movement::Speed, Character},
    constants::locations::main_room::{THRONE_X, THRONE_Y},
    cutscene::{CameraFocus, CameraFocusType, PlayMode, PlayerIsInControl},
};

// TODO: cutscene - an event in the location sensor to trigger the temple entrance cutscene

#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct Cutscene {
    frames: Vec<FrameShot>,
}

impl Cutscene {
    pub fn new() -> Self {
        Self {
            frames: vec![
                FrameShot {
                    camera_focus_type: CameraFocusType::Normal,
                    player_is_in_control: false,
                    pop_star: Some("Player".to_string()),
                    dialogue: None,
                    positions: vec![("Player".to_string(), (THRONE_X, THRONE_Y - 100.))],
                    duration: 5.,
                },
                FrameShot {
                    camera_focus_type: CameraFocusType::Normal,
                    player_is_in_control: false,
                    pop_star: Some("NPC Supreme God".to_string()),
                    dialogue: Some("prout prout".to_string()),
                    positions: vec![],
                    duration: 2.,
                },
                FrameShot {
                    camera_focus_type: CameraFocusType::Normal,
                    player_is_in_control: false,
                    pop_star: Some("Player".to_string()),
                    dialogue: None,
                    positions: vec![("Player".to_string(), (THRONE_X, THRONE_Y - 50.))],
                    duration: 2.,
                },
                FrameShot {
                    camera_focus_type: CameraFocusType::Normal,
                    player_is_in_control: true,
                    pop_star: Some("NPC Supreme God".to_string()),
                    dialogue: None,
                    positions: vec![("NPC Supreme God".to_string(), (THRONE_X, THRONE_Y - 40.))],
                    duration: 5.,
                },
                // Control to the player while giving them destinations: the player has priority
                // FrameShot {
                //     camera_focus_type: CameraFocusType::Normal,
                //     player_is_in_control: true,
                //     pop_star: Some("Player".to_string()),
                //     dialogue: None,
                //     positions: vec![("Player".to_string(), (THRONE_X, THRONE_Y - 100.))],
                //     duration: 5.,
                // },
            ],
        }
    }

    /// Import from YML file
    ///
    /// YML script obtained by
    /// ```
    /// std::fs::write(
    ///     "data/cutscene/open_temple.yml",
    ///     serde_yaml::to_string(&Cutscene::new()).unwrap(),
    /// );
    /// ```
    pub fn import_from_script(file_path: String) -> Self {
        if let Ok(cutscene) = std::fs::read_to_string(file_path.clone()) {
            if let Ok(cutscene) = serde_yaml::from_str(&cutscene) {
                cutscene
            } else {
                error!(target: "Cutscene", "Failed to load: `{file_path}`");
                Self { frames: vec![] }
            }
        } else {
            error!(target: "Cutscene", "Failed to load: `{file_path}`");
            Self { frames: vec![] }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct FrameShot {
    /// WARNING: if you control the position of the player in the frame and give them control
    player_is_in_control: bool,
    /// - Some(name of entity) to impose the point of view of this entity
    /// - None to let the player have his own point of view during the cutscene
    ///
    /// ## Notes
    ///
    /// REFACTOR: how to store the pop star?
    /// for now the name of the entity.
    pop_star: Option<String>,
    camera_focus_type: CameraFocusType,
    dialogue: Option<String>,
    /// the position of all characters
    positions: Vec<(String, (f32, f32))>,
    duration: f32,
}

#[derive(Component)]
pub struct FrameTime {
    /// track when the frame is done
    pub timer: Timer,
}

#[derive(Component)]
pub struct CutsceneDestination(f32, f32, f32);

/* -------------------------------------------------------------------------- */
/*                                  Systems                                   */
/* -------------------------------------------------------------------------- */

/// TODO: cutscene - import specific cutscene for specific trigger
pub fn spawn_cutscene(mut commands: Commands) {
    commands.spawn((
        Name::new("Cutscene"),
        // Cutscene::new(),
        Cutscene::import_from_script("data/cutscene/open_temple_dos.yml".to_string()),
    ));

    // let _ = std::fs::write(
    //     "data/cutscene/open_temple.yml",
    //     serde_yaml::to_string(&Cutscene::new()).unwrap(),
    // );
}

pub fn run_cutscene(
    mut commands: Commands,
    mut cutscene_query: Query<(Entity, &mut Cutscene), Without<FrameTime>>,
    characters_query: Query<(Entity, &Name), With<Character>>,

    mut player_is_in_control_resource: ResMut<PlayerIsInControl>,
    current_play_mode: Res<State<PlayMode>>,
    mut next_play_mode: ResMut<NextState<PlayMode>>,
    // pos_query: Query<&GlobalTransform>,
) {
    if let Ok((cutscene_entity, mut cutscene)) = cutscene_query.get_single_mut() {
        if let Some(frame) = cutscene.frames.first() {
            info!(target: "Cutscene", "{frame:#?}");

            if *current_play_mode != PlayMode::InCinematic {
                next_play_mode.set(PlayMode::InCinematic);
                info!(target:"States", "PlayMode::InCinematic sended");
            }

            // define if the player can move freely
            player_is_in_control_resource.0 = frame.player_is_in_control;

            // define the pop star
            if let Some(pop_star_name) = &frame.pop_star {
                for (character, name) in characters_query.iter() {
                    if name.as_str().eq(pop_star_name) {
                        commands.entity(character).insert(CameraFocus::default());
                    } else {
                        commands.entity(character).remove::<CameraFocus>();
                    }
                }
            }

            // move each characters
            for (character_name, (x, y)) in frame.positions.iter() {
                let mut character_found = false;

                for (character, _name) in characters_query
                    .iter()
                    .filter(|(_, name)| name.as_str().eq(character_name))
                {
                    character_found = true;

                    commands
                        .entity(character)
                        .insert(CutsceneDestination(*x, *y, 0.));
                }

                if !character_found {
                    let all_names: String = characters_query
                        .iter()
                        .map(|(_, name)| name.to_string())
                        .reduce(|first, second| format!("{first}\n- {second}"))
                        .unwrap();
                    warn!(target: "Cutscene", "frame's position character not found: {character_name}\nall characters' name:\n{all_names}");
                }
            }

            commands.entity(cutscene_entity).insert(FrameTime {
                timer: Timer::from_seconds(frame.duration, TimerMode::Once),
            });
            cutscene.frames.remove(0);
        } else {
            // despawn the empty Cutscene
            commands.entity(cutscene_entity).despawn();

            if *current_play_mode != PlayMode::Improvisation {
                next_play_mode.set(PlayMode::Improvisation);
                info!(target:"States", "PlayMode::Improvisation sended");

                // NOTE: we could have created a system InEnter(PlayMode::Improvisation) to auto handle (but there is only this spot where we switch to Improvisation)
                player_is_in_control_resource.0 = true;
            }
        }
    }
}

/// Run the timer between the cutscene's frames.
/// Block the `run_cutscene` to pursue to the next frame.
pub fn frame_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut cutscene_query: Query<(Entity, &mut FrameTime)>,
) {
    if let Ok((cutscene, mut timer)) = cutscene_query.get_single_mut() {
        timer.timer.tick(time.delta());
        if timer.timer.finished() {
            commands.entity(cutscene).remove::<FrameTime>();
        }
    }
}

/// FIXME: character movement - the character don't have velocity put to 0. if not moving (they can be sent flying)
/// REFACTOR: duplicate with a lot of movement systems
pub fn cutscene_characters_movement(
    mut commands: Commands,
    mut characters_query: Query<
        (
            Entity,
            &Transform,
            &Speed,
            &mut Velocity,
            &CutsceneDestination,
            &Name,
        ),
        With<Character>,
    >,
) {
    for (character, transform, speed, mut rb_vel, destination, _name) in &mut characters_query {
        let (x, y): (f32, f32) = (destination.0, destination.1);
        let up = y as i32 > transform.translation.y as i32;
        let down = (y as i32) < transform.translation.y as i32;
        let left = (x as i32) < transform.translation.x as i32;
        let right = x as i32 > transform.translation.x as i32;

        // println!(
        //     "x: {} to {}, y: {} to {}",
        //     transform.translation.x as i32, x as i32, transform.translation.y as i32, y as i32
        // );

        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        // println!("x: {x_axis}, y: {y_axis}");

        let mut vel_x = x_axis as f32 * **speed;
        let mut vel_y = y_axis as f32 * **speed;

        if x_axis != 0 && y_axis != 0 {
            vel_x *= (std::f32::consts::PI / 4.).cos();
            vel_y *= (std::f32::consts::PI / 4.).cos();
        }
        // let (vel_x, vel_y) = move_to((x, y, 0.), false, transform, speed);

        rb_vel.linvel.x = vel_x;
        rb_vel.linvel.y = vel_y;

        if vel_x == 0. && vel_y == 0. {
            commands.entity(character).remove::<CutsceneDestination>();
        }
    }
}

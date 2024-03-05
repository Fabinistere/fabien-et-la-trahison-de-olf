//! All systems about Tactical Position and their transform

// TODO: Move Unit / Swap

use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};

use crate::{
    combat::{InCombat, Player, Recruted, TacticalPlace, TacticalPosition},
    constants::ui::{fighting_hall_position::*, FIGHTING_HALL_WIDTH},
};

/// Happens when:
/// - the window (size, etc) changed
/// - one of the TacticalPosition changed
///
/// Read in:
/// - tactical_position::update_character_position()
///   - Adapt OnScreenPosition for each InCombat character
#[derive(Event)]
pub struct UpdateCharacterPositionEvent;

/// Automatic detection to change tactical position render.
///
/// Detect Window change (size) or
/// Detect Change in the component TacticalPosition in an entity
///
/// At each change detected, will send an event to adapt OnScreenPosition
pub fn detect_window_tactical_pos_change(
    resize_event: Res<Events<WindowResized>>,
    characters_query: Query<Entity, (Changed<TacticalPosition>, With<InCombat>, With<Transform>)>,

    mut update_char_pos_event: EventWriter<UpdateCharacterPositionEvent>,
) {
    let mut reader = resize_event.get_reader();
    for _ in reader.iter(&resize_event) {
        info!("Window Resized");
        update_char_pos_event.send(UpdateCharacterPositionEvent);
    }
    for _ in characters_query.iter() {
        info!("Tactical Pos Change");
        update_char_pos_event.send(UpdateCharacterPositionEvent);
        break;
    }
}

/// Adapt transform depending their tactical position and the window size
///
/// # Note
///
/// FIXME: The window resize by automatic action (snap windows, etc) "spoil" the adaptation
pub fn update_character_position(
    mut update_char_pos_event: EventReader<UpdateCharacterPositionEvent>,

    window_query: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    camera_q: Query<(&Camera, &GlobalTransform)>,

    ally_query: Query<Or<(With<Recruted>, With<Player>)>>,
    mut characters_query: Query<(Entity, &mut Transform, &TacticalPosition, &Name), With<InCombat>>,
) {
    for _ in update_char_pos_event.iter() {
        let window = window_query.get_single().unwrap();
        // assuming there is exactly one main camera entity, so query::single() is OK
        let (camera, camera_transform) = camera_q.single();

        let width = window.width();
        let height = window.height();

        // 56% = width of the Fighting Hall
        // 17 = number of box / line
        // 2 = half of the box (to point to the center)
        let x = (width * (FIGHTING_HALL_WIDTH / 100.)) / 17.;
        let y = (height * 1.) / 17.;

        // info!("width: {}, x: {}", width, x);
        // info!("height: {}, y: {}", height, y);

        for (character, mut transform, tactical_position, _name) in characters_query.iter_mut() {
            // if recruted or player == Ally
            let (x_offset, y_offset) = if ally_query.contains(character) {
                match tactical_position {
                    TacticalPosition::FrontLine(place) => match place {
                        TacticalPlace::Left => ALLY_FRONTLINE_LEFT,
                        TacticalPlace::Middle => ALLY_FRONTLINE_MIDDLE,
                        TacticalPlace::Right => ALLY_FRONTLINE_RIGHT,
                    },
                    TacticalPosition::MiddleLine(place) => match place {
                        TacticalPlace::Left => ALLY_MIDDLELINE_LEFT,
                        TacticalPlace::Middle => ALLY_MIDDLELINE_MIDDLE,
                        TacticalPlace::Right => ALLY_MIDDLELINE_RIGHT,
                    },
                    TacticalPosition::BackLine(place) => match place {
                        TacticalPlace::Left => ALLY_BACKLINE_LEFT,
                        TacticalPlace::Middle => ALLY_BACKLINE_MIDDLE,
                        TacticalPlace::Right => ALLY_BACKLINE_RIGHT,
                    },
                }
            } else {
                match tactical_position {
                    TacticalPosition::FrontLine(place) => match place {
                        TacticalPlace::Left => ENEMY_FRONTLINE_LEFT,
                        TacticalPlace::Middle => ENEMY_FRONTLINE_MIDDLE,
                        TacticalPlace::Right => ENEMY_FRONTLINE_RIGHT,
                    },
                    TacticalPosition::MiddleLine(place) => match place {
                        TacticalPlace::Left => ENEMY_MIDDLELINE_LEFT,
                        TacticalPlace::Middle => ENEMY_MIDDLELINE_MIDDLE,
                        TacticalPlace::Right => ENEMY_MIDDLELINE_RIGHT,
                    },
                    TacticalPosition::BackLine(place) => match place {
                        TacticalPlace::Left => ENEMY_BACKLINE_LEFT,
                        TacticalPlace::Middle => ENEMY_BACKLINE_MIDDLE,
                        TacticalPlace::Right => ENEMY_BACKLINE_RIGHT,
                    },
                }
            };

            // info!("{}", _name);

            // info!("x_offset: {}, y_offset: {}", x_offset, y_offset);

            // To be in the center of the box = - (x,y / 2.)
            let window_coordinates = Vec2::new(x * x_offset - (x / 2.), y * y_offset - (y / 2.));
            // info!(
            //     "x_w: {}, y_w: {}",
            //     window_coordinates.0, window_coordinates.1
            // );

            let transform_coordinates = Some(window_coordinates)
                .and_then(|onscreen_position| {
                    camera.viewport_to_world(camera_transform, onscreen_position)
                })
                .map(|ray| ray.origin.truncate())
                .unwrap();
            // info!(
            //     "x_t: {}, y_t: {}",
            //     transform_coordinates.0, transform_coordinates.1
            // );

            transform.translation.x = transform_coordinates.x;
            // y axe is inverted
            transform.translation.y = -transform_coordinates.y;

            // info!("---------------");
        }
    }
}

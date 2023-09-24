use crate::{constants::TILE_SIZE, locations::landmarks::Direction};

use super::{FLOWER_PANELS_X, FLOWER_PANEL_Y, FLOWER_REPAIR_PANEL_Y};

pub const LANDMARK_SINGLETONS: [((f32, f32, f32), &str, Option<Direction>); 5] = [
    (LANDMARKS_CORRIDOR_FRONT, "Corridor Front", None),
    (LANDMARKS_EXIT_ROOM_TOP_LEFT, "Exit Top Left", None),
    (LANDMARKS_EXIT_ROOM_TOP_RIGHT, "Exit Room Top Right", None),
    (LANDMARKS_EXIT_ROOM_BOT_LEFT, "Exit Room Bot Left", None),
    (LANDMARKS_EXIT_ROOM_BOT_RIGHT, "Exit Room Bot Right", None),
];

/* -------------------------------------------------------------------------- */
/*                                Flower Panels                               */
/* -------------------------------------------------------------------------- */

pub const LANDMARK_FLOWER_PANEL_OFFSET: f32 = 20.;
pub const LANDMARKS_FLOWER_PANELS: [((f32, f32, f32), &str, Option<Direction>); 5] = [
    (
        (
            FLOWER_PANELS_X[0] * TILE_SIZE,
            (FLOWER_PANEL_Y - LANDMARK_FLOWER_PANEL_OFFSET) * TILE_SIZE,
            0.,
        ),
        "Far Left",
        None,
    ),
    (
        (
            FLOWER_PANELS_X[1] * TILE_SIZE,
            (FLOWER_PANEL_Y - LANDMARK_FLOWER_PANEL_OFFSET) * TILE_SIZE,
            0.,
        ),
        "Left",
        None,
    ),
    (
        (
            FLOWER_PANELS_X[2] * TILE_SIZE,
            (FLOWER_PANEL_Y - LANDMARK_FLOWER_PANEL_OFFSET) * TILE_SIZE,
            0.,
        ),
        "Right",
        None,
    ),
    (
        (
            FLOWER_PANELS_X[3] * TILE_SIZE,
            (FLOWER_PANEL_Y - LANDMARK_FLOWER_PANEL_OFFSET) * TILE_SIZE,
            0.,
        ),
        "Far Right",
        None,
    ),
    (
        (
            FLOWER_PANELS_X[4] * TILE_SIZE,
            (FLOWER_REPAIR_PANEL_Y - LANDMARK_FLOWER_PANEL_OFFSET) * TILE_SIZE,
            0.,
        ),
        "Repair",
        None,
    ),
];

pub const LANDMARKS_CORRIDOR_FRONT: (f32, f32, f32) = (-40. * TILE_SIZE, 156. * TILE_SIZE, 0.);
pub const LANDMARKS_EXIT_ROOM_TOP_LEFT: (f32, f32, f32) = (14. * TILE_SIZE, 154. * TILE_SIZE, 0.);
pub const LANDMARKS_EXIT_ROOM_TOP_RIGHT: (f32, f32, f32) = (75. * TILE_SIZE, 154. * TILE_SIZE, 0.);
pub const LANDMARKS_EXIT_ROOM_BOT_LEFT: (f32, f32, f32) = (14. * TILE_SIZE, 120. * TILE_SIZE, 0.);
pub const LANDMARKS_EXIT_ROOM_BOT_RIGHT: (f32, f32, f32) = (75. * TILE_SIZE, 120. * TILE_SIZE, 0.);

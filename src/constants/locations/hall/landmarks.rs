use crate::{constants::TILE_SIZE, locations::landmarks::Direction};

use super::{BOXES_X, BOXES_Y, STATUE_X, STATUE_Y};

pub const LANDMARK_GROUPS: [([((f32, f32, f32), &str, Option<Direction>); 3], &str); 2] = [
    (LANDMARK_STATUE, "Hall Statue"),
    (LANDMARK_BALCONY, "Balcony"),
];

pub const LANDMARK_SINGLETONS: [((f32, f32, f32), &str, Option<Direction>); 3] = [
    (LANDMARK_BOX_BEHIND, "Box Behind", None),
    (LANDMARK_BOX_RIGHT, "Box Right", None),
    (
        LANDMARK_TOP_LEFT_CORNER,
        "Top Left Corner",
        Some(Direction::Right),
    ),
];

/* -------------------------------------------------------------------------- */
/*                                   Groups                                   */
/* -------------------------------------------------------------------------- */

pub const LANDMARK_STATUE: [((f32, f32, f32), &str, Option<Direction>); 3] = [
    (
        (
            (STATUE_X - 10.) * TILE_SIZE,
            (STATUE_Y - 10.) * TILE_SIZE,
            0.,
        ),
        "Left",
        Some(Direction::Right),
    ),
    (
        (STATUE_X * TILE_SIZE, (STATUE_Y - 15.) * TILE_SIZE, 0.),
        "Middle",
        None,
    ),
    (
        (
            (STATUE_X + 10.) * TILE_SIZE,
            (STATUE_Y - 10.) * TILE_SIZE,
            0.,
        ),
        "Right",
        Some(Direction::Left),
    ),
];

pub const LANDMARK_BALCONY: [((f32, f32, f32), &str, Option<Direction>); 3] = [
    (
        (125. * TILE_SIZE, -157.5 * TILE_SIZE, 0.),
        "Balcony Up",
        Some(Direction::Right),
    ),
    (
        (122.5 * TILE_SIZE, -167.5 * TILE_SIZE, 0.),
        "Balcony Middle",
        Some(Direction::Right),
    ),
    (
        (115. * TILE_SIZE, -172.5 * TILE_SIZE, 0.),
        "Balcony Down",
        Some(Direction::Right),
    ),
];

/* -------------------------------------------------------------------------- */
/*                                 Singletons                                 */
/* -------------------------------------------------------------------------- */

pub const LANDMARK_BOX_BEHIND: (f32, f32, f32) =
    (BOXES_X * TILE_SIZE, (BOXES_Y + 10.) * TILE_SIZE, 0.);
pub const LANDMARK_BOX_RIGHT: (f32, f32, f32) =
    ((BOXES_X + 10.) * TILE_SIZE, BOXES_Y * TILE_SIZE, 0.);
pub const LANDMARK_TOP_LEFT_CORNER: (f32, f32, f32) =
    (BOXES_X * TILE_SIZE, (BOXES_Y + 40.) * TILE_SIZE, 0.);

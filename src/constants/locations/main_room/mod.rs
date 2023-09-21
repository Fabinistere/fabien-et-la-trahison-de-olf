pub mod landmarks;

use crate::constants::{interactions::INTERACT_BUTTON_Z, TILE_SIZE};

use super::{MAP_DISTANCE_IN_Z, MAP_START_Y, PROPS_Z, ROOF_Z, WILL_BE_COMPUTE_LATER, Y_UNIT};

pub const TEMPLE_EXIT_Y: f32 = 87.;
pub const MAIN_ROOM_Z: f32 = (TEMPLE_EXIT_Y - MAP_START_Y) * Y_UNIT - MAP_DISTANCE_IN_Z;

pub const TEMPLE_HALL_LOCATION_SENSOR_POSITION: (f32, f32, f32) = (-24., -94., 0.);
pub const TEMPLE_SECRET_LOCATION_SENSOR_POSITION: (f32, f32, f32) = (-44.5, 80., 0.);

pub const PILLAR_SWITCH_Z_OFFSET: f32 = 0.1;
pub const PILLAR_HITBOX_Y_OFFSET: f32 = -12.5;
pub const PILLAR_FIRST_COLUMN_X: f32 = -49.5;
pub const PILLAR_SECOND_COLUMN_X: f32 = 1.5;
pub const PILLAR_FIRST_LINE_Y: f32 = 22.5;
pub const PILLAR_SECOND_LINE_Y: f32 = -14.5;
pub const PILLAR_THIRD_LINE_Y: f32 = -54.5;
pub const PILLAR_POSITIONS: [(f32, f32, f32); 6] = [
    // 1    4
    // 2    5
    // 3    6
    // ----- 1 -----
    (
        PILLAR_FIRST_COLUMN_X * TILE_SIZE,
        PILLAR_FIRST_LINE_Y * TILE_SIZE,
        WILL_BE_COMPUTE_LATER,
    ),
    // ----- 2 -----
    (
        PILLAR_FIRST_COLUMN_X * TILE_SIZE,
        PILLAR_SECOND_LINE_Y * TILE_SIZE,
        WILL_BE_COMPUTE_LATER,
    ),
    // ----- 3 -----
    (
        PILLAR_FIRST_COLUMN_X * TILE_SIZE,
        PILLAR_THIRD_LINE_Y * TILE_SIZE,
        WILL_BE_COMPUTE_LATER,
    ),
    // ----- 4 -----
    (
        PILLAR_SECOND_COLUMN_X * TILE_SIZE,
        PILLAR_FIRST_LINE_Y * TILE_SIZE,
        WILL_BE_COMPUTE_LATER,
    ),
    // ----- 5 -----
    (
        PILLAR_SECOND_COLUMN_X * TILE_SIZE,
        PILLAR_SECOND_LINE_Y * TILE_SIZE,
        WILL_BE_COMPUTE_LATER,
    ),
    // ----- 6 -----
    (
        PILLAR_SECOND_COLUMN_X * TILE_SIZE,
        PILLAR_THIRD_LINE_Y * TILE_SIZE,
        WILL_BE_COMPUTE_LATER,
    ),
];

pub const BANNERS_POSITION: (f32, f32, f32) = (-20. * TILE_SIZE, 80. * TILE_SIZE, 0.);

pub const THRONE_SWITCH_Z_OFFSET: f32 = -0.1;
pub const THRONE_X: f32 = -24.;
pub const THRONE_Y: f32 = 71.5;
pub const THRONE_POSITION: (f32, f32, f32) = (
    THRONE_X * TILE_SIZE,
    THRONE_Y * TILE_SIZE,
    WILL_BE_COMPUTE_LATER,
);

const CHANDELIER_Z: f32 = ROOF_Z;
pub const TEMPLE_CHANDELIER_POSITIONS: [(f32, f32, f32); 4] = [
    (-77.5 * TILE_SIZE, 6. * TILE_SIZE, CHANDELIER_Z), // left top
    (-77.5 * TILE_SIZE, -40. * TILE_SIZE, CHANDELIER_Z), // left bottom
    (29.5 * TILE_SIZE, 6. * TILE_SIZE, CHANDELIER_Z),  // right top
    (29.5 * TILE_SIZE, -40. * TILE_SIZE, CHANDELIER_Z), // right bottom
];

pub const PLANTS_SWITCH_Z_OFFSET: f32 = 0.5;
pub const PLANTS_LEFT_SIDE_X: f32 = -125.5;
pub const PLANTS_RIGHT_SIDE_X: f32 = 77.5;
pub const PLANTS_FIRST_LINE_Y: f32 = 44.;
pub const PLANTS_SECOND_LINE_Y: f32 = -27.;
pub const PLANTS_POSITIONS: [(f32, f32, f32); 4] = [
    // TopLeft
    (
        PLANTS_LEFT_SIDE_X * TILE_SIZE,
        PLANTS_FIRST_LINE_Y * TILE_SIZE,
        WILL_BE_COMPUTE_LATER,
    ),
    // BottomLeft
    (
        PLANTS_LEFT_SIDE_X * TILE_SIZE,
        PLANTS_SECOND_LINE_Y * TILE_SIZE,
        WILL_BE_COMPUTE_LATER,
    ),
    // TopRight
    (
        PLANTS_RIGHT_SIDE_X * TILE_SIZE,
        PLANTS_FIRST_LINE_Y * TILE_SIZE,
        WILL_BE_COMPUTE_LATER,
    ),
    // BottomRight
    (
        PLANTS_RIGHT_SIDE_X * TILE_SIZE,
        PLANTS_SECOND_LINE_Y * TILE_SIZE,
        WILL_BE_COMPUTE_LATER,
    ),
];

pub const BRAZIER_Z_OFFSET: f32 = -0.1;
pub const BRAZIER_FLAME_OFFSET: (f32, f32, f32) = (0., 11.5, 0.);
pub const BRAZIERS_POSITIONS: [(f32, f32, f32); 4] = [
    (-116.5 * TILE_SIZE, 63.5 * TILE_SIZE, WILL_BE_COMPUTE_LATER), // LeftLeft
    (-83.5 * TILE_SIZE, 63.5 * TILE_SIZE, WILL_BE_COMPUTE_LATER),  // LeftRight
    (35.5 * TILE_SIZE, 63.5 * TILE_SIZE, WILL_BE_COMPUTE_LATER),   // RightLeft
    (68.5 * TILE_SIZE, 63.5 * TILE_SIZE, WILL_BE_COMPUTE_LATER),   // RightRight
];

pub const STATUE_Y: f32 = 75.;
pub const CAT_STATUE_X: f32 = -100.;
pub const CAT_STATUE_POSITION: (f32, f32, f32) = (
    CAT_STATUE_X * TILE_SIZE,
    STATUE_Y * TILE_SIZE,
    WILL_BE_COMPUTE_LATER,
);
pub const FABIEN_STATUE_X: f32 = 52.;
pub const FABIEN_STATUE_POSITION: (f32, f32, f32) = (
    FABIEN_STATUE_X * TILE_SIZE,
    STATUE_Y * TILE_SIZE,
    WILL_BE_COMPUTE_LATER,
);

pub const BANNER_INTERACTION_ID: u32 = 3;
pub const BANNER_INTERACT_BUTTON_POSITION: (f32, f32, f32) =
    (0. * TILE_SIZE, 0. * TILE_SIZE, INTERACT_BUTTON_Z);
pub const BANNER_POSITION: (f32, f32, f32) = (-44.5 * TILE_SIZE, 91. * TILE_SIZE, PROPS_Z);
pub const BANNER_SENSOR_OFFSET: (f32, f32, f32) = (0., 0., 0.);
pub const BANNER_COLLIDER_OFFSET: (f32, f32, f32) = (0., 0.5 * TILE_SIZE, 0.);
pub const BANNER_OPEN_DELTA_S: f32 = 0.1;

pub mod landmarks;

use super::{MAP_DISTANCE_IN_Z, MAP_END_Y, MAP_START_Y, PROPS_Z, WILL_BE_COMPUTE_LATER, Y_UNIT};
use crate::constants::TILE_SIZE;

pub const SECRET_ROOM_EXIT_Y: f32 = MAP_END_Y;
pub const SECRET_ROOM_Z: f32 = (SECRET_ROOM_EXIT_Y - MAP_START_Y) * Y_UNIT - MAP_DISTANCE_IN_Z;

pub const SECRET_LOCATION_SENSOR_POSITION: (f32, f32, f32) = (-44.5, SECRET_ROOM_TRIGGER_Y, 0.);

pub const SECRET_ROOM_TRIGGER_Y: f32 = 85.5;
pub const SECRET_ROOM_TRIGGER_CUBOID: (f32, f32) = (7., 5.);
pub const SECRET_ROOM_TRIGGER_POSITION: (f32, f32, f32) = (-44.5, SECRET_ROOM_TRIGGER_Y, 0.);
pub const SECRET_ROOM_COVER_POSITION: (f32, f32, f32) = (-24., 161., 6.9);
pub const SECRET_ROOM_COVER_SIZE: (f32, f32) = (250., 100.);

pub const SECOND_FAKE_WALL_SWITCH_Z_OFFSET: f32 = -2.4;

pub const FAKE_STONE_POSITION: (f32, f32, f32) = (0., 0., WILL_BE_COMPUTE_LATER);
pub const FAKE_STONE_SWITCH_Z_OFFSET: f32 = -2.5;

pub const FLOWER_PANEL_SWITCH_Z_OFFSET: f32 = 0.3;
pub const FLOWER_PANELS_X: [f32; 5] = [-116., -83., 35., 68., -105.5];
pub const FLOWER_PANEL_Y: f32 = 100.5;
pub const FLOWER_REPAIR_PANEL_Y: f32 = 165.5;
pub const FLOWER_PANEL_POSITIONS: [(f32, f32, f32); 5] = [
    // ----- 1 -----
    (
        FLOWER_PANELS_X[0] * TILE_SIZE,
        FLOWER_PANEL_Y * TILE_SIZE,
        WILL_BE_COMPUTE_LATER,
    ),
    // ----- 2 -----
    (
        FLOWER_PANELS_X[1] * TILE_SIZE,
        FLOWER_PANEL_Y * TILE_SIZE,
        WILL_BE_COMPUTE_LATER,
    ),
    // ----- 3 -----
    (
        FLOWER_PANELS_X[2] * TILE_SIZE,
        FLOWER_PANEL_Y * TILE_SIZE,
        WILL_BE_COMPUTE_LATER,
    ),
    // ----- 4 -----
    (
        FLOWER_PANELS_X[3] * TILE_SIZE,
        FLOWER_PANEL_Y * TILE_SIZE,
        WILL_BE_COMPUTE_LATER,
    ),
    // ----- Repair -----
    (
        FLOWER_PANELS_X[4] * TILE_SIZE,
        FLOWER_REPAIR_PANEL_Y * TILE_SIZE,
        WILL_BE_COMPUTE_LATER,
    ),
];

pub const WALL_POT_POSITION: (f32, f32, f32) = (-59.5 * TILE_SIZE, 171.5 * TILE_SIZE, PROPS_Z);

pub const STAIRS_RAMP_POSITION: (f32, f32, f32) =
    (-64.5 * TILE_SIZE, 141. * TILE_SIZE, WILL_BE_COMPUTE_LATER);
pub const STAIRS_RAMP_SWITCH_Z_OFFSET: f32 = -0.1;

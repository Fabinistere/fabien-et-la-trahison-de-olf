pub mod landmarks;

use super::{MAP_DISTANCE_IN_Z, MAP_START_Y, PROPS_Z, ROOF_Z, WILL_BE_COMPUTE_LATER, Y_UNIT};
use crate::constants::{interactions::INTERACT_BUTTON_Z, TILE_SIZE};

// HALL_END_Y
// FIXME: hair hang over the temple door (could be like the second_layer of fake wall)
pub const HALL_EXIT_Y: f32 = -89.; // -92.
pub const HALL_Z: f32 = (HALL_EXIT_Y - MAP_START_Y) * Y_UNIT - MAP_DISTANCE_IN_Z;
pub const UP_DOOR_Z: f32 = 10.;
pub const UP_DOOR_POSITION: (f32, f32, f32) = (0., 0., UP_DOOR_Z);
pub const BALCONY_Z: f32 = 3.;
pub const BALCONY_POSITION: (f32, f32, f32) = (0., 0., BALCONY_Z - HALL_Z);

pub const BALCONY_COVER_POSITION: (f32, f32, f32) = (114., -165., 0.1);
pub const BALCONY_COVER_SIZE: (f32, f32) = (60., 70.);

pub const HALL_FROM_TEMPLE_LOCATION_SENSOR_POSITION: (f32, f32, f32) = (-24., -105., 0.);
pub const HALL_FROM_BALCONY_LOCATION_SENSOR_POSITION: (f32, f32, f32) = (80., -160., 0.);
pub const BALCONY_LOCATION_SENSOR_SIZE: (f32, f32) = (13., 6.5);
pub const BALCONY_LOCATION_SENSOR_POSITION: (f32, f32, f32) = (95., -162.5, 0.);

pub const BOX_INTERACTION_ID: u32 = 0;
pub const BOXES_X: f32 = -121.5;
pub const BOXES_Y: f32 = -158.;
pub const BOX_POSITION: (f32, f32, f32) = (
    BOXES_X * TILE_SIZE,
    BOXES_Y * TILE_SIZE,
    WILL_BE_COMPUTE_LATER,
);
pub const BOX_SENSOR_OFFSET: (f32, f32, f32) = (0., -10. * TILE_SIZE, 0.);
pub const BOX_INTERACT_BUTTON_POSITION: (f32, f32, f32) =
    (12. * TILE_SIZE, 7. * TILE_SIZE, INTERACT_BUTTON_Z);

pub const DOOR_INTERACTION_ID: u32 = 1;
pub const DOOR_INTERACT_BUTTON_POSITION: (f32, f32, f32) =
    (17.5 * TILE_SIZE, -3.5 * TILE_SIZE, INTERACT_BUTTON_Z);
pub const DOOR_POSITION: (f32, f32, f32) =
    (-24. * TILE_SIZE, -88. * TILE_SIZE, WILL_BE_COMPUTE_LATER);
pub const DOOR_SENSOR_OFFSET: (f32, f32, f32) = (0., -10. * TILE_SIZE, 0.);
pub const DOOR_COLLIDER_OFFSET: (f32, f32, f32) = (0., -10. * TILE_SIZE, 0.);
pub const DOOR_OPEN_DELTA_S: f32 = 0.2;
pub const TEMPLE_DOOR_SWITCH_Z_OFFSET_CLOSED: f32 = 0.25;
pub const TEMPLE_DOOR_SWITCH_Z_OFFSET_OPENED: f32 = 0.3;

pub const STATUE_X: f32 = 59.;
pub const STATUE_Y: f32 = -100.;
pub const STATUE_POSITION: (f32, f32, f32) = (
    STATUE_X * TILE_SIZE,
    STATUE_Y * TILE_SIZE,
    WILL_BE_COMPUTE_LATER,
);
pub const STATUE_INTERACTION_ID: u32 = 2;
pub const STATUE_INTERACT_BUTTON_POSITION: (f32, f32, f32) =
    (-8.3 * TILE_SIZE, 0., INTERACT_BUTTON_Z);

const LIGHT_Z: f32 = ROOF_Z;
pub const HALL_CHANDELIER_POSITIONS: [(f32, f32, f32); 2] = [
    (-77.5 * TILE_SIZE, -125. * TILE_SIZE, LIGHT_Z), // left
    (29.5 * TILE_SIZE, -125. * TILE_SIZE, LIGHT_Z),  // right
];
pub const WALL_LIGHT_POSITIONS: [(f32, f32, f32); 4] = [
    (-87. * TILE_SIZE, -74. * TILE_SIZE, LIGHT_Z), // far left
    (-47. * TILE_SIZE, -74. * TILE_SIZE, LIGHT_Z), // center left
    (-2. * TILE_SIZE, -74. * TILE_SIZE, LIGHT_Z),  // center right
    (38. * TILE_SIZE, -74. * TILE_SIZE, LIGHT_Z),  // far right
];
pub const LIGHT_SUPPORT_OFFSET: (f32, f32, f32) = (0.5, -4.5, PROPS_Z);

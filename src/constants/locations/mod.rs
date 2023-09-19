pub mod hall;
pub mod main_room;
pub mod secret_room;

/*
| Layer              | Back Z, Front Z |
| ------------------ | --------------- |
| Character          |      depend     |
| Balcony            | 3.0,        --- |
| Balcony Doors      | 3.0,        11. |
| Hall               | 3.5,        8.0 |
| Hall Props         | 3.6,        8.1 |
| Hall Statue        | 3.6,        8.1 |
| Hall Door          | 3.6,        8.1 |
| Hall Chandeliers   | ---,        8.1 |
| Temple             | 2.0,        7.0 |
| Temple Plants      | 2.1,        7.1 |
| Temple Pillars     | 2.1,        7.1 |
| Temple Throne      | 2.1,        7.1 |
| Temple Statues     | 2.2,        7.2 |
| Temple Brazier     | 2.2,        7.2 |
| Temple Chandeliers | ---,        7.3 |
| Secret Room        | 1.0,        6.0 |
| Secret Room Panels | 1.1,        6.1 |
| Roof               | ---,        11. |
 */

pub const MAP_START_Y: f32 = -170.;
pub const MAP_END_Y: f32 = 165.;
pub const MAP_START_Z: f32 = 11.;
pub const MAP_END_Z: f32 = 1.;
pub const MAP_DISTANCE_IN_Y: f32 = MAP_END_Y - MAP_START_Y;
pub const MAP_DISTANCE_IN_Z: f32 = MAP_END_Z - MAP_START_Z;
// `Z_UNIT` is the equivalent of the y value of 1 z
pub const Z_UNIT: f32 = MAP_DISTANCE_IN_Y / MAP_DISTANCE_IN_Z;
// `Y_UNIT` is the equivalent of the z value of 1 y
pub const Y_UNIT: f32 = 1. / Z_UNIT;

// In `locations::temple::y_to_z_conversion`
pub const WILL_BE_COMPUTE_LATER: f32 = 0.;

pub const CHANDELIER_SIZE: (f32, f32) = (20., 10.);
pub const CHANDELIER_TRANSPARENCY_COLOR: f32 = 170. / 255.;
pub const CHANDELIER_PLAIN_COLOR: f32 = 1.;
pub const CHANDELIER_FLAME_POSITIONS: [(f32, f32, f32); 3] = [
    (-6.5, -2., PROPS_Z), // left
    (-0.5, -2., PROPS_Z), // center
    (5.5, -2., PROPS_Z),  // right
];

pub const GROUND_Z: f32 = 0.5;
pub const ROOF_Z: f32 = 11.;
// just enoguht to be above parent :) (smile from Horor Humanum Est <3)
pub const PROPS_Z: f32 = 0.01;

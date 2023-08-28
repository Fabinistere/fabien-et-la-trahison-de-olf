// dark purple #25131a = 39/255, 19/255, 26/255
pub const BACKGROUND_COLOR: bevy::render::color::Color =
    bevy::render::color::Color::rgb(0.153, 0.07, 0.102);
// pub const BACKGROUND_COLOR: bevy::render::color::Color = bevy::render::color::Color::Rgba {
//     red: 58. / 256.,
//     green: 36. / 256.,
//     blue: 48. / 256.,
//     alpha: 1.,
// };

pub const RESOLUTION: f32 = 16. / 9.;
pub const TILE_SIZE: f32 = 1.;

pub const FRAME_TIME: f32 = 0.1;

pub mod interactions {
    pub const INTERACT_BUTTON_Z: f32 = 20.;
    pub const INTERACT_BUTTON_SCALE: f32 = 0.25;
}

pub mod ui {
    pub mod dialogs {
        pub const DIALOG_BOX_ANIMATION_OFFSET: f32 = -1000.;
        pub const DIALOG_BOX_UPDATE_DELTA_S: f32 = 0.05;
        pub const DIALOG_BOX_ANIMATION_TIME_MS: u64 = 500;
        pub const SCROLL_SIZE: (f32, f32) = (490., 11700. / 45.);
        pub const SCROLL_ANIMATION_DELTA_S: f32 = 0.1;
        pub const SCROLL_ANIMATION_FRAMES_NUMBER: usize = 45;
    }
}

pub mod character {
    use super::TILE_SIZE;

    pub const CHAR_SCALE: f32 = 0.6 * TILE_SIZE;

    pub const CHAR_HITBOX_HEIGHT: f32 = 1.5 * CHAR_SCALE;
    pub const CHAR_HITBOX_WIDTH: f32 = 5. * CHAR_SCALE;
    pub const CHAR_HITBOX_Y_OFFSET: f32 = -6.25;
    pub const CHAR_SENSOR_Y_OFFSET: f32 = -1.25;

    pub mod player {
        use crate::animations::sprite_sheet_animation::CharacterState;

        pub const PLAYER_WIDTH: f32 = 12.;
        pub const PLAYER_HEIGHT: f32 = 15.;
        pub const PLAYER_SCALE: f32 = super::CHAR_SCALE;
        pub const PLAYER_Z: f32 = 5.;
        pub const PLAYER_SPAWN: (f32, f32, f32) = (-24., -150., PLAYER_Z);

        pub const CAMERA_INTERPOLATION: f32 = 0.1;

        /* -------------------------------------------------------------------------- */
        /*                                  Animation                                 */
        /* -------------------------------------------------------------------------- */

        // (start_frame, end_frame, next_state)
        pub const PLAYER_RUN_FRAMES: (usize, usize, CharacterState) = (6, 9, CharacterState::Idle);
        pub const PLAYER_IDLE_FRAMES: (usize, usize, CharacterState) =
            (10, 11, CharacterState::Idle);
    }
}

pub mod locations {

    /*
    | Layer              | Back Z, Front Z |
    | ------------------ | --------------- |
    | Player             |       5.0       |
    | Balcony            | 3.0,        --- |
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
     */

    // We encapsulate all props/objects in the parent room
    // It herits its parent's transform
    pub const PROPS_Z_BACK: f32 = 0.1;
    // pub const PROPS_Z_FRONT: f32 = PLAYER_Z + PROPS_Z_BACK;

    pub const CHANDELIER_SIZE: (f32, f32) = (20., 10.);
    pub const CHANDELIER_TRANSPARENCY_COLOR: f32 = 170. / 255.;
    pub const CHANDELIER_PLAIN_COLOR: f32 = 1.;
    pub const CHANDELIER_FLAME_POSITIONS: [(f32, f32, f32); 3] = [
        (-6.5, -2., 0.1), // left
        (-0.5, -2., 0.1), // center
        (5.5, -2., 0.1),  // right
    ];

    pub const GROUND_Z: f32 = 0.5;

    pub mod hall {
        use crate::constants::{
            character::player::PLAYER_Z, interactions::INTERACT_BUTTON_Z, TILE_SIZE,
        };

        use super::PROPS_Z_BACK;

        pub const HALL_Z: f32 = 3.5;
        pub const HALL_Z_IN_MAIN_ROOM: f32 = 8.;
        pub const UP_DOOR_Z: f32 = PLAYER_Z;
        pub const UP_DOOR_POSITION: (f32, f32, f32) = (0., 0., UP_DOOR_Z);
        pub const BALCONY_Z: f32 = 3.;
        pub const BALCONY_POSITION: (f32, f32, f32) = (0., 0., BALCONY_Z - HALL_Z);

        pub const BALCONY_COVER_POSITION: (f32, f32, f32) = (114., -165., 0.1);
        pub const BALCONY_COVER_SIZE: (f32, f32) = (60., 70.);

        pub const HALL_FROM_TEMPLE_LOCATION_SENSOR_POSITION: (f32, f32, f32) = (-24., -105., 0.);
        pub const HALL_FROM_BALCONY_LOCATION_SENSOR_POSITION: (f32, f32, f32) = (80., -160., 0.);
        pub const BALCONY_LOCATION_SENSOR_SIZE: (f32, f32) = (13., 6.5);
        pub const BALCONY_LOCATION_SENSOR_POSITION: (f32, f32, f32) = (95., -162.5, 0.);

        pub const PROPS_INTERACTION_ID: u32 = 0;
        pub const PROPS_POSITION: (f32, f32, f32) =
            (-121.5 * TILE_SIZE, -158. * TILE_SIZE, PROPS_Z_BACK);
        pub const BOX_SENSOR_OFFSET: (f32, f32, f32) = (0., -10. * TILE_SIZE, 0.);
        pub const BOX_INTERACT_BUTTON_POSITION: (f32, f32, f32) =
            (12. * TILE_SIZE, 7. * TILE_SIZE, INTERACT_BUTTON_Z);

        pub const DOOR_INTERACTION_ID: u32 = 1;
        pub const DOOR_INTERACT_BUTTON_POSITION: (f32, f32, f32) =
            (17.5 * TILE_SIZE, -3.5 * TILE_SIZE, INTERACT_BUTTON_Z);
        pub const DOOR_POSITION: (f32, f32, f32) =
            (-24. * TILE_SIZE, -88. * TILE_SIZE, PROPS_Z_BACK);
        pub const DOOR_SENSOR_OFFSET: (f32, f32, f32) = (0., -10. * TILE_SIZE, 0.);
        pub const DOOR_COLLIDER_OFFSET: (f32, f32, f32) = (0., -10. * TILE_SIZE, 0.);
        pub const DOOR_OPEN_DELTA_S: f32 = 0.2;

        pub const STATUE_POSITION: (f32, f32, f32) = (59., -101., PROPS_Z_BACK);
        pub const STATUE_INTERACTION_ID: u32 = 2;
        pub const STATUE_INTERACT_BUTTON_POSITION: (f32, f32, f32) =
            (-8.3 * TILE_SIZE, 0., INTERACT_BUTTON_Z);

        const LIGHT_Z: f32 = HALL_Z_IN_MAIN_ROOM + PROPS_Z_BACK;
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
        pub const LIGHT_SUPPORT_OFFSET: (f32, f32, f32) = (0.5, -4.5, 0.);
    }

    pub mod main_room {
        use crate::constants::{interactions::INTERACT_BUTTON_Z, TILE_SIZE};

        use super::PROPS_Z_BACK;

        // REFACTOR: Change this threshold by a sensorin the Temple Door
        pub const MAIN_ROOM_ENTER_Y: f32 = -91.;

        pub const MAIN_ROOM_Z: f32 = 2.;
        pub const MAIN_ROOM_Z_WHEN_IN_SECRET_ROOM: f32 = 7.;

        pub const TEMPLE_HALL_LOCATION_SENSOR_POSITION: (f32, f32, f32) = (-24., -94., 0.);
        pub const TEMPLE_SECRET_LOCATION_SENSOR_POSITION: (f32, f32, f32) = (-44.5, 80., 0.);

        pub const PILLAR_SWITCH_Z_OFFSET: f32 = 5.;
        pub const PILLAR_HITBOX_Y_OFFSET: f32 = -12.5;
        pub const PILLAR_POSITIONS: [(f32, f32, f32); 6] = [
            // 1    4
            // 2    5
            // 3    6
            (-49.5 * TILE_SIZE, 25.5 * TILE_SIZE, PROPS_Z_BACK), // 1
            (-49.5 * TILE_SIZE, -14.5 * TILE_SIZE, PROPS_Z_BACK), // 2
            (-49.5 * TILE_SIZE, -54.5 * TILE_SIZE, PROPS_Z_BACK), // 3
            (1.5 * TILE_SIZE, 25.5 * TILE_SIZE, PROPS_Z_BACK),   // 4
            (1.5 * TILE_SIZE, -14.5 * TILE_SIZE, PROPS_Z_BACK),  // 5
            (1.5 * TILE_SIZE, -54.5 * TILE_SIZE, PROPS_Z_BACK),  // 6
        ];

        pub const BANNERS_POSITION: (f32, f32, f32) =
            (-20. * TILE_SIZE, 80. * TILE_SIZE, PROPS_Z_BACK);

        pub const THRONE_SWITCH_Z_OFFSET: f32 = -3.5;
        pub const THRONE_POSITION: (f32, f32, f32) =
            (-24. * TILE_SIZE, 71.5 * TILE_SIZE, PROPS_Z_BACK);

        const CHANDELIER_Z: f32 = 7.1;
        pub const TEMPLE_CHANDELIER_POSITIONS: [(f32, f32, f32); 4] = [
            (-77.5 * TILE_SIZE, 6. * TILE_SIZE, CHANDELIER_Z), // left top
            (-77.5 * TILE_SIZE, -40. * TILE_SIZE, CHANDELIER_Z), // left bottom
            (29.5 * TILE_SIZE, 6. * TILE_SIZE, CHANDELIER_Z),  // right top
            (29.5 * TILE_SIZE, -40. * TILE_SIZE, CHANDELIER_Z), // right bottom
        ];

        pub const PLANTS_SWITCH_Z_OFFSET: f32 = 29.;
        pub const PLANTS_POSITIONS: [(f32, f32, f32); 4] = [
            (-125.5 * TILE_SIZE, 44. * TILE_SIZE, PROPS_Z_BACK), // TopLeft
            (-125.5 * TILE_SIZE, -27. * TILE_SIZE, PROPS_Z_BACK), // BottomLeft
            (77.5 * TILE_SIZE, 44. * TILE_SIZE, PROPS_Z_BACK),   // TopRight
            (77.5 * TILE_SIZE, -27. * TILE_SIZE, PROPS_Z_BACK),  // BottomRight
        ];

        pub const BRAZIER_FLAME_OFFSET: (f32, f32, f32) = (0., 11.5, 0.1);
        pub const BRAZIERS_POSITIONS: [(f32, f32, f32); 4] = [
            (-116.5 * TILE_SIZE, 63.5 * TILE_SIZE, PROPS_Z_BACK), // LeftLeft
            (-83.5 * TILE_SIZE, 63.5 * TILE_SIZE, PROPS_Z_BACK),  // LeftRight
            (35.5 * TILE_SIZE, 63.5 * TILE_SIZE, PROPS_Z_BACK),   // RightLeft
            (68.5 * TILE_SIZE, 63.5 * TILE_SIZE, PROPS_Z_BACK),   // RightRight
        ];

        pub const STATUE_SWITCH_Z_OFFSET: f32 = 3.;
        pub const CAT_STATUE_POSITION: (f32, f32, f32) = (-100., 75., PROPS_Z_BACK);
        pub const FABIEN_STATUE_POSITION: (f32, f32, f32) = (52., 77., PROPS_Z_BACK);

        pub const BANNER_INTERACTION_ID: u32 = 3;
        pub const BANNER_INTERACT_BUTTON_POSITION: (f32, f32, f32) =
            (0. * TILE_SIZE, 0. * TILE_SIZE, INTERACT_BUTTON_Z);
        pub const BANNER_POSITION: (f32, f32, f32) =
            (-44.5 * TILE_SIZE, 91. * TILE_SIZE, PROPS_Z_BACK);
        pub const BANNER_SENSOR_OFFSET: (f32, f32, f32) = (0., 0., 0.);
        pub const BANNER_COLLIDER_OFFSET: (f32, f32, f32) = (0., 0.5 * TILE_SIZE, 0.);
        pub const BANNER_OPEN_DELTA_S: f32 = 0.1;
    }

    pub mod secret_room {
        use crate::constants::TILE_SIZE;

        use super::PROPS_Z_BACK;

        pub const SECRET_ROOM_Z: f32 = 1.;
        pub const SECRET_ROOM_Z_WHEN_OUTSIDE: f32 = 6.;

        pub const SECRET_LOCATION_SENSOR_POSITION: (f32, f32, f32) =
            (-44.5, SECRET_ROOM_TRIGGER_Y, 0.);

        pub const SECRET_ROOM_TRIGGER_Y: f32 = 85.5;
        pub const SECRET_ROOM_TRIGGER_CUBOID: (f32, f32) = (7., 5.);
        pub const SECRET_ROOM_TRIGGER_POSITION: (f32, f32, f32) =
            (-44.5, SECRET_ROOM_TRIGGER_Y, 0.);
        pub const SECRET_ROOM_COVER_POSITION: (f32, f32, f32) =
            (-24., 160., SECRET_ROOM_Z_WHEN_OUTSIDE + 0.9);
        pub const SECRET_ROOM_COVER_SIZE: (f32, f32) = (250., 100.);

        pub const FAKE_STONE_POSITION: (f32, f32, f32) = (0., 0., PROPS_Z_BACK);
        pub const FAKE_STONE_SWITCH_Z_OFFSET: f32 = -80.;

        pub const FLOWER_PANEL_SWITCH_Z_OFFSET: f32 = 13.;
        pub const FLOWER_PANEL_POSITIONS: [(f32, f32, f32); 5] = [
            (-116. * TILE_SIZE, 100.5 * TILE_SIZE, PROPS_Z_BACK), // 1
            (-83. * TILE_SIZE, 100.5 * TILE_SIZE, PROPS_Z_BACK),  // 2
            (35. * TILE_SIZE, 100.5 * TILE_SIZE, PROPS_Z_BACK),   // 3
            (68. * TILE_SIZE, 100.5 * TILE_SIZE, PROPS_Z_BACK),   // 4
            (-105.5 * TILE_SIZE, 165.5 * TILE_SIZE, PROPS_Z_BACK), // Repair
        ];

        // TODO: when outside change the wall_pot Z to `SECRET_ROOM_Z_WHEN_OUTSIDE`
        pub const WALL_POT_POSITION: (f32, f32, f32) =
            (-59.5 * TILE_SIZE, 171.5 * TILE_SIZE, SECRET_ROOM_Z);

        pub const CAT_SWITCH_Z_OFFSET: f32 = 0.;
        pub const OLF_CAT_Z: f32 = PROPS_Z_BACK;
        pub const OLF_CAT_SCALE: f32 = 0.5;
        pub const OLF_CAT_ANIMATION_DELTA: f32 = 0.5;
        pub const OLF_CAT_POSITION: (f32, f32, f32) = (-104., 134., OLF_CAT_Z);
        pub const OLF_CAT_HITBOX_OFFSET: (f32, f32, f32) = (0., -1.75, 0.);
    }
}

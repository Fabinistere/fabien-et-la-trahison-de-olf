// dark blue #213757 = 33/255, 55/255, 87/255
pub const BACKGROUND_COLOR_INMENU: bevy::render::color::Color =
    bevy::render::color::Color::rgb(33. / 255., 55. / 255., 87. / 255.);
// dark purple #25131a = 39/255, 19/255, 26/255
pub const BACKGROUND_COLOR_INGAME: bevy::render::color::Color =
    bevy::render::color::Color::rgb(0.153, 0.07, 0.102);
// pub const BACKGROUND_COLOR: bevy::render::color::Color = bevy::render::color::Color::Rgba {
//     red: 58. / 256.,
//     green: 36. / 256.,
//     blue: 48. / 256.,
//     alpha: 1.,
// };

pub const RESOLUTION: f32 = 9. / 16.; // 16. / 9.;
pub const TILE_SIZE: f32 = 1.;

pub const FRAME_TIME: f32 = 0.1;

pub mod title_screen {

    /* -------------------------------------------------------------------------- */
    /*                                   Lights                                   */
    /* -------------------------------------------------------------------------- */
    pub const FULL_LIGHTS_INDEX: (usize, usize) = (0, 0);
    pub const BOT_SHUTDOWN_INDEX: (usize, usize) = (1, 1);
    pub const TOP_SHUTDOWN_INDEX: (usize, usize) = (2, 2);
    pub const TOWER_RESET_INDEX: (usize, usize) = (3, 3);
    pub const SMALL_SHUTDOWN_INDEX: (usize, usize) = (4, 12);
    pub const LEFT_SHUTDOWN_INDEX: (usize, usize) = (13, 20);

    pub const MANOR_LIGHTS_PATTERN_INDEXES: &[(usize, usize); 6] = &[
        FULL_LIGHTS_INDEX,
        TOWER_RESET_INDEX,
        SMALL_SHUTDOWN_INDEX,
        TOP_SHUTDOWN_INDEX,
        BOT_SHUTDOWN_INDEX,
        LEFT_SHUTDOWN_INDEX,
    ];

    /* -------------------------------------------------------------------------- */
    /*                                    Title                                   */
    /* -------------------------------------------------------------------------- */

    pub const TITLE_FLEX_BOT_DELTA_S: u64 = 2;
    // stay twice more time in the top position
    pub const TITLE_FLEX_TOP_DELTA_S: u64 = TITLE_FLEX_BOT_DELTA_S * 2;
    pub const TITLE_FLEX_TOP: f32 = 0.;
    pub const TITLE_FLEX_BOT: f32 = -5.;
}

pub mod interactions {
    pub const INTERACT_BUTTON_Z: f32 = 20.;
    pub const INTERACT_BUTTON_SCALE: f32 = 0.25;

    // REFACTOR: INTERACTION_ID
}

pub mod ui {
    pub mod dialogs {
        use bevy::prelude::Color;

        pub const DIALOG_PANEL_ANIMATION_OFFSET: f32 = -1000.;
        pub const DIALOG_BOX_UPDATE_DELTA_S: f32 = 0.05;
        pub const DIALOG_PANEL_ANIMATION_TIME_MS: u64 = 500;
        pub const SCROLL_SIZE: (f32, f32) = (490., 11700. / 45.);
        pub const SCROLL_ANIMATION_DELTA_S: f32 = 0.1;
        pub const SCROLL_ANIMATION_FRAMES_NUMBER: usize = 45;

        pub const TRANSPARENT_BUTTON: Color = Color::rgba(0., 0., 0., 0.);
        // pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
        pub const NORMAL_BUTTON: Color = Color::rgba(0.1, 0.1, 0.1, 0.1);
        pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
        pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
    }
}

pub mod character {
    pub const CHAR_SCALE: f32 = 0.6 * super::TILE_SIZE;

    pub const CHAR_HITBOX_HEIGHT: f32 = 1.5 * CHAR_SCALE;
    pub const CHAR_HITBOX_WIDTH: f32 = 5. * CHAR_SCALE;
    pub const CHAR_HITBOX_Y_OFFSET: f32 = -6.25;
    pub const CHAR_SENSOR_Y_OFFSET: f32 = -1.25;

    pub const COLUMN_FRAME_RUN_END: usize = 3;
    pub const COLUMN_FRAME_IDLE_START: usize = 4;
    pub const COLUMN_FRAME_IDLE_END: usize = 5;

    pub const SPRITESHEET_COLUMN_NUMBER: usize = 6;

    pub mod player {
        use crate::animations::sprite_sheet_animation::CharacterState;

        use super::{
            COLUMN_FRAME_IDLE_END, COLUMN_FRAME_IDLE_START, COLUMN_FRAME_RUN_END,
            SPRITESHEET_COLUMN_NUMBER,
        };

        pub const PLAYER_WIDTH: f32 = 12.;
        pub const PLAYER_HEIGHT: f32 = 15.;
        pub const PLAYER_SCALE: f32 = super::CHAR_SCALE;
        pub const PLAYER_SPAWN: (f32, f32, f32) = (-24., -150., 0.);

        pub const CAMERA_INTERPOLATION: f32 = 0.1;

        /* -------------------------------------------------------------------------- */
        /*                                  Animation                                 */
        /* -------------------------------------------------------------------------- */

        pub const PLAYER_LINE_START: usize = 1 * SPRITESHEET_COLUMN_NUMBER;
        // (start_frame, end_frame, next_state)
        pub const PLAYER_RUN_FRAMES: (usize, usize, CharacterState) = (
            PLAYER_LINE_START,
            PLAYER_LINE_START + COLUMN_FRAME_RUN_END,
            CharacterState::Idle,
        );
        pub const PLAYER_IDLE_FRAMES: (usize, usize, CharacterState) = (
            PLAYER_LINE_START + COLUMN_FRAME_IDLE_START,
            PLAYER_LINE_START + COLUMN_FRAME_IDLE_END,
            CharacterState::Idle,
        );
    }

    pub mod npc {
        use crate::constants::{
            interactions::INTERACT_BUTTON_Z, locations::main_room::THRONE_POSITION, TILE_SIZE,
        };

        pub const NPC_SCALE: f32 = super::CHAR_SCALE;

        pub const SUPREME_GOD_SPAWN_POSITION: (f32, f32, f32) = THRONE_POSITION;
        pub const SUPREME_GOD_INTERACTION_ID: u32 = 10;
        pub const SUPREME_GOD_INTERACT_BUTTON_POSITION: (f32, f32, f32) =
            (15. * TILE_SIZE, 10. * TILE_SIZE, INTERACT_BUTTON_Z);

        /* -------------------------------------------------------------------------- */
        /*                                  Animation                                 */
        /* -------------------------------------------------------------------------- */

        pub const ADMIRAL_LINE: usize = 0;
        pub const FABIEN_LOYAL_LINE: usize = 2;
        pub const FABIEN_DISLOYAL_LINE: usize = 3;
        pub const OLF_LINE: usize = 4;
        pub const OLF_GHOST_LINE: usize = 5;
        pub const FOOL_LINE: usize = 6;
        pub const SUPREME_GOD_LINE: usize = 7;
        pub const GENERAL_LINE: usize = 8;
        pub const HEALER_V1_LINE: usize = 9;
        pub const HEALER_V2_LINE: usize = 10;
        pub const FABICURION_LINE: usize = 11;
        pub const VAMPIRE_LINE: usize = 12;
        pub const AGENT_LINE: usize = 13;
        pub const BLACK_CAT_LINE: usize = 14;
        pub const BLUE_CAT_LINE: usize = 15;

        pub const CAT_SWITCH_Z_OFFSET: f32 = 0.;
        pub const OLF_CAT_SCALE: f32 = 0.5;
        pub const OLF_CAT_ANIMATION_DELTA: f32 = 0.5;
        pub const OLF_CAT_POSITION: (f32, f32, f32) = (-104., 134., 0.);
        pub const OLF_CAT_HITBOX_OFFSET: (f32, f32, f32) = (0., -5., 0.);
    }

    pub mod dialog {
        // Flibittygibbit

        // TODO: feature - Read at dialog_file instead of CST
        // CST = path to the file

        pub const RANDOM_DIALOG: &str = "1:
  source: Fabien
  content:
    text:
      - Enfant, j'ai eu un poney
      - Mais j'ai toujours voulu un agneau
    exit_state: 2\n";

        pub const OLF_DIALOG: &str = "1:
  source: Olf
  content:
    text:
      - Il faut absolument sauver les Fabien du Chien Geant
    exit_state: 2
2:
  source: Player
  content:
    - text: ...
      condition: null
      exit_state: 3
3:
  source: Olf
  content:
    text:
      - Il me faut donc obtenir le trone
    exit_state: 4
4:
  source: Player
  content:
    - text: ...
      condition: null
      exit_state: 5
    - text: et de l'$
      condition: null
      exit_state: 6
5:
  source: Olf
  content:
    text:
      - Et de l'$
    exit_state: 6
6:
  source: Olf
  content:
    text:
      - C'est essentiel
    exit_state: 7\n";

        pub const FABIEN_DIALOG: &str = "1:
  source: Fabien
  content:
    text:
      - Hello
      - <3
    exit_state: 2
2:
  source: Player
  content:
    - text: Hey
      condition: null
      exit_state: 3
    - text: No Hello
      condition: null
      exit_state: 4
    - text: Want to share a flat ?
      condition: null
      exit_state: 5
3:
  source: Fabien
  content:
    text:
      - :)
    exit_state: 6
4:
  source: Fabien
  content:
    text:
      - :O
    exit_state: 6
5:
  source: Fabien
  content:
    text:
      - Sure
    exit_state: 6\n";

        pub const MORGAN_DIALOG: &str = "1:
  source: Player
  content:
    text:
      - Bonjour Flo.
      - Comment vas-tu ?
      - J'ai faim.
    exit_state: 2\n";
    }
}

pub mod locations {

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

    pub mod hall {
        use crate::constants::{interactions::INTERACT_BUTTON_Z, TILE_SIZE};

        use super::{
            MAP_DISTANCE_IN_Z, MAP_START_Y, PROPS_Z, ROOF_Z, WILL_BE_COMPUTE_LATER, Y_UNIT,
        };

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
        pub const BOX_POSITION: (f32, f32, f32) =
            (-121.5 * TILE_SIZE, -158. * TILE_SIZE, WILL_BE_COMPUTE_LATER);
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

        pub const STATUE_POSITION: (f32, f32, f32) = (59., -101., WILL_BE_COMPUTE_LATER);
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
    }

    pub mod main_room {
        use crate::constants::{interactions::INTERACT_BUTTON_Z, TILE_SIZE};

        use super::{
            MAP_DISTANCE_IN_Z, MAP_START_Y, PROPS_Z, ROOF_Z, WILL_BE_COMPUTE_LATER, Y_UNIT,
        };

        pub const TEMPLE_EXIT_Y: f32 = 87.;
        pub const MAIN_ROOM_Z: f32 = (TEMPLE_EXIT_Y - MAP_START_Y) * Y_UNIT - MAP_DISTANCE_IN_Z;

        pub const TEMPLE_HALL_LOCATION_SENSOR_POSITION: (f32, f32, f32) = (-24., -94., 0.);
        pub const TEMPLE_SECRET_LOCATION_SENSOR_POSITION: (f32, f32, f32) = (-44.5, 80., 0.);

        pub const PILLAR_SWITCH_Z_OFFSET: f32 = 0.1;
        pub const PILLAR_HITBOX_Y_OFFSET: f32 = -12.5;
        pub const PILLAR_POSITIONS: [(f32, f32, f32); 6] = [
            // 1    4
            // 2    5
            // 3    6
            (-49.5 * TILE_SIZE, 25.5 * TILE_SIZE, WILL_BE_COMPUTE_LATER), // 1
            (-49.5 * TILE_SIZE, -14.5 * TILE_SIZE, WILL_BE_COMPUTE_LATER), // 2
            (-49.5 * TILE_SIZE, -54.5 * TILE_SIZE, WILL_BE_COMPUTE_LATER), // 3
            (1.5 * TILE_SIZE, 25.5 * TILE_SIZE, WILL_BE_COMPUTE_LATER),   // 4
            (1.5 * TILE_SIZE, -14.5 * TILE_SIZE, WILL_BE_COMPUTE_LATER),  // 5
            (1.5 * TILE_SIZE, -54.5 * TILE_SIZE, WILL_BE_COMPUTE_LATER),  // 6
        ];

        pub const BANNERS_POSITION: (f32, f32, f32) = (-20. * TILE_SIZE, 80. * TILE_SIZE, 0.);

        pub const THRONE_SWITCH_Z_OFFSET: f32 = -0.1;
        pub const THRONE_POSITION: (f32, f32, f32) =
            (-24. * TILE_SIZE, 71.5 * TILE_SIZE, WILL_BE_COMPUTE_LATER);

        const CHANDELIER_Z: f32 = ROOF_Z;
        pub const TEMPLE_CHANDELIER_POSITIONS: [(f32, f32, f32); 4] = [
            (-77.5 * TILE_SIZE, 6. * TILE_SIZE, CHANDELIER_Z), // left top
            (-77.5 * TILE_SIZE, -40. * TILE_SIZE, CHANDELIER_Z), // left bottom
            (29.5 * TILE_SIZE, 6. * TILE_SIZE, CHANDELIER_Z),  // right top
            (29.5 * TILE_SIZE, -40. * TILE_SIZE, CHANDELIER_Z), // right bottom
        ];

        pub const PLANTS_SWITCH_Z_OFFSET: f32 = 0.5;
        pub const PLANTS_POSITIONS: [(f32, f32, f32); 4] = [
            (-125.5 * TILE_SIZE, 44. * TILE_SIZE, WILL_BE_COMPUTE_LATER), // TopLeft
            (-125.5 * TILE_SIZE, -27. * TILE_SIZE, WILL_BE_COMPUTE_LATER), // BottomLeft
            (77.5 * TILE_SIZE, 44. * TILE_SIZE, WILL_BE_COMPUTE_LATER),   // TopRight
            (77.5 * TILE_SIZE, -27. * TILE_SIZE, WILL_BE_COMPUTE_LATER),  // BottomRight
        ];

        pub const BRAZIER_Z_OFFSET: f32 = -0.1;
        pub const BRAZIER_FLAME_OFFSET: (f32, f32, f32) = (0., 11.5, 0.);
        pub const BRAZIERS_POSITIONS: [(f32, f32, f32); 4] = [
            (-116.5 * TILE_SIZE, 63.5 * TILE_SIZE, WILL_BE_COMPUTE_LATER), // LeftLeft
            (-83.5 * TILE_SIZE, 63.5 * TILE_SIZE, WILL_BE_COMPUTE_LATER),  // LeftRight
            (35.5 * TILE_SIZE, 63.5 * TILE_SIZE, WILL_BE_COMPUTE_LATER),   // RightLeft
            (68.5 * TILE_SIZE, 63.5 * TILE_SIZE, WILL_BE_COMPUTE_LATER),   // RightRight
        ];

        // pub const STATUE_SWITCH_Z_OFFSET: f32 = 0.;
        pub const CAT_STATUE_POSITION: (f32, f32, f32) = (-100., 75., WILL_BE_COMPUTE_LATER);
        pub const FABIEN_STATUE_POSITION: (f32, f32, f32) = (52., 77., WILL_BE_COMPUTE_LATER);

        pub const BANNER_INTERACTION_ID: u32 = 3;
        pub const BANNER_INTERACT_BUTTON_POSITION: (f32, f32, f32) =
            (0. * TILE_SIZE, 0. * TILE_SIZE, INTERACT_BUTTON_Z);
        pub const BANNER_POSITION: (f32, f32, f32) = (-44.5 * TILE_SIZE, 91. * TILE_SIZE, PROPS_Z);
        pub const BANNER_SENSOR_OFFSET: (f32, f32, f32) = (0., 0., 0.);
        pub const BANNER_COLLIDER_OFFSET: (f32, f32, f32) = (0., 0.5 * TILE_SIZE, 0.);
        pub const BANNER_OPEN_DELTA_S: f32 = 0.1;
    }

    pub mod secret_room {
        use crate::constants::TILE_SIZE;

        use super::{
            MAP_DISTANCE_IN_Z, MAP_END_Y, MAP_START_Y, PROPS_Z, WILL_BE_COMPUTE_LATER, Y_UNIT,
        };

        pub const SECRET_ROOM_EXIT_Y: f32 = MAP_END_Y;
        pub const SECRET_ROOM_Z: f32 =
            (SECRET_ROOM_EXIT_Y - MAP_START_Y) * Y_UNIT - MAP_DISTANCE_IN_Z;

        pub const SECRET_LOCATION_SENSOR_POSITION: (f32, f32, f32) =
            (-44.5, SECRET_ROOM_TRIGGER_Y, 0.);

        pub const SECRET_ROOM_TRIGGER_Y: f32 = 85.5;
        pub const SECRET_ROOM_TRIGGER_CUBOID: (f32, f32) = (7., 5.);
        pub const SECRET_ROOM_TRIGGER_POSITION: (f32, f32, f32) =
            (-44.5, SECRET_ROOM_TRIGGER_Y, 0.);
        pub const SECRET_ROOM_COVER_POSITION: (f32, f32, f32) = (-24., 161., 6.9);
        pub const SECRET_ROOM_COVER_SIZE: (f32, f32) = (250., 100.);

        pub const SECOND_FAKE_WALL_SWITCH_Z_OFFSET: f32 = -2.4;

        pub const FAKE_STONE_POSITION: (f32, f32, f32) = (0., 0., WILL_BE_COMPUTE_LATER);
        pub const FAKE_STONE_SWITCH_Z_OFFSET: f32 = -2.5;

        pub const FLOWER_PANEL_SWITCH_Z_OFFSET: f32 = 0.3;
        pub const FLOWER_PANEL_POSITIONS: [(f32, f32, f32); 5] = [
            (-116. * TILE_SIZE, 100.5 * TILE_SIZE, WILL_BE_COMPUTE_LATER), // 1
            (-83. * TILE_SIZE, 100.5 * TILE_SIZE, WILL_BE_COMPUTE_LATER),  // 2
            (35. * TILE_SIZE, 100.5 * TILE_SIZE, WILL_BE_COMPUTE_LATER),   // 3
            (68. * TILE_SIZE, 100.5 * TILE_SIZE, WILL_BE_COMPUTE_LATER),   // 4
            (-105.5 * TILE_SIZE, 165.5 * TILE_SIZE, WILL_BE_COMPUTE_LATER), // Repair
        ];

        pub const WALL_POT_POSITION: (f32, f32, f32) =
            (-59.5 * TILE_SIZE, 171.5 * TILE_SIZE, PROPS_Z);

        pub const STAIRS_RAMP_POSITION: (f32, f32, f32) =
            (-64.5 * TILE_SIZE, 141. * TILE_SIZE, WILL_BE_COMPUTE_LATER);
        pub const STAIRS_RAMP_SWITCH_Z_OFFSET: f32 = -0.1;
    }
}

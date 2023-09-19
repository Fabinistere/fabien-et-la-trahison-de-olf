pub const CHAR_SCALE: f32 = 0.6 * super::TILE_SIZE;

pub const CHAR_HITBOX_HEIGHT: f32 = 1.5 * CHAR_SCALE;
pub const CHAR_HITBOX_WIDTH: f32 = 5. * CHAR_SCALE;
pub const CHAR_HITBOX_Y_OFFSET: f32 = -6.25;
pub const CHAR_SENSOR_Y_OFFSET: f32 = -1.25;

pub const COLUMN_FRAME_RUN_END: usize = 3;
pub const COLUMN_FRAME_IDLE_START: usize = 4;
pub const COLUMN_FRAME_IDLE_END: usize = 5;

pub const SPRITESHEET_LINE_NUMBER: usize = 16;
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

    pub const PLAYER_LINE: usize = 1;
    pub const PLAYER_LINE_START: usize = PLAYER_LINE * SPRITESHEET_COLUMN_NUMBER;
    // (start_frame, end_frame, next_state)
    pub const PLAYER_RUN_FRAMES: (usize, usize, CharacterState) = (
        PLAYER_LINE_START,
        PLAYER_LINE_START + COLUMN_FRAME_RUN_END,
        // CharacterState::Idle,
        CharacterState::Run,
    );
    pub const PLAYER_IDLE_FRAMES: (usize, usize, CharacterState) = (
        PLAYER_LINE_START + COLUMN_FRAME_IDLE_START,
        PLAYER_LINE_START + COLUMN_FRAME_IDLE_END,
        CharacterState::Idle,
    );
}

pub mod npcs {
    use crate::constants::{
        interactions::INTERACT_BUTTON_Z,
        locations::main_room::{CAT_STATUE_POSITION, THRONE_POSITION},
        TILE_SIZE,
    };

    pub const NPC_SCALE: f32 = super::CHAR_SCALE;

    pub const CHARACTER_INTERACT_BUTTON_POSITION: (f32, f32, f32) =
        (15. * TILE_SIZE, 10. * TILE_SIZE, INTERACT_BUTTON_Z);

    pub const OLF_CAT_SCALE: f32 = 0.5;
    pub const OLF_CAT_POSITION: (f32, f32, f32) = (-104., 134., 0.);
    pub const SUPREME_GOD_SPAWN_POSITION: (f32, f32, f32) = THRONE_POSITION;
    pub const OLF_SPAWN_POSITION: (f32, f32, f32) = OLF_CAT_POSITION;
    pub const VAMPIRE_SPAWN_POSITION: (f32, f32, f32) = CAT_STATUE_POSITION;

    pub const NPC_TALK_INTERACTION_ID: u32 = 10;

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
    pub const OLF_CAT_ANIMATION_DELTA: f32 = 0.5;
    pub const OLF_CAT_HITBOX_OFFSET: (f32, f32, f32) = (0., -5., 0.);

    pub mod movement {
        use crate::constants::TILE_SIZE;

        pub const REST_TIMER: u64 = 3;
        // TODO: adjust EVASION_TIMER / FAIR_PLAY_TIMER
        pub const EVASION_TIMER: u64 = 5;

        pub const NPC_SPEED_LEADER: f32 = 70. * TILE_SIZE;
        pub const NPC_SPEED: f32 = 50. * TILE_SIZE; // -> Speed::default()
    }
}

pub mod dialog {
    // Flibittygibbit

    pub const RANDOM_DIALOG: &str = "1:
source: Fabien
content:
text:
  - Enfant, j'ai eu un poney
  - Mais j'ai toujours voulu un agneau
exit_state: 2\n";
}

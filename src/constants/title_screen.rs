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

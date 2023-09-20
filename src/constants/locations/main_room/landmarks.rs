use super::*;
use crate::{constants::TILE_SIZE, locations::landmarks::Direction};

pub const LANDMARK_SENSOR_SIZE: f32 = 2.5;

pub const LANDMARK_GROUPS: [([((f32, f32, f32), &str, Option<Direction>); 3], &str); 8] = [
    (LANDMARK_CAT_STATUE, "Cat Statue"),
    (LANDMARK_FABIEN_STATUE, "Fabien Statue"),
    (LANDMARK_PLANTS_TOP_LEFT, "Plants Top Left"),
    (
        LANDMARK_PLANTS_BOTTOM_LEFT_UPPER_SIDE,
        "Plants Bot Left UpperPart",
    ),
    (
        LANDMARK_PLANTS_BOTTOM_LEFT_LOWER_SIDE,
        "Plants Bot Left LowerPart",
    ),
    (LANDMARK_PLANTS_TOP_RIGHT, "Plants Top Right"),
    (
        LANDMARK_PLANTS_BOTTOM_RIGHT_UPPER_SIDE,
        "Plants Bot Right UpperPart",
    ),
    (
        LANDMARK_PLANTS_BOTTOM_RIGHT_LOWER_SIDE,
        "Plants Bot Right LowerPart",
    ),
];

pub const LANDMARK_SINGLETONS: [((f32, f32, f32), &str, Option<Direction>); 8] = [
    (LANDMARK_THRONE_SIT, "Throne Sit", None),
    (LANDMARK_THRONE_LEFT_SIDE, "Throne Left Side", None),
    (LANDMARK_THRONE_RIGHT_SIDE, "Throne Right Side", None),
    (LANDMARK_STAIRS_LEFT, "Stairs Left", None),
    (LANDMARK_STAIRS_RIGHT, "Stairs Right", None),
    (LANDMARK_AUDIENCE, "Audience", None),
    (
        LANDMARK_CAT_STATUE_BEHIND,
        "Behind Cat Statue",
        Some(Direction::Right),
    ),
    (
        LANDMARK_FABIEN_STATUE_BEHIND,
        "Behind Fabien Statue",
        Some(Direction::Right),
    ),
];

/* -------------------------------------------------------------------------- */
/*                                   Statues                                  */
/* -------------------------------------------------------------------------- */

pub const LANDMARK_CAT_STATUE: [((f32, f32, f32), &str, Option<Direction>); 3] = [
    (
        (
            CAT_STATUE_X - 10. * TILE_SIZE,
            STATUE_Y - 25. * TILE_SIZE,
            0.,
        ),
        "Left",
        Some(Direction::Right),
    ),
    (
        (CAT_STATUE_X * TILE_SIZE, (STATUE_Y - 35.) * TILE_SIZE, 0.),
        "Middle",
        None,
    ),
    (
        (
            (CAT_STATUE_X + 10.) * TILE_SIZE,
            (STATUE_Y - 25.) * TILE_SIZE,
            0.,
        ),
        "Right",
        Some(Direction::Left),
    ),
];

pub const LANDMARK_CAT_STATUE_BEHIND: (f32, f32, f32) = (
    CAT_STATUE_X + 10. * TILE_SIZE,
    (STATUE_Y - 5.) * TILE_SIZE,
    0.,
);

pub const LANDMARK_FABIEN_STATUE: [((f32, f32, f32), &str, Option<Direction>); 3] = [
    (
        (
            (FABIEN_STATUE_X - 10.) * TILE_SIZE,
            (STATUE_Y - 25.) * TILE_SIZE,
            0.,
        ),
        "Left",
        Some(Direction::Right),
    ),
    (
        (
            FABIEN_STATUE_X * TILE_SIZE,
            (STATUE_Y - 35.) * TILE_SIZE,
            0.,
        ),
        "Middle",
        None,
    ),
    (
        (
            FABIEN_STATUE_X + 10. * TILE_SIZE,
            STATUE_Y - 25. * TILE_SIZE,
            0.,
        ),
        "Right",
        Some(Direction::Left),
    ),
];

pub const LANDMARK_FABIEN_STATUE_BEHIND: (f32, f32, f32) = (
    (FABIEN_STATUE_X - 10.) * TILE_SIZE,
    (STATUE_Y - 5.) * TILE_SIZE,
    0.,
);

/* -------------------------------------------------------------------------- */
/*                                   Plants                                   */
/* -------------------------------------------------------------------------- */

// ----- Left -----

pub const LANDMARK_PLANTS_TOP_LEFT: [((f32, f32, f32), &str, Option<Direction>); 3] = [
    // ----- Bot Side -----
    (
        (
            (PLANTS_LEFT_SIDE_X + 15.) * TILE_SIZE,
            (PLANTS_FIRST_LINE_Y - 5.) * TILE_SIZE,
            0.,
        ),
        "TL Up",
        None,
    ),
    (
        (
            (PLANTS_LEFT_SIDE_X + 20.) * TILE_SIZE,
            (PLANTS_FIRST_LINE_Y - 15.) * TILE_SIZE,
            0.,
        ),
        "TL Middle Right",
        Some(Direction::Left),
    ),
    (
        (
            (PLANTS_LEFT_SIDE_X + 15.) * TILE_SIZE,
            (PLANTS_FIRST_LINE_Y - 25.) * TILE_SIZE,
            0.,
        ),
        "TL Down",
        None,
    ),
];

pub const LANDMARK_PLANTS_BOTTOM_LEFT_UPPER_SIDE: [((f32, f32, f32), &str, Option<Direction>); 3] = [
    (
        (
            (PLANTS_LEFT_SIDE_X + 15.) * TILE_SIZE,
            (PLANTS_SECOND_LINE_Y + 5.) * TILE_SIZE,
            0.,
        ),
        "BL Up",
        None,
    ),
    (
        (
            (PLANTS_LEFT_SIDE_X + 20.) * TILE_SIZE,
            (PLANTS_SECOND_LINE_Y + 15.) * TILE_SIZE,
            0.,
        ),
        "BL Middle Right",
        Some(Direction::Left),
    ),
    (
        (
            (PLANTS_LEFT_SIDE_X + 15.) * TILE_SIZE,
            (PLANTS_SECOND_LINE_Y + 25.) * TILE_SIZE,
            0.,
        ),
        "BL Down",
        None,
    ),
];

pub const LANDMARK_PLANTS_BOTTOM_LEFT_LOWER_SIDE: [((f32, f32, f32), &str, Option<Direction>); 3] = [
    (
        (
            (PLANTS_LEFT_SIDE_X + 15.) * TILE_SIZE,
            (PLANTS_SECOND_LINE_Y - 5.) * TILE_SIZE,
            0.,
        ),
        "BL Up",
        None,
    ),
    (
        (
            (PLANTS_LEFT_SIDE_X + 20.) * TILE_SIZE,
            (PLANTS_SECOND_LINE_Y - 15.) * TILE_SIZE,
            0.,
        ),
        "BL Middle Right",
        Some(Direction::Left),
    ),
    (
        (
            (PLANTS_LEFT_SIDE_X + 15.) * TILE_SIZE,
            (PLANTS_SECOND_LINE_Y - 25.) * TILE_SIZE,
            0.,
        ),
        "BL Down",
        None,
    ),
];

// ----- Right -----

pub const LANDMARK_PLANTS_TOP_RIGHT: [((f32, f32, f32), &str, Option<Direction>); 3] = [
    // ----- Bot Side -----
    (
        (
            (PLANTS_RIGHT_SIDE_X - 15.) * TILE_SIZE,
            (PLANTS_FIRST_LINE_Y - 5.) * TILE_SIZE,
            0.,
        ),
        "TR Up",
        None,
    ),
    (
        (
            (PLANTS_RIGHT_SIDE_X - 20.) * TILE_SIZE,
            (PLANTS_FIRST_LINE_Y - 15.) * TILE_SIZE,
            0.,
        ),
        "TR Middle Left",
        Some(Direction::Right),
    ),
    (
        (
            (PLANTS_RIGHT_SIDE_X - 15.) * TILE_SIZE,
            (PLANTS_FIRST_LINE_Y - 25.) * TILE_SIZE,
            0.,
        ),
        "TR Down",
        None,
    ),
];

pub const LANDMARK_PLANTS_BOTTOM_RIGHT_UPPER_SIDE: [((f32, f32, f32), &str, Option<Direction>); 3] = [
    (
        (
            (PLANTS_RIGHT_SIDE_X - 15.) * TILE_SIZE,
            (PLANTS_SECOND_LINE_Y + 5.) * TILE_SIZE,
            0.,
        ),
        "BR Up",
        None,
    ),
    (
        (
            (PLANTS_RIGHT_SIDE_X - 20.) * TILE_SIZE,
            (PLANTS_SECOND_LINE_Y + 15.) * TILE_SIZE,
            0.,
        ),
        "BR Middle Left",
        Some(Direction::Right),
    ),
    (
        (
            (PLANTS_RIGHT_SIDE_X - 15.) * TILE_SIZE,
            (PLANTS_SECOND_LINE_Y + 25.) * TILE_SIZE,
            0.,
        ),
        "BR Down",
        None,
    ),
];

pub const LANDMARK_PLANTS_BOTTOM_RIGHT_LOWER_SIDE: [((f32, f32, f32), &str, Option<Direction>); 3] = [
    (
        (
            (PLANTS_RIGHT_SIDE_X - 15.) * TILE_SIZE,
            (PLANTS_SECOND_LINE_Y - 5.) * TILE_SIZE,
            0.,
        ),
        "BR Up",
        None,
    ),
    (
        (
            (PLANTS_RIGHT_SIDE_X - 20.) * TILE_SIZE,
            (PLANTS_SECOND_LINE_Y - 15.) * TILE_SIZE,
            0.,
        ),
        "BR Middle Left",
        Some(Direction::Right),
    ),
    (
        (
            (PLANTS_RIGHT_SIDE_X - 15.) * TILE_SIZE,
            (PLANTS_SECOND_LINE_Y - 25.) * TILE_SIZE,
            0.,
        ),
        "BR Down",
        None,
    ),
];

/* -------------------------------------------------------------------------- */
/*                                  Audience                                  */
/* -------------------------------------------------------------------------- */

pub const LANDMARK_AUDIENCE: (f32, f32, f32) =
    (THRONE_X * TILE_SIZE, (THRONE_Y - 50.) * TILE_SIZE, 0.);

/* -------------------------------------------------------------------------- */
/*                              Throne and Sides                              */
/* -------------------------------------------------------------------------- */

pub const LANDMARK_THRONE_SIT: (f32, f32, f32) =
    (THRONE_X * TILE_SIZE, (THRONE_Y - 6.) * TILE_SIZE, 0.);

pub const LANDMARK_THRONE_RIGHT_SIDE: (f32, f32, f32) = (
    (THRONE_X - 20.) * TILE_SIZE,
    (THRONE_Y - 10.) * TILE_SIZE,
    0.,
);
pub const LANDMARK_THRONE_LEFT_SIDE: (f32, f32, f32) = (
    (THRONE_X + 20.) * TILE_SIZE,
    (THRONE_Y - 10.) * TILE_SIZE,
    0.,
);

pub const LANDMARK_STAIRS_RIGHT: (f32, f32, f32) = (
    (THRONE_X - 35.) * TILE_SIZE,
    (THRONE_Y - 30.) * TILE_SIZE,
    0.,
);
pub const LANDMARK_STAIRS_LEFT: (f32, f32, f32) = (
    (THRONE_X + 35.) * TILE_SIZE,
    (THRONE_Y - 30.) * TILE_SIZE,
    0.,
);

/* -------------------------------------------------------------------------- */
/*                                   Pillars                                  */
/* -------------------------------------------------------------------------- */

pub const OUTER_X_OFFSET: f32 = 10.;
pub const INNER_X_OFFSET: f32 = 10.;
pub const INNER_Y_OFFSET: f32 = 5.;

// each pillar has two landmarks
pub const LANDMARK_PILLARS: [([((f32, f32, f32), &str, Option<Direction>); 2], &str); 6] = [
    (
        [
            (
                (
                    (PILLAR_FIRST_COLUMN_X - OUTER_X_OFFSET) * TILE_SIZE,
                    PILLAR_FIRST_LINE_Y * TILE_SIZE,
                    0.,
                ),
                "Outer Side",
                Some(Direction::Right),
            ),
            (
                (
                    (PILLAR_FIRST_COLUMN_X + INNER_X_OFFSET) * TILE_SIZE,
                    (PILLAR_FIRST_LINE_Y - INNER_Y_OFFSET) * TILE_SIZE,
                    0.,
                ),
                "Inner Side",
                Some(Direction::Right),
            ),
        ],
        "First Pillar",
    ),
    (
        [
            (
                (
                    (PILLAR_FIRST_COLUMN_X - OUTER_X_OFFSET) * TILE_SIZE,
                    PILLAR_SECOND_LINE_Y * TILE_SIZE,
                    0.,
                ),
                "Outer Side",
                Some(Direction::Right),
            ),
            (
                (
                    (PILLAR_FIRST_COLUMN_X + INNER_X_OFFSET) * TILE_SIZE,
                    (PILLAR_SECOND_LINE_Y - INNER_Y_OFFSET) * TILE_SIZE,
                    0.,
                ),
                "Inner Side",
                Some(Direction::Right),
            ),
        ],
        "Second Pillar",
    ),
    (
        [
            (
                (
                    (PILLAR_FIRST_COLUMN_X - OUTER_X_OFFSET) * TILE_SIZE,
                    PILLAR_THIRD_LINE_Y * TILE_SIZE,
                    0.,
                ),
                "Outer Side",
                Some(Direction::Right),
            ),
            (
                (
                    (PILLAR_FIRST_COLUMN_X + INNER_X_OFFSET) * TILE_SIZE,
                    (PILLAR_THIRD_LINE_Y - INNER_Y_OFFSET) * TILE_SIZE,
                    0.,
                ),
                "Inner Side",
                Some(Direction::Right),
            ),
        ],
        "Third Pillar",
    ),
    (
        [
            (
                (
                    (PILLAR_SECOND_COLUMN_X + OUTER_X_OFFSET) * TILE_SIZE,
                    PILLAR_FIRST_LINE_Y * TILE_SIZE,
                    0.,
                ),
                "Outer Side",
                Some(Direction::Left),
            ),
            (
                (
                    (PILLAR_SECOND_COLUMN_X - INNER_X_OFFSET) * TILE_SIZE,
                    (PILLAR_FIRST_LINE_Y - INNER_Y_OFFSET) * TILE_SIZE,
                    0.,
                ),
                "Inner Side",
                Some(Direction::Left),
            ),
        ],
        "Fourth Pillar",
    ),
    (
        [
            (
                (
                    (PILLAR_SECOND_COLUMN_X + OUTER_X_OFFSET) * TILE_SIZE,
                    PILLAR_SECOND_LINE_Y * TILE_SIZE,
                    0.,
                ),
                "Outer Side",
                Some(Direction::Left),
            ),
            (
                (
                    (PILLAR_SECOND_COLUMN_X - INNER_X_OFFSET) * TILE_SIZE,
                    (PILLAR_SECOND_LINE_Y - INNER_Y_OFFSET) * TILE_SIZE,
                    0.,
                ),
                "Inner Side",
                Some(Direction::Left),
            ),
        ],
        "Fifth Pillar",
    ),
    (
        [
            (
                (
                    (PILLAR_SECOND_COLUMN_X + OUTER_X_OFFSET) * TILE_SIZE,
                    PILLAR_THIRD_LINE_Y * TILE_SIZE,
                    0.,
                ),
                "Outer Side",
                Some(Direction::Left),
            ),
            (
                (
                    (PILLAR_SECOND_COLUMN_X - INNER_X_OFFSET) * TILE_SIZE,
                    (PILLAR_THIRD_LINE_Y - INNER_Y_OFFSET) * TILE_SIZE,
                    0.,
                ),
                "Inner Side",
                Some(Direction::Left),
            ),
        ],
        "Sixth Pillar",
    ),
];

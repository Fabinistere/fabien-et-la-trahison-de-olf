use super::*;
use crate::constants::TILE_SIZE;

pub const LANDMARK_SENSOR_SIZE: f32 = 2.5;

pub const LANDMARK_GROUPS: [([((f32, f32, f32), &str); 3], &str); 8] = [
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

pub const LANDMARK_SINGLETONS: [((f32, f32, f32), &str); 8] = [
    (LANDMARK_THRONE_SIT, "Throne Sit"),
    (LANDMARK_THRONE_LEFT_SIDE, "Throne Left Side"),
    (LANDMARK_THRONE_RIGHT_SIDE, "Throne Right Side"),
    (LANDMARK_STAIRS_LEFT, "Stairs Left"),
    (LANDMARK_STAIRS_RIGHT, "Stairs Right"),
    (LANDMARK_AUDIENCE, "Audience"),
    (LANDMARK_CAT_STATUE_BEHIND, "Behind Cat Statue"),
    (LANDMARK_FABIEN_STATUE_BEHIND, "Behind Fabien Statue"),
];

/* -------------------------------------------------------------------------- */
/*                                   Statues                                  */
/* -------------------------------------------------------------------------- */

pub const LANDMARK_CAT_STATUE: [((f32, f32, f32), &str); 3] = [
    (
        (
            CAT_STATUE_X + 10. * TILE_SIZE,
            STATUE_Y - 25. * TILE_SIZE,
            0.,
        ),
        "Right",
    ),
    (
        (CAT_STATUE_X * TILE_SIZE, STATUE_Y - 35. * TILE_SIZE, 0.),
        "Middle",
    ),
    (
        (
            CAT_STATUE_X - 10. * TILE_SIZE,
            STATUE_Y - 25. * TILE_SIZE,
            0.,
        ),
        "Left",
    ),
];

pub const LANDMARK_CAT_STATUE_BEHIND: (f32, f32, f32) = (
    CAT_STATUE_X + 5. * TILE_SIZE,
    STATUE_Y + 25. * TILE_SIZE,
    0.,
);

pub const LANDMARK_FABIEN_STATUE: [((f32, f32, f32), &str); 3] = [
    (
        (
            FABIEN_STATUE_X + 10. * TILE_SIZE,
            STATUE_Y - 25. * TILE_SIZE,
            0.,
        ),
        "Right",
    ),
    (
        (FABIEN_STATUE_X * TILE_SIZE, STATUE_Y - 35. * TILE_SIZE, 0.),
        "Middle",
    ),
    (
        (
            FABIEN_STATUE_X - 10. * TILE_SIZE,
            STATUE_Y - 25. * TILE_SIZE,
            0.,
        ),
        "Left",
    ),
];

pub const LANDMARK_FABIEN_STATUE_BEHIND: (f32, f32, f32) = (
    FABIEN_STATUE_X + 5. * TILE_SIZE,
    STATUE_Y + 25. * TILE_SIZE,
    0.,
);

/* -------------------------------------------------------------------------- */
/*                                   Plants                                   */
/* -------------------------------------------------------------------------- */

// ----- Left -----

pub const LANDMARK_PLANTS_TOP_LEFT: [((f32, f32, f32), &str); 3] = [
    // ----- Bot Side -----
    (
        (
            PLANTS_LEFT_SIDE_X + 15. * TILE_SIZE,
            PLANTS_FIRST_LINE_Y - 5. * TILE_SIZE,
            0.,
        ),
        "TL Up",
    ),
    (
        (
            PLANTS_LEFT_SIDE_X + 20. * TILE_SIZE,
            PLANTS_FIRST_LINE_Y - 15. * TILE_SIZE,
            0.,
        ),
        "TL Middle Right",
    ),
    (
        (
            PLANTS_LEFT_SIDE_X + 15. * TILE_SIZE,
            PLANTS_FIRST_LINE_Y - 25. * TILE_SIZE,
            0.,
        ),
        "TL Down",
    ),
];

pub const LANDMARK_PLANTS_BOTTOM_LEFT_UPPER_SIDE: [((f32, f32, f32), &str); 3] = [
    (
        (
            PLANTS_LEFT_SIDE_X + 15. * TILE_SIZE,
            PLANTS_SECOND_LINE_Y + 5. * TILE_SIZE,
            0.,
        ),
        "BL Up",
    ),
    (
        (
            PLANTS_LEFT_SIDE_X + 20. * TILE_SIZE,
            PLANTS_SECOND_LINE_Y + 15. * TILE_SIZE,
            0.,
        ),
        "BL Middle Right",
    ),
    (
        (
            PLANTS_LEFT_SIDE_X + 15. * TILE_SIZE,
            PLANTS_SECOND_LINE_Y + 25. * TILE_SIZE,
            0.,
        ),
        "BL Down",
    ),
];

pub const LANDMARK_PLANTS_BOTTOM_LEFT_LOWER_SIDE: [((f32, f32, f32), &str); 3] = [
    (
        (
            PLANTS_LEFT_SIDE_X + 15. * TILE_SIZE,
            PLANTS_SECOND_LINE_Y - 5. * TILE_SIZE,
            0.,
        ),
        "BL Up",
    ),
    (
        (
            PLANTS_LEFT_SIDE_X + 20. * TILE_SIZE,
            PLANTS_SECOND_LINE_Y - 15. * TILE_SIZE,
            0.,
        ),
        "BL Middle Right",
    ),
    (
        (
            PLANTS_LEFT_SIDE_X + 15. * TILE_SIZE,
            PLANTS_SECOND_LINE_Y - 25. * TILE_SIZE,
            0.,
        ),
        "BL Down",
    ),
];

// ----- Right -----

pub const LANDMARK_PLANTS_TOP_RIGHT: [((f32, f32, f32), &str); 3] = [
    // ----- Bot Side -----
    (
        (
            PLANTS_RIGHT_SIDE_X - 15. * TILE_SIZE,
            PLANTS_FIRST_LINE_Y - 5. * TILE_SIZE,
            0.,
        ),
        "TR Up",
    ),
    (
        (
            PLANTS_RIGHT_SIDE_X - 20. * TILE_SIZE,
            PLANTS_FIRST_LINE_Y - 15. * TILE_SIZE,
            0.,
        ),
        "TR Middle Right",
    ),
    (
        (
            PLANTS_RIGHT_SIDE_X - 15. * TILE_SIZE,
            PLANTS_FIRST_LINE_Y - 25. * TILE_SIZE,
            0.,
        ),
        "TR Down",
    ),
];

pub const LANDMARK_PLANTS_BOTTOM_RIGHT_UPPER_SIDE: [((f32, f32, f32), &str); 3] = [
    (
        (
            PLANTS_RIGHT_SIDE_X - 15. * TILE_SIZE,
            PLANTS_SECOND_LINE_Y + 5. * TILE_SIZE,
            0.,
        ),
        "BR Up",
    ),
    (
        (
            PLANTS_RIGHT_SIDE_X - 20. * TILE_SIZE,
            PLANTS_SECOND_LINE_Y + 15. * TILE_SIZE,
            0.,
        ),
        "BR Middle Right",
    ),
    (
        (
            PLANTS_RIGHT_SIDE_X - 15. * TILE_SIZE,
            PLANTS_SECOND_LINE_Y + 25. * TILE_SIZE,
            0.,
        ),
        "BR Down",
    ),
];

pub const LANDMARK_PLANTS_BOTTOM_RIGHT_LOWER_SIDE: [((f32, f32, f32), &str); 3] = [
    (
        (
            PLANTS_RIGHT_SIDE_X - 15. * TILE_SIZE,
            PLANTS_SECOND_LINE_Y - 5. * TILE_SIZE,
            0.,
        ),
        "BR Up",
    ),
    (
        (
            PLANTS_RIGHT_SIDE_X - 20. * TILE_SIZE,
            PLANTS_SECOND_LINE_Y - 15. * TILE_SIZE,
            0.,
        ),
        "BR Middle Right",
    ),
    (
        (
            PLANTS_RIGHT_SIDE_X - 15. * TILE_SIZE,
            PLANTS_SECOND_LINE_Y - 25. * TILE_SIZE,
            0.,
        ),
        "BR Down",
    ),
];

/* -------------------------------------------------------------------------- */
/*                                  Audience                                  */
/* -------------------------------------------------------------------------- */

pub const LANDMARK_AUDIENCE: (f32, f32, f32) =
    (THRONE_X * TILE_SIZE, THRONE_Y - 50. * TILE_SIZE, 0.);

/* -------------------------------------------------------------------------- */
/*                              Throne and Sides                              */
/* -------------------------------------------------------------------------- */

pub const LANDMARK_THRONE_SIT: (f32, f32, f32) = (THRONE_X * TILE_SIZE, THRONE_Y * TILE_SIZE, 0.);

pub const LANDMARK_THRONE_RIGHT_SIDE: (f32, f32, f32) =
    (THRONE_X - 20. * TILE_SIZE, THRONE_Y - 10. * TILE_SIZE, 0.);
pub const LANDMARK_THRONE_LEFT_SIDE: (f32, f32, f32) =
    (THRONE_X + 20. * TILE_SIZE, THRONE_Y - 10. * TILE_SIZE, 0.);

pub const LANDMARK_STAIRS_RIGHT: (f32, f32, f32) =
    (THRONE_X - 40. * TILE_SIZE, THRONE_Y - 25. * TILE_SIZE, 0.);
pub const LANDMARK_STAIRS_LEFT: (f32, f32, f32) =
    (THRONE_X + 40. * TILE_SIZE, THRONE_Y - 25. * TILE_SIZE, 0.);

/* -------------------------------------------------------------------------- */
/*                                   Pillars                                  */
/* -------------------------------------------------------------------------- */

// each pillar has two landmarks
pub const LANDMARK_PILLARS: [([((f32, f32, f32), &str); 2], &str); 6] = [
    (
        [
            (
                (
                    PILLAR_FIRST_COLUMN_X + 10. * TILE_SIZE,
                    PILLAR_FIRST_LINE_Y * TILE_SIZE,
                    0.,
                ),
                "Outer Side",
            ),
            (
                (
                    PILLAR_FIRST_COLUMN_X - 15. * TILE_SIZE,
                    PILLAR_FIRST_LINE_Y + 10. * TILE_SIZE,
                    0.,
                ),
                "Inner Side",
            ),
        ],
        "First Pillar",
    ),
    (
        [
            (
                (
                    PILLAR_FIRST_COLUMN_X + 10. * TILE_SIZE,
                    PILLAR_SECOND_LINE_Y * TILE_SIZE,
                    0.,
                ),
                "Outer Side",
            ),
            (
                (
                    PILLAR_FIRST_COLUMN_X - 15. * TILE_SIZE,
                    PILLAR_SECOND_LINE_Y + 10. * TILE_SIZE,
                    0.,
                ),
                "Inner Side",
            ),
        ],
        "Second Pillar",
    ),
    (
        [
            (
                (
                    PILLAR_FIRST_COLUMN_X + 10. * TILE_SIZE,
                    PILLAR_THIRD_LINE_Y * TILE_SIZE,
                    0.,
                ),
                "Outer Side",
            ),
            (
                (
                    PILLAR_FIRST_COLUMN_X - 15. * TILE_SIZE,
                    PILLAR_THIRD_LINE_Y + 10. * TILE_SIZE,
                    0.,
                ),
                "Inner Side",
            ),
        ],
        "Third Pillar",
    ),
    (
        [
            (
                (
                    PILLAR_SECOND_COLUMN_X + 10. * TILE_SIZE,
                    PILLAR_FIRST_LINE_Y * TILE_SIZE,
                    0.,
                ),
                "Outer Side",
            ),
            (
                (
                    PILLAR_SECOND_COLUMN_X - 15. * TILE_SIZE,
                    PILLAR_FIRST_LINE_Y + 10. * TILE_SIZE,
                    0.,
                ),
                "Inner Side",
            ),
        ],
        "Fourth Pillar",
    ),
    (
        [
            (
                (
                    PILLAR_SECOND_COLUMN_X + 10. * TILE_SIZE,
                    PILLAR_SECOND_LINE_Y * TILE_SIZE,
                    0.,
                ),
                "Outer Side",
            ),
            (
                (
                    PILLAR_SECOND_COLUMN_X - 15. * TILE_SIZE,
                    PILLAR_SECOND_LINE_Y + 10. * TILE_SIZE,
                    0.,
                ),
                "Inner Side",
            ),
        ],
        "Fifth Pillar",
    ),
    (
        [
            (
                (
                    PILLAR_SECOND_COLUMN_X + 10. * TILE_SIZE,
                    PILLAR_THIRD_LINE_Y * TILE_SIZE,
                    0.,
                ),
                "Outer Side",
            ),
            (
                (
                    PILLAR_SECOND_COLUMN_X - 15. * TILE_SIZE,
                    PILLAR_THIRD_LINE_Y + 10. * TILE_SIZE,
                    0.,
                ),
                "Inner Side",
            ),
        ],
        "Sixth Pillar",
    ),
];

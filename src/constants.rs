pub const BACKGROUND_COLOR: bevy::render::color::Color = bevy::render::color::Color::Rgba {
    red: 58.0 / 256.0,
    green: 36.0 / 256.0,
    blue: 48.0 / 256.0,
    alpha: 1.0,
};

pub mod ui {
    pub mod dialogs {
        pub const DIALOG_BOX_UPDATE_DELTA: f32 = 0.08;
        pub const SCROLL_SIZE: (f32, f32) = (490.0, 11700.0 / 45.0);
        pub const SCROLL_ANIMATION_DELTA_S: f32 = 0.1;
        pub const SCROLL_ANIMATION_FRAMES_NUMBER: usize = 45;
    }
}

pub mod player {
    use crate::player::PlayerAnimationType;

    pub const STARTING_ANIMATION: PlayerAnimationType = PlayerAnimationType::RightIdle;
    pub const PLAYER_WIDTH: f32 = 12.0;
    pub const PLAYER_HEIGHT: f32 = 15.0;
    pub const PLAYER_Z: f32 = 5.0;
    pub const PLAYER_SCALE: f32 = 6.0;
    pub const PLAYER_HITBOX_WIDTH: f32 = 35.0;
    pub const PLAYER_HITBOX_HEIGHT: f32 = 20.0;
    pub const PLAYER_HITBOX_Y_OFFSET: f32 = -25.0;

    pub const CAMERA_INTERPOLATION: f32 = 0.1;
}

pub mod locations {
    pub mod temple {
        pub const BACKGROUND_Z: f32 = 0.0;

        pub const TEMPLE_Z: f32 = 2.0;
        pub const TEMPLE_Z_WHEN_IN_SECRET_ROOM: f32 = 5.5;

        pub const SECOND_CORRIDOR_Z: f32 = 2.1;
        pub const FIRST_CORRIDOR_Z: f32 = 2.2;
        pub const CORRIDOR_DOORS_Z: f32 = 7.0;

        pub const GROUND_Z: f32 = 0.5;

        pub const THRONE_Z_BACK: f32 = 3.0;
        pub const THRONE_Z_FRONT: f32 = 6.0;
        pub const THRONE_POSITION: (f32, f32, f32) = (-230.0, 900.0, THRONE_Z_BACK);

        pub const SECRET_ROOM_Z: f32 = 1.0;
        pub const SECRET_ROOM_TRIGGER_Y: f32 = 990.0;
        pub const SECRET_ROOM_COVER_POSITION: (f32, f32, f32) = (-240.0, 1375.0, 1.5);
        pub const SECRET_ROOM_COVER_SIZE: (f32, f32) = (2500.0, 800.0);

        pub const CURTAINS_Z_BACK: f32 = 3.0;
        pub const CURTAINS_Z_FRONT: f32 = 7.0;
        pub const CURTAINS_ANIMATION_DELTA: f32 = 0.1;
        pub const CURTAINS_SENSOR_Y_OFFSET: f32 = -150.0;
        pub const CURTAINS_CHANGE_Z_TIME: f32 = 0.17;
        pub const CURTAINS_TRIGGER_Y: f32 = 930.0;
        pub const LEFT_CURTAIN_POSITION: (f32, f32, f32) = (-440.0, 1080.0, CURTAINS_Z_BACK);
        pub const RIGHT_CURTAIN_POSITION: (f32, f32, f32) = (-40.0, 1080.0, CURTAINS_Z_BACK);

        pub const PILLARS_Z_BACK: f32 = 3.0;
        pub const PILLARS_Z_FRONT: f32 = 6.0;
        pub const PILLAR_POSITIONS: [(f32, f32, f32); 12] = [
            // 1 2    7  8
            // 3 4    9  10
            // 5 6    11 12
            (-960.0, 585.0, PILLARS_Z_BACK),  // 1
            (-530.0, 585.0, PILLARS_Z_BACK),  // 2
            (-960.0, 125.0, PILLARS_Z_BACK),  // 3
            (-530.0, 125.0, PILLARS_Z_BACK),  // 4
            (-960.0, -335.0, PILLARS_Z_BACK), // 5
            (-530.0, -335.0, PILLARS_Z_BACK), // 6
            (70.0, 585.0, PILLARS_Z_BACK),    // 7
            (500.0, 585.0, PILLARS_Z_BACK),   // 8
            (70.0, 125.0, PILLARS_Z_BACK),    // 9
            (500.0, 125.0, PILLARS_Z_BACK),   // 10
            (70.0, -335.0, PILLARS_Z_BACK),   // 11
            (500.0, -335.0, PILLARS_Z_BACK),  // 12
        ];

        pub const OLF_CAT_Z: f32 = 1.4;
        pub const OLF_CAT_SCALE: f32 = 0.5;
        pub const OLF_CAT_ANIMATION_DELTA: f32 = 0.5;
        pub const OLF_CAT_POSITION: (f32, f32, f32) = (-500.0, 1480.0, OLF_CAT_Z);
        pub const OLF_CAT_HITBOX_POSITION: (f32, f32, f32) =
            (OLF_CAT_POSITION.0, OLF_CAT_POSITION.1 - 5.0, OLF_CAT_Z);
    }
}

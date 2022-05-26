pub const BACKGROUND_COLOR: bevy::render::color::Color = bevy::render::color::Color::Rgba {
    red: 58.0 / 256.0,
    green: 36.0 / 256.0,
    blue: 48.0 / 256.0,
    alpha: 1.0,
};

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
        pub const TEMPLE_Z_WHEN_IN_SECRET_ROOM: f32 = 6.0;
        pub const GROUND_Z: f32 = 0.5;
        pub const THRONE_Z_BACK: f32 = 3.0;
        pub const THRONE_Z_FRONT: f32 = 6.0;
        pub const SECRET_ROOM_Z: f32 = 1.0;
        pub const SECRET_ROOM_COVER_Z: f32 = 1.5;
        pub const CURTAINS_Z_BACK: f32 = 3.0;
        pub const CURTAINS_Z_FRONT: f32 = 7.0;
        pub const CURTAINS_ANIMATION_DELTA: f32 = 0.1;
        pub const CURTAINS_SENSOR_Y_OFFSET: f32 = -150.0;
        pub const CURTAINS_CHANGE_Z_TIME: f32 = 0.17;
        pub const PILLARS_Z_BACK: f32 = 3.0;
        pub const PILLARS_Z_FRONT: f32 = 6.0;
        pub const PILLAR_POSITIONS: [(f32, f32, f32); 12] = [
            (-900.0, 140.0, PILLARS_Z_BACK),
            (-300.0, 140.0, PILLARS_Z_BACK),
            (300.0, 140.0, PILLARS_Z_BACK),
            (900.0, 140.0, PILLARS_Z_BACK),
            (-900.0, -300.0, PILLARS_Z_BACK),
            (-300.0, -300.0, PILLARS_Z_BACK),
            (300.0, -300.0, PILLARS_Z_BACK),
            (900.0, -300.0, PILLARS_Z_BACK),
            (-900.0, -750.0, PILLARS_Z_BACK),
            (-300.0, -750.0, PILLARS_Z_BACK),
            (300.0, -750.0, PILLARS_Z_BACK),
            (900.0, -750.0, PILLARS_Z_BACK),
        ];
        pub const SECRET_ROOM_TRIGGER_Y: f32 = 550.0;
        pub const OLF_CAT_Z: f32 = 2.0;
        pub const OLF_CAT_SCALE: f32 = 0.5;
        pub const OLF_CAT_ANIMATION_DELTA: f32 = 0.5;
    }
}

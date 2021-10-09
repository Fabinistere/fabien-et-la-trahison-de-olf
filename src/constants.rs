pub mod player {
    use crate::player::PlayerAnimationType;

    pub const STARTING_ANIMATION: PlayerAnimationType = PlayerAnimationType::RightIdle;
    pub const PLAYER_WIDTH: f32 = 12.0;
    pub const PLAYER_HEIGHT: f32 = 15.0;
    pub const PLAYER_Z: f32 = 5.0;
    pub const PLAYER_SCALE: f32 = 6.0;
}

pub mod locations {
    pub mod temple {
        pub const BACKGROUND_Z: f32 = 0.0;
        pub const TEMPLE_Z: f32 = 1.0;
        pub const TEMPLE_Z_WHEN_IN_SECRET_ROOM: f32 = 6.0;
        pub const STONES_Z: f32 = 3.5;
        pub const SECRET_ROOM_Z: f32 = 2.0;
        pub const PILLARS_Z_BACK: f32 = 3.0;
        pub const PILLARS_Z_FRONT: f32 = 6.0;
        pub const PILLAR_POSITIONS: [(f32, f32, f32); 12] = [
            (-900.0, 210.0, PILLARS_Z_BACK),
            (-300.0, 210.0, PILLARS_Z_BACK),
            (300.0, 210.0, PILLARS_Z_BACK),
            (900.0, 210.0, PILLARS_Z_BACK),
            (-900.0, -230.0, PILLARS_Z_BACK),
            (-300.0, -230.0, PILLARS_Z_BACK),
            (300.0, -230.0, PILLARS_Z_BACK),
            (900.0, -230.0, PILLARS_Z_BACK),
            (-900.0, -680.0, PILLARS_Z_BACK),
            (-300.0, -680.0, PILLARS_Z_BACK),
            (300.0, -680.0, PILLARS_Z_BACK),
            (900.0, -680.0, PILLARS_Z_BACK),
        ];
    }
}

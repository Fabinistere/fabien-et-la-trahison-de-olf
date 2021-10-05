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
        pub const TEMPLE_Z: f32 = 1.0;
        pub const TEMPLE_SCALE: f32 = 1.0;
        pub const COLUMNS_Z_BACK: f32 = 4.0;
        pub const COLUMNS_Z_FRONT: f32 = 6.0;
    }
}

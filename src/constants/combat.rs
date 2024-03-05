pub const BASE_ACTION_COUNT: usize = 1;
pub const MAX_PARTY: usize = 6;
pub const FIRST_ALLY_ID: usize = 0;
pub const FIRST_ENEMY_ID: usize = MAX_PARTY;

pub mod team {
    pub const TEAM_MC: i32 = 0;
    pub const TEAM_OLF: i32 = 1;
    pub const TEAM_FABICURION: i32 = 2;
}

pub mod skill {
    use crate::animations::sprite_sheet_animation::SpriteSheetIndex;

    pub const BAM: i32 = 150;

    pub const HOLY_SPELL_01_START_INDEX: usize = 16;
    pub const HOLY_SPELL_01_END_INDEX: usize = 22;
    pub const HOLY_SPELL_02_START_INDEX: usize = 0;
    pub const HOLY_SPELL_02_END_INDEX: usize = 15;

    pub const HOLY_SPELL_01: SpriteSheetIndex = SpriteSheetIndex {
        start_index: HOLY_SPELL_01_START_INDEX,
        end_index: HOLY_SPELL_01_END_INDEX,
    };
    pub const HOLY_SPELL_02: SpriteSheetIndex = SpriteSheetIndex {
        start_index: HOLY_SPELL_02_START_INDEX,
        end_index: HOLY_SPELL_02_END_INDEX,
    };
}

pub mod alteration {
    pub const SIZE_ALTERATION_ICON: f32 = 5.;
}

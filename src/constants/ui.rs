use bevy::render::color::Color;

pub const DRAGGED_ENTITY_Z: f32 = 100.0;
pub const FIGHTING_HALL_WIDTH: f32 = 100. - (INITIATIVE_BAR_WIDTH + HUD_WALL_WIDTH);
pub const INITIATIVE_BAR_WIDTH: f32 = 8.;
pub const HUD_WALL_WIDTH: f32 = 40.;

pub const HUD_PANEL_ANIMATION_TIME_MS: u64 = 500;
pub const HUD_PANEL_ANIMATION_OFFSET: f32 = -1000.;

// --- Buttons ---

pub const TRANSPARENT_BUTTON: Color = Color::rgba(0., 0., 0., 0.);
pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
// pub const NORMAL_BUTTON: Color = Color::rgba(0.1, 0.1, 0.1, 0.1);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

// #3c3e40
pub const INACTIVE_BUTTON: Color = Color::rgb(0.23, 0.24, 0.25);
// #60666a
pub const INACTIVE_HOVERED_BUTTON: Color = Color::rgb(0.37, 0.40, 0.41);

pub mod fighting_hall_position {

    /* -------------------------------------------------------------------------- */
    /*                               Enemy Position                               */
    /* -------------------------------------------------------------------------- */

    pub const ENEMY_FRONTLINE_LEFT: (f32, f32) = (5., 8.);
    pub const ENEMY_FRONTLINE_MIDDLE: (f32, f32) = (7., 10.);
    pub const ENEMY_FRONTLINE_RIGHT: (f32, f32) = (9., 12.);

    pub const ENEMY_MIDDLELINE_LEFT: (f32, f32) = (3., 10.);
    pub const ENEMY_MIDDLELINE_MIDDLE: (f32, f32) = (5., 12.);
    pub const ENEMY_MIDDLELINE_RIGHT: (f32, f32) = (7., 14.);

    pub const ENEMY_BACKLINE_LEFT: (f32, f32) = (1., 12.);
    pub const ENEMY_BACKLINE_MIDDLE: (f32, f32) = (3., 14.);
    pub const ENEMY_BACKLINE_RIGHT: (f32, f32) = (5., 16.);

    /* -------------------------------------------------------------------------- */
    /*                                Ally Position                               */
    /* -------------------------------------------------------------------------- */

    pub const ALLY_FRONTLINE_LEFT: (f32, f32) = (9., 5.);
    pub const ALLY_FRONTLINE_MIDDLE: (f32, f32) = (11., 7.);
    pub const ALLY_FRONTLINE_RIGHT: (f32, f32) = (13., 9.);

    pub const ALLY_MIDDLELINE_LEFT: (f32, f32) = (11., 3.);
    pub const ALLY_MIDDLELINE_MIDDLE: (f32, f32) = (13., 5.);
    pub const ALLY_MIDDLELINE_RIGHT: (f32, f32) = (15., 7.);

    pub const ALLY_BACKLINE_LEFT: (f32, f32) = (13., 1.);
    pub const ALLY_BACKLINE_MIDDLE: (f32, f32) = (15., 3.);
    pub const ALLY_BACKLINE_RIGHT: (f32, f32) = (17., 5.);
}

pub mod dialogs {
    pub const DIALOG_BOX_UPDATE_DELTA_S: f32 = 0.05;
    pub const SCROLL_SIZE: (f32, f32) = (490., 11700. / 45.);
    pub const SCROLL_ANIMATION_DELTA_S: f32 = 0.1;
    pub const SCROLL_ANIMATION_FRAMES_NUMBER: usize = 45;

    pub const FIRST_BUTTON_TOP_VAL: f32 = 690.;
    pub const BUTTON_SPACING: f32 = 320.;
    pub const BUTTON_LEFT_VAL: f32 = -52.;
}

pub mod style {
    //, text::TextStyle, ui::*
    use bevy::prelude::*;

    pub fn get_text_style(asset_server: &Res<AssetServer>, font_size: f32) -> TextStyle {
        TextStyle {
            font: asset_server.load("fonts/dpcomic.ttf"),
            font_size,
            color: Color::WHITE, // rgb(0.9, 0.9, 0.9),
        }
    }

    // NOTE: Style Constant or Style Method ? (see: https://discord.com/channels/691052431525675048/1119426776033140879)
    // --- Style Constant ---
    // pub const TEXT_STYLE: Style = {
    //     let mut style = Style::DEFAULT;
    //     style.flex_shrink = 0.;
    //     style.width = Val::Px(0.);
    //     style.height = Val::Px(20.);
    //     style.margin = UiRect {
    //         left: Val::Auto,
    //         right: Val::Auto,
    //         ..UiRect::DEFAULT
    //     };
    //     style
    // };
    // --- Style Method ---
    // pub const TEXT_STYLE: Style = text_style();
    // pub const fn text_style() -> Style {
    //     Style {
    //         flex_shrink: 0.,
    //         width: Val::Px(0.),
    //         height: Val::Px(20.),
    //         margin: UiRect {
    //             left: Val::Auto,
    //             right: Val::Auto,
    //             ..UiRect::DEFAULT
    //         },
    //         ..Style::DEFAULT
    //     }
    // }

    pub const TEXT_STYLE: Style = {
        let mut style = Style::DEFAULT;
        style.flex_shrink = 0.;
        style.height = Val::Px(20.);
        style.margin = UiRect {
            left: Val::Auto,
            right: Val::Auto,
            ..UiRect::DEFAULT
        };
        style
    };

    pub const LIST_HIDDEN_OVERFLOW_STYLE: Style = {
        let mut style = Style::DEFAULT;
        style.flex_direction = FlexDirection::Column;
        style.align_self = AlignSelf::Stretch;
        style.overflow = Overflow::clip_y();
        style
    };

    pub const MOVING_PANEL_STYLE: Style = {
        let mut style = Style::DEFAULT;
        style.flex_direction = FlexDirection::Column;
        style.flex_wrap = FlexWrap::NoWrap;
        style.align_items = AlignItems::FlexStart;
        style
    };

    pub const SKILL_BUTTON_STYLE: Style = {
        let mut style = Style::DEFAULT;
        style.width = Val::Px(150.0);
        style.height = Val::Px(65.0);
        // center button
        style.margin = UiRect::all(Val::Auto);
        // horizontally center child text
        style.justify_content = JustifyContent::Center;
        // vertically center child text
        style.align_items = AlignItems::Center;
        style
    };

    pub const ACTION_BUTTON_STYLE: Style = {
        let mut style = Style::DEFAULT;
        style.width = Val::Px(154.); // Val::Percent(100.);
        style.height = Val::Px(103.);
        style.justify_content = JustifyContent::Center;
        style.flex_direction = FlexDirection::ColumnReverse;
        style.flex_wrap = FlexWrap::NoWrap;
        style.align_items = AlignItems::Center;
        style
    };

    pub const ALLIES_SHEET_STYLE: Style = {
        let mut style = Style::DEFAULT;
        style.flex_shrink = 0.;
        style.flex_direction = FlexDirection::Column;
        style.width = Val::Percent(100.);
        style.height = Val::Percent(50.);
        // gap between the two rows
        style.row_gap = Val::Percent(8.);
        style
    };

    pub const ROW_SHEETS_STYLE: Style = {
        let mut style = Style::DEFAULT;
        style.flex_shrink = 0.;
        style.flex_direction = FlexDirection::Row;
        style.height = Val::Percent(50.);
        // gap between the three scrolls
        style.column_gap = Val::Percent(2.3);
        style
    };

    pub const MINI_CHARACTER_SHEET_STYLE: Style = {
        let mut style = Style::DEFAULT;
        style.width = Val::Percent(23.);
        style.height = Val::Percent(96.);
        style.left = Val::Percent(16.8);
        style.top = Val::Percent(16.);
        style
    };
}

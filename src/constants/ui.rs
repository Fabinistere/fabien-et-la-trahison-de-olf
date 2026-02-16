pub mod dialogs {
    use bevy::prelude::Color;

    pub const DIALOG_PANEL_ANIMATION_OFFSET: f32 = -1000.;
    pub const DIALOG_BOX_UPDATE_DELTA_S: f32 = 0.05;
    pub const DIALOG_PANEL_ANIMATION_TIME_MS: u64 = 500;
    pub const SCROLL_SIZE: (f32, f32) = (490., 11700. / 45.);
    pub const SCROLL_ANIMATION_DELTA_S: f32 = 0.1;
    pub const SCROLL_ANIMATION_FRAMES_NUMBER: usize = 45;

    pub const UPPER_TEXT_TOP: f32 = 53.;

    pub const FIRST_BUTTON_TOP_PERCENT: f32 = 65.;
    pub const BUTTON_SPACING: f32 = 30.;
    pub const BUTTON_LEFT_PERCENT: f32 = 2.5;
    pub const BUTTON_WIDTH: f32 = 60.;

    pub const TRANSPARENT_BUTTON: Color = Color::rgba(0., 0., 0., 0.);
    // pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
    pub const NORMAL_BUTTON: Color = Color::rgba(0.1, 0.1, 0.1, 0.1);
    pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
    pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
}

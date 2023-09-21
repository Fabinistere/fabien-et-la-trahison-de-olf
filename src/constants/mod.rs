pub mod character;
pub mod locations;
pub mod title_screen;
pub mod ui;

// dark blue #213757 = 33/255, 55/255, 87/255
pub const BACKGROUND_COLOR_INMENU: bevy::render::color::Color =
    bevy::render::color::Color::rgb(33. / 255., 55. / 255., 87. / 255.);
// dark purple #25131a = 39/255, 19/255, 26/255
pub const BACKGROUND_COLOR_INGAME: bevy::render::color::Color =
    bevy::render::color::Color::rgb(0.153, 0.07, 0.102);
// pub const BACKGROUND_COLOR: bevy::render::color::Color = bevy::render::color::Color::Rgba {
//     red: 58. / 256.,
//     green: 36. / 256.,
//     blue: 48. / 256.,
//     alpha: 1.,
// };

pub const RESOLUTION: f32 = 9. / 16.; // 16. / 9.;
pub const TILE_SIZE: f32 = 1.;

pub const FRAME_TIME: f32 = 0.1;

pub mod interactions {
    pub const INTERACT_BUTTON_Z: f32 = 20.;
    pub const INTERACT_BUTTON_SCALE: f32 = 0.25;

    // REFACTOR: INTERACTION_ID
}

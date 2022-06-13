pub mod dialog_box;

use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ui)
            .add_startup_system(dialog_box::load_textures)
            .add_system(dialog_box::update_dialog_box)
            .add_system(dialog_box::animate_scroll)
            .add_system(dialog_box::create_dialog_box_on_key_press);
    }
}

#[derive(Component)]
pub struct UiCamera;
#[derive(Component)]
pub struct UiElement;

fn setup_ui(mut commands: Commands) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(UiCamera);
}

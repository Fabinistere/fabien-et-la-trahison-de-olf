use bevy::prelude::*;

use self::dialog::UiDialogPlugin;

pub mod dialog;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiDialogPlugin);
    }
}

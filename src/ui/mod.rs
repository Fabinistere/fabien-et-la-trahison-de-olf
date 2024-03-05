use bevy::{prelude::*, winit::WinitSettings};
use bevy_tweening::TweenCompleted;

use self::combat::UiCombatPlugin;
use self::dialog::UiDialogPlugin;

pub mod combat;
pub mod dialog;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            // OPTIMIZE: Only run the app when there is user input. This will significantly reduce CPU/GPU use.
            .insert_resource(WinitSettings::game())
            .add_plugins((UiDialogPlugin, UiCombatPlugin))
            .add_systems(Update, despawn_hud_panel);
    }
}

pub fn despawn_hud_panel(mut commands: Commands, mut completed_event: EventReader<TweenCompleted>) {
    for TweenCompleted { entity, user_data } in completed_event.iter() {
        if *user_data == 0 {
            commands.entity(*entity).despawn_recursive();
        }
    }
}

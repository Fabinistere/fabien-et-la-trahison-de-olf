use bevy::prelude::*;

use crate::HUDState;

mod dialog_box;
pub mod dialog_panel;
mod dialog_player;
pub mod dialog_scrolls;
pub mod dialog_systems;

pub struct UiDialogPlugin;

impl Plugin for UiDialogPlugin {
    // #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app.insert_resource(dialog_systems::DialogMap::default())
            .insert_resource(dialog_systems::CurrentInterlocutor::default())
            .insert_resource(dialog_systems::ActiveWorldEvents::default())
            .insert_resource(dialog_scrolls::Monolog::default())
            .add_event::<dialog_box::ResetDialogBoxEvent>()
            .add_event::<dialog_systems::ChangeStateEvent>()
            .add_event::<dialog_systems::TriggerEvents>()
            // Trigger Event
            // .add_event::<dialog_system::FightEvent>()
            // .add_event::<dialog_system::TriggerEvent>()
            .add_systems(Startup, dialog_panel::load_textures)
            // OPTIMIZE: System Ordering
            .add_systems(Update, dialog_panel::create_dialog_panel_on_key_press)
            .add_systems(
                OnEnter(HUDState::DialogWall),
                dialog_panel::create_dialog_panel,
            )
            .add_systems(
                Update,
                (
                    // TODO: Chain
                    dialog_systems::change_dialog_state,
                    dialog_systems::update_dialog_panel.after(dialog_systems::change_dialog_state),
                    dialog_systems::update_monolog.after(dialog_systems::update_dialog_panel),
                    // End Chain
                    dialog_systems::trigger_event_handler
                        .after(dialog_systems::change_dialog_state),
                    /* -------------------------------------------------------------------------- */
                    /*                                   Inputs                                   */
                    /* -------------------------------------------------------------------------- */
                    dialog_player::choose_answer,
                    dialog_player::continue_monolog,
                    /* -------------------------------------------------------------------------- */
                    /*                                  Animation                                 */
                    /* -------------------------------------------------------------------------- */
                    dialog_scrolls::animate_scroll,
                    dialog_box::reset_dialog_box.after(dialog_systems::update_monolog),
                    dialog_box::update_dialog_box,
                )
                    .run_if(in_state(HUDState::DialogWall)),
            )
            .add_systems(
                OnExit(HUDState::DialogWall),
                dialog_panel::close_dialog_panel,
            );
    }
}

#[derive(Component)]
pub struct UiElement;

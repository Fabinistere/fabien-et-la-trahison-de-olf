//! All dialog method handler related with the player directly (input, etc)

use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};
use yml_dialog::Content;

use crate::ui::{
    dialog_scrolls::{ButtonChoice, Monolog},
    dialog_systems::{ChangeStateEvent, CurrentInterlocutor, DialogMap},
};

pub fn choose_answer(
    choice_query: Query<(&ButtonChoice, &Interaction), Changed<Interaction>>,
    mut change_state_event: EventWriter<ChangeStateEvent>,
) {
    for (button_infos, interaction) in &choice_query {
        if *interaction == Interaction::Pressed {
            change_state_event.send(ChangeStateEvent(button_infos.exit_state))
        }
    }
}

pub fn continue_monolog(
    mut key_evr: EventReader<KeyboardInput>,
    mut current_monolog: ResMut<Monolog>,
    current_interlocutor: Res<CurrentInterlocutor>,
    dialogs: Res<DialogMap>,

    mut change_state_event: EventWriter<ChangeStateEvent>,
) {
    for KeyboardInput {
        scan_code: _,
        key_code: _,
        state,
        window: _,
    } in key_evr.iter()
    {
        if *state == ButtonState::Pressed {
            if current_monolog.texts.len() > 1 {
                if let Some((_first, rem)) = current_monolog.texts.split_first() {
                    current_monolog.texts = rem.to_vec();
                }
            } else {
                match current_interlocutor.interlocutor {
                    None => {}
                    Some(interlocutor) => {
                        if let Some(&(current_state, ref dialog)) = dialogs.get(&interlocutor) {
                            if let Some(current_node) = dialog.get(&current_state) {
                                match current_node.content() {
                                    Content::Choices(_) => {}
                                    Content::Monolog {
                                        text: _,
                                        exit_state,
                                    } => change_state_event.send(ChangeStateEvent(*exit_state)),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

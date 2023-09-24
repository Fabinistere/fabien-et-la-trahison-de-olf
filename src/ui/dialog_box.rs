//! Every Structs and methods only about Dialog Box

use bevy::prelude::*;

use crate::{constants::ui::dialogs::DIALOG_BOX_UPDATE_DELTA_S, ui::dialog_scrolls::ButtonChoice};

use super::dialog_scrolls::MonologPanel;

/// Represents the entity containing the displayed text as first children.
///
/// Used to animate the Text, letter by letter.
#[derive(Debug, Component)]
pub struct DialogBox {
    pub text: String,
    progress: usize,
    finished: bool,
    update_timer: Timer,
}

impl DialogBox {
    pub fn new(text: String, update_time: f32) -> Self {
        DialogBox {
            text,
            progress: 0,
            finished: false,
            update_timer: Timer::from_seconds(update_time, TimerMode::Once),
        }
    }

    // Same as new but keep the signature
    // fn reset(&self, text: String, update_time: f32) {
    //     *self.text = text;
    //     *self.progress = 0;
    //     *self.finished = false;
    //     *self.update_timer = Timer::from_seconds(update_time, TimerMode::Once);
    // }
}

/// DOC: REDO the upper/player scroll doc related
/// Happens when
///   - ui::dialog_panel::update_upper_scroll
///     - updates UpperScroll Text with the UpperScroll infos
///   - ui::dialog_panel::update_player_scroll
///     - updates PlayerScroll Text with the UpperScroll infos
///     happens for every choice there is in the PlayerScroll
/// Read in
///   - ui::dialog_panel::reset_dialog_box
///     - creates a DialogBox to transfer info to the child Text
///     if there is none
///     or resets the text and dialogBox
#[derive(Event)]
pub struct ResetDialogBoxEvent {
    pub dialog_box: Entity,
    /// could be
    ///
    /// - a Choice
    /// - a Text
    pub text: String,
}

/// Reset DialogBox on Event
pub fn reset_dialog_box(
    mut commands: Commands,

    mut reset_event: EventReader<ResetDialogBoxEvent>,

    mut dialog_box_query: Query<
        (Option<&mut DialogBox>, &Children),
        Or<(With<ButtonChoice>, With<MonologPanel>)>,
    >,
    mut text_query: Query<&mut Text>,
) {
    for ResetDialogBoxEvent {
        dialog_box,
        text: event_text,
    } in reset_event.iter()
    {
        let (potential_dialog_box, children) = dialog_box_query.get_mut(*dialog_box).unwrap();
        match potential_dialog_box {
            None => {
                // info!("DEBUG: no DialogBox in the UpperScroll/ButtonChoice");
                commands.entity(*dialog_box).insert(DialogBox::new(
                    event_text.clone(),
                    DIALOG_BOX_UPDATE_DELTA_S,
                ));
                let mut text = text_query.get_mut(children[0]).unwrap();
                text.sections[0].value.clear();
            }
            Some(mut dialog_box) => {
                // FIXME: bug - Reset the text even if there is no change
                // Clear the DialogBox Child: the Text
                let mut text = text_query.get_mut(children[0]).unwrap();
                if dialog_box.text != event_text.clone() {
                    text.sections[0].value.clear();
                    // replace current DialogBox with a brand new one
                    *dialog_box = DialogBox::new(event_text.clone(), DIALOG_BOX_UPDATE_DELTA_S);
                }
            }
        }
    }
}

/// Animates, letter by letter, each Text.
/// ( being the DialogBox's 1rt child )
pub fn update_dialog_box(
    time: Res<Time>,
    mut dialog_box_query: Query<(&mut DialogBox, &Children)>,
    mut text_query: Query<&mut Text>,
) {
    for (mut dialog_box, children) in dialog_box_query.iter_mut() {
        dialog_box.update_timer.tick(time.delta());

        if dialog_box.update_timer.finished() && !dialog_box.finished {
            // let mut text = text_query.get_mut(children[0]).unwrap();
            match text_query.get_mut(children[0]) {
                // FIXME: If there is no TEXT then insert one in it
                // pb: on which scroll...
                Err(e) => error!("No Text in the Dialog Wall: {:?}", e),
                Ok(mut text) => {
                    // prompt the simple text
                    // FIXME: bug - if the given text contains a accent this will crash
                    match dialog_box.text.chars().nth(dialog_box.progress) {
                        // will ignore any louche symbol
                        None => {
                            println!("text: {}", dialog_box.text);
                            error!("Blank or Accent Typical Crash");
                            dialog_box.progress += 1;
                            if dialog_box.progress >= dialog_box.text.len() {
                                dialog_box.finished = true;
                            }
                        }
                        Some(next_letter) => {
                            text.sections[0].value.push(next_letter);

                            dialog_box.progress += 1;
                            if dialog_box.progress >= dialog_box.text.len() {
                                dialog_box.finished = true;
                            }
                        }
                    }
                }
            }
        }
    }
}

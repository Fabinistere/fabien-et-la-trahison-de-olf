//! Dialog Logic

use core::fmt;
use std::{collections::BTreeMap, str::FromStr};

use bevy::prelude::*;
use rand::seq::SliceRandom;
use yml_dialog::{Content, DialogNode};

use super::{
    dialog_box::ResetDialogBoxEvent,
    dialog_scrolls::{ButtonChoice, Monolog, MonologPanel},
};

// Funny artefacts:

// don't change panel.dialog_tree here
// it will be detected by update_dialog_panel
// i'm living in the fear
// i'm in danger
// my own program wants me dead

/// Points to the current entity, if they exist, who we're talking with.
/// Query this entity to get the current Dialog.
#[derive(Debug, Reflect, Deref, DerefMut, Clone, Default, Resource)]
pub struct CurrentInterlocutor {
    pub interlocutor: Option<Entity>,
}

/// Points to the current entity, if they exist, who we're talking with.
/// Query this entity to get the current Dialog.
#[derive(Debug, Reflect, Deref, DerefMut, Clone, Default, Resource)]
pub struct ActiveWorldEvents {
    active_world_events: Vec<WorldEvent>,
}

/// - `key`: interlocutor
/// - `value`: (current state, BinaryTreeMap of the dialog)
#[derive(Debug, Deref, DerefMut, Default, Resource)]
pub struct DialogMap(BTreeMap<Entity, (usize, BTreeMap<usize, DialogNode>)>);

#[derive(Reflect, PartialEq, Clone, Copy, Debug)]
pub enum WorldEvent {
    BeatTheGame,
    FirstKill,
    AreaCleared,
    HasCharisma,
    HasFriend,
}

impl fmt::Display for WorldEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WorldEvent::BeatTheGame => write!(f, "BeatTheGame"),
            WorldEvent::FirstKill => write!(f, "FirstKill"),
            WorldEvent::AreaCleared => write!(f, "AreaCleared"),
            WorldEvent::HasCharisma => write!(f, "HasCharisma"),
            WorldEvent::HasFriend => write!(f, "HasFriend"),
        }
    }
}

impl FromStr for WorldEvent {
    type Err = (); // ParseIntError;

    fn from_str(input: &str) -> Result<WorldEvent, Self::Err> {
        match input {
            "BeatTheGame" => Ok(WorldEvent::BeatTheGame),
            "FirstKill" => Ok(WorldEvent::FirstKill),
            "AreaCleared" => Ok(WorldEvent::AreaCleared),
            "HasCharisma" => Ok(WorldEvent::HasCharisma),
            "HasFriend" => Ok(WorldEvent::HasFriend),
            _ => Err(()),
        }
    }
}

/// DOC
///
/// List all triggerable event,
/// that can be send when quitting a dialog node
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ThrowableEvent {
    FightEvent,
    HasFriend,
}

impl fmt::Display for ThrowableEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ThrowableEvent::FightEvent => write!(f, "FightEvent"),
            ThrowableEvent::HasFriend => write!(f, "HasFriend"),
        }
    }
}

impl FromStr for ThrowableEvent {
    type Err = (); // ParseIntError;

    fn from_str(input: &str) -> Result<ThrowableEvent, Self::Err> {
        match input {
            "FightEvent" => Ok(ThrowableEvent::FightEvent),
            "HasFriend" => Ok(ThrowableEvent::HasFriend),
            _ => Err(()),
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                                   Systems                                  */
/* -------------------------------------------------------------------------- */

/// Happens when
///   - `dialog_dive()`
///     - when leaving a node
///
/// Read in
///   - `trigger_event_handler()`
///     - If the event is not already active
///     add it to the WorldEvent list.
#[derive(Event)]
pub struct TriggerEvents(Vec<String>);

pub fn trigger_event_handler(
    mut trigger_event: EventReader<TriggerEvents>,
    mut active_world_events: ResMut<ActiveWorldEvents>,
) {
    for TriggerEvents(incomming_events) in trigger_event.iter() {
        for event_to_trigger in incomming_events {
            match WorldEvent::from_str(event_to_trigger) {
                Err(_) => {}
                Ok(event) => {
                    if !active_world_events.contains(&event) {
                        active_world_events.push(event)
                    }
                }
            }
        }
    }
}

/// Happens when
///   - `continue_monolog()`
///     - any key pressed in a monolog
///
/// Read in
///   - `change_dialog_state()`
///     - analyze the current node;
///     If the state asked is a `Content::Choice`
///     without any choice verified it won't transit to the new state.
///     Else transit and throw all trigger events,
///     while leaving the `current_node`.
#[derive(Event)]
pub struct ChangeStateEvent(pub usize);

/// Analyze the current node;
///
/// If the state asked is a `Content::Choice` without any choice verified
/// don't transit to the new state.
/// Else transit and throw all trigger events.
pub fn change_dialog_state(
    mut change_state_event: EventReader<ChangeStateEvent>,
    current_interlocutor: Res<CurrentInterlocutor>,
    mut dialogs: ResMut<DialogMap>,
    active_world_events: Res<ActiveWorldEvents>,

    mut trigger_event: EventWriter<TriggerEvents>,
) {
    for ChangeStateEvent(new_state) in change_state_event.iter() {
        if let Some(interlocutor) = current_interlocutor.interlocutor {
            if let Some((current_state, ref dialog)) = dialogs.get_mut(&interlocutor) {
                if let Some(current_node) = dialog.get(new_state) {
                    let new_state_is_available = match current_node.content() {
                        Content::Choices(choices) => {
                            let mut at_least_one_is_verified = false;
                            for choice in choices {
                                if choice.is_verified(
                                    None,
                                    active_world_events
                                        .iter()
                                        .map(|x| x.to_string())
                                        .collect::<Vec<String>>(),
                                ) {
                                    // transit if at least on verified
                                    at_least_one_is_verified = true;
                                    break;
                                }
                            }
                            at_least_one_is_verified
                        }
                        Content::Monolog { .. } => true,
                    };

                    if new_state_is_available {
                        *current_state = *new_state;
                        trigger_event.send(TriggerEvents(current_node.trigger_event().to_vec()));
                    }
                }
            }
        }
    }
}

/// # Purpose
///
/// When the dialog file implied in the talk is changed,
/// updates the panels' content.
///
/// # Process
///
/// check the current node from the interlocutor
/// - this is a monolog
///   - change the resource monolog
/// - this is a set of choices
///   - Player Choice
///     - display only the verified choice to the button choice
///   - NPC Choice
///     - Randomly choose without display anything and ask to change state instantly
pub fn update_dialog_panel(
    current_interlocutor: Res<CurrentInterlocutor>,
    active_world_events: Res<ActiveWorldEvents>,
    dialogs: Res<DialogMap>,

    mut current_monolog: ResMut<Monolog>,
    mut player_choices_query: Query<(Entity, &mut ButtonChoice, &mut Visibility)>,

    mut change_state_event: EventWriter<ChangeStateEvent>,
    mut reset_event: EventWriter<ResetDialogBoxEvent>,
) {
    if current_interlocutor.is_some() && (current_interlocutor.is_changed() || dialogs.is_changed())
    {
        // info!("UpdateDialogPanel");
        let interlocutor = current_interlocutor.interlocutor.unwrap();
        if let Some(&(current_state, ref dialog)) = dialogs.get(&interlocutor) {
            // info!("current_state: {}", current_state);
            match dialog.get(&current_state) {
                None => {
                    current_monolog.texts = Vec::new();
                    for (_, _, mut visibility) in &mut player_choices_query {
                        *visibility = Visibility::Hidden;
                    }
                }
                Some(current_node) => {
                    match current_node.content() {
                        Content::Monolog {
                            text,
                            exit_state: _,
                        } => {
                            // println!("{text:#?}");
                            current_monolog.texts = text.clone();
                            current_monolog.source = current_node.source().to_string();

                            // Clear the previous choice if there is any
                            for (_, _, mut visibility) in &mut player_choices_query {
                                *visibility = Visibility::Hidden;
                            }
                        }
                        Content::Choices(choices) => {
                            if current_node.source() == &"Player".to_string() {
                                // replace current by the new set of choices
                                let mut verified_choices = Vec::<(usize, String)>::new();

                                for choice in choices.iter() {
                                    if choice.is_verified(
                                        None,
                                        active_world_events
                                            .iter()
                                            .map(|x| x.to_string())
                                            .collect::<Vec<String>>(),
                                    ) {
                                        // info!(
                                        //     "{} -> {}",
                                        //     choice.text().to_owned(),
                                        //     *choice.exit_state()
                                        // );
                                        verified_choices
                                            .push((*choice.exit_state(), choice.text().to_owned()));
                                    }
                                }

                                for (button_entity, mut button_infos, mut visibility) in
                                    &mut player_choices_query
                                {
                                    // Here you could compare the index with `dialogs.len()` to incorpore all choice but
                                    // lock the unsatisfied choice's condition
                                    if button_infos.ui_position < verified_choices.len() {
                                        reset_event.send(ResetDialogBoxEvent {
                                            dialog_box: button_entity,
                                            text: verified_choices[button_infos.ui_position]
                                                .1
                                                .clone(),
                                        });
                                        button_infos.exit_state =
                                            verified_choices[button_infos.ui_position].0;
                                        *visibility = Visibility::Inherited;
                                    } else {
                                        *visibility = Visibility::Hidden;
                                    }
                                }

                                // // Remove all text which aren't said by the current interlocutor
                                // if current_interlocutor.is_changed() {
                                //     monolog_text.sections[0].value.clear();
                                // }
                            } else {
                                // NPC Choices
                                let mut possible_choices_index: Vec<usize> = Vec::new();
                                for choice in choices.iter() {
                                    match choice.condition() {
                                        None => possible_choices_index.push(*choice.exit_state()),
                                        Some(condition) => {
                                            if condition.is_verified(
                                                None,
                                                active_world_events
                                                    .iter()
                                                    .map(|x| x.to_string())
                                                    .collect::<Vec<String>>(),
                                            ) {
                                                possible_choices_index.push(*choice.exit_state());
                                            }
                                        }
                                    }
                                }
                                if let Some(child_index) =
                                    possible_choices_index.choose(&mut rand::thread_rng())
                                {
                                    change_state_event.send(ChangeStateEvent(*child_index))
                                } else {
                                    warn!("The NPC doesn't have a possible choice");
                                    // TODO: if `possible_choices_index.is_empty()`
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// If the resource `Monolog` is changed,
/// update the NPC/Player text.
pub fn update_monolog(
    current_monolog: Res<Monolog>,
    monolog_panel_query: Query<Entity, With<MonologPanel>>,
    // mut text_query: Query<&mut Text>,
    mut reset_event: EventWriter<ResetDialogBoxEvent>,
) {
    if current_monolog.is_changed() {
        let monolog_panel = monolog_panel_query.single();
        let dialog_box_text = match current_monolog.texts.first() {
            None => String::new(),
            Some(first) => first.to_owned(),
        };

        reset_event.send(ResetDialogBoxEvent {
            dialog_box: monolog_panel,
            text: dialog_box_text,
        });
    }
}

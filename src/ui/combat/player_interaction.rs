//! All dialog method handler related with the player input directly

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    window::PrimaryWindow,
};

use crate::{
    combat::{
        phases::TransitionPhaseEvent,
        skills::{Skill, TargetOption},
        teamwork::Recruited,
        Action, ActionCount, CombatResources, CombatState, InCombat,
    },
    constants::{
        combat::{FIRST_ALLY_ID, FIRST_ENEMY_ID, MAX_PARTY},
        ui::*,
    },
    ui::combat::{
        combat_panel::{ActionDisplayer, SkillDisplayer},
        combat_system::{Selected, Targeted},
    },
    CombatWallStage, HUDState,
};

use super::{combat_panel::MiniCharacterSheet, combat_system::UpdateUnitSelectedEvent};

/* -------------------------------------------------------------------------- */
/*                          ----- UI Components -----                         */
/* -------------------------------------------------------------------------- */

#[derive(Component)]
pub struct Hoverable;
// {
//     hovered: bool
// }

#[derive(Component)]
pub struct Hovered;

#[derive(Component)]
pub struct Clickable;

#[derive(Component)]
pub struct Clicked;

#[derive(Component)]
pub struct Draggable;
// {
//     pub dragged: bool,
//     pub dropped: bool,
// }

#[derive(Component)]
pub struct Dragged;
// old_z

#[derive(Component)]
pub struct Dropped;

#[derive(Component)]
pub struct SpriteSize {
    pub width: f32,
    pub height: f32,
}

/* -------------------------------------------------------------------------- */
/*                        ----- Global UI systems -----                       */
/* -------------------------------------------------------------------------- */

/// Change color depending of Interaction
///
/// Does not affect Skill Button
/// (color management is different: if no action no color.)
pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>, Without<Skill>),
    >,
) {
    for (interaction, mut color, _children) in &mut interaction_query {
        // let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

#[derive(Component, Default)]
pub struct ScrollingList {
    position: f32,
}

/// # Note
///
/// TODO: Unsynchronise lists (scroll only if the cursor is on the list in question)
///
/// TODO: Customise the mouse scrolling system for actions (could also work with the skills menu overflow)
/// TODO: (Prevent) Only allow scrolling on visible actions
pub fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        for (mut scrolling_list, mut style, parent, list_node) in &mut query_list {
            let items_height = list_node.size().y;
            let container_height = query_node.get(parent.get()).unwrap().size().y;

            let max_scroll = (items_height - container_height).max(0.);

            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };

            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.top = Val::Px(scrolling_list.position);
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                       ----- Specific UI systems -----                      */
/* -------------------------------------------------------------------------- */

// /// TODO: couldhave - Hover Unit = Preview Combat Page (even in SelectionTarget?)
// /// Give Hovered which is prioritized to be displayed if it exists
// pub fn hover_unit_by_mouse() {}

/// Adds the Component 'Clicked' to a valid Entity
///
/// # Note
///
/// TODO: couldhave - can drag unit just to cancel the click = avoid missclick by dragging
/// BUG: In smaller resolution, units might overlaps and you may click severals entities
/// \----> cause a block when the update of selection and absurd situation: No Selected in SelectionSkill (crash potential)
pub fn select_unit_by_mouse(
    mut commands: Commands,

    game_state: Res<State<HUDState>>,
    mut next_state: ResMut<NextState<HUDState>>,

    primary_query: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    // With<MainCamera>
    camera_q: Query<(&Camera, &GlobalTransform)>,
    buttons: Res<Input<MouseButton>>,

    selectable_unit_query: Query<
        (Entity, &Transform, &SpriteSize, &Name),
        (With<Clickable>, Without<Clicked>),
    >,
    // mut update_unit_selected_event: EventWriter<UpdateUnitSelectedEvent>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let Ok(primary) = primary_query.get_single() else {
            return;
        };
        let (camera, camera_transform) = camera_q.single();

        // check if the cursor is inside the window and get its position
        // then, ask bevy to convert into world coordinates, and truncate to discard Z
        if let Some(world_position) = primary
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        {
            // eprintln!("World coords: {}/{}", world_position.x, world_position.y);

            for (unit, transform, sprite_size, _name) in selectable_unit_query.iter() {
                let half_width = (sprite_size.width * transform.scale.x) / 2.0;
                let half_height = (sprite_size.height * transform.scale.y) / 2.0;

                if transform.translation.x - half_width < world_position.x
                    && transform.translation.x + half_width > world_position.x
                    && transform.translation.y - half_height < world_position.y
                    && transform.translation.y + half_height > world_position.y
                {
                    if game_state.get() == &HUDState::LogCave {
                        info!("Transi LogCave -> CombatWall");
                        next_state.set(HUDState::CombatWall);
                        // return;
                    }

                    // info!("{} clicked", _name);
                    commands.entity(unit).insert(Clicked);
                    // v-- instead of --^
                    // update_unit_selected_event.send(UpdateUnitSelectedEvent(unit));

                    // prevent when clicking on overlapping entities
                    break;
                }
            }
        } else {
            // cursor is not inside the window
        }
    }
}

/// Combat logic (no display) a skill button is clicked
///
/// # Note
///
/// - `[[ui::combat::character_sheet::skill_color]]` handles the skill button color
/// - TODO: couldhave - Skill dropped
///   - To a possible target: Confirm
///   - To something else: Cancel (or just back to skill clicked)
pub fn select_skill(
    mut combat_resources: ResMut<CombatResources>,
    combat_state: Res<CombatState>,
    combat_wall_state: Res<State<CombatWallStage>>,

    mut interaction_query: Query<
        (&Interaction, &Skill),
        (Changed<Interaction>, With<Button>, With<SkillDisplayer>),
    >,

    unit_selected_query: Query<(Entity, &Name, &ActionCount), With<Selected>>,
    mut transition_phase_event: EventWriter<TransitionPhaseEvent>,
) {
    if combat_wall_state.get() == &CombatWallStage::Preparation {
        return;
    }
    // TOTEST: Why does this Query triggered in a phase transi ?
    // Maybe because since the spawn the change didn't get treated ?
    for (interaction, skill) in &mut interaction_query {
        // XXX: Tempo the command which give a Selected after cancel_input a selfcast skill in SelectionCaster
        if unit_selected_query.get_single().is_err() {
            warn!("No Selected in SelectionSkill");
            continue;
        }

        // if this system can run
        // we are in SelectionSkill or SelectionTarget
        // so there is a selected unit.
        let (caster, _caster_name, action_count) = unit_selected_query.single();

        if interaction == &Interaction::Pressed && action_count.current != 0 {
            // In the `CombatWallStage::Preparation`, you can't select skill

            // BUG: XXX: Weird "Bug" Event/HUDState related handle
            // Prevent the Trigger of the "double press"
            if let Some(last_action) = combat_resources.history.last() {
                if last_action.skill == skill.clone() && last_action.targets.is_none() {
                    // warn!("Same Skill Selected Event handled twice");
                    continue;
                }
            }

            // Change last action saved to the new skill selected
            if combat_state.clone() == CombatState::SelectionTarget {
                info!("Skill changed for {}", skill.name);
                // we already wrote the waiting skill in the actions history
                // cause we're in the TargetSelection phase

                let last_action = combat_resources.history.last_mut().unwrap();
                // FIXME: CLARIFICATION NEEDED - Select Bam/Swing instantly into select solo will create two action "solo"
                // caster stay the same
                last_action.skill = skill.clone();
                last_action.targets = None;

                // This transitionEvent will trigger all the verification about skill selected (selfcast, etc)
                transition_phase_event.send(TransitionPhaseEvent(CombatState::SelectionTarget));

                // info!("DEBUG: action = {} do {} to None", caster_name, skill.name);

                // info!("rewrite last action");
            } else {
                transition_phase_event.send(TransitionPhaseEvent(CombatState::SelectionTarget));

                let action = Action::new(caster, skill.clone(), None);
                combat_resources.history.push(action);

                // info!("DEBUG: action = {} do {} to None", _caster_name, skill.name);
                // info!("new action");
            }

            info!("Skill {} selected", skill.name);
        }
    }
}

#[derive(Component)]
pub struct EndOfTurnButton;

/// # Note
///
/// TODO: feature - Confirm The EndOfTurn
/// BUG: End of turn in SelectionSkill: trigger a double press
/// @see [`ui::player_interaction::confirm_action_button()`] to check: correct target number
pub fn end_of_turn_button(
    mut combat_resources: ResMut<CombatResources>,

    mut interaction_query: Query<
        (&Interaction, &Children),
        (Changed<Interaction>, With<Button>, With<EndOfTurnButton>),
    >,

    mut text_query: Query<&mut Text>,

    mut transition_phase_event: EventWriter<TransitionPhaseEvent>,
) {
    for (interaction, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                if let Some(last_action) = combat_resources.history.last() {
                    if !last_action.is_correct(combat_resources.number_of_fighters.clone()) {
                        combat_resources.history.pop();
                    }
                } else {
                    // allow pass with no action in the history
                }

                // Pressed
                info!("End of Turn - Requested");

                transition_phase_event.send(TransitionPhaseEvent(CombatState::AIStrategy));

                text.sections[0].value = "Next".to_string();
            }
            Interaction::Hovered => {
                text.sections[0].value = "Can't Undo".to_string();
            }
            Interaction::None => {
                text.sections[0].value = "End of Turn".to_string();
            }
        }
    }
}

/// If the user press 'esc',
/// depending of the phase we're in,
/// will undo the previous input (predicted, not real undo)
///
/// If in LogCave, just transi to CombatWall.
///
/// # Note
///
/// Many operation are processed in `combat::phases::phase_transition()`.
///
/// Can be an [Exclusive System](https://github.com/bevyengine/bevy/blob/1c5c94715cb17cda5ae209eef12a938501de90b5/examples/ecs/ecs_guide.rs#L198)
pub fn cancel_last_input(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,

    game_state: Res<State<HUDState>>,
    mut next_state: ResMut<NextState<HUDState>>,

    mut combat_resources: ResMut<CombatResources>,
    combat_state: Res<CombatState>,

    selected_unit_query: Query<(Entity, &Name), With<Selected>>,
    mut caster_query: Query<(Entity, &mut ActionCount)>,

    mut transition_phase_event: EventWriter<TransitionPhaseEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if game_state.get() == &HUDState::LogCave {
            info!("Transi LogCave -> CombatWall");
            next_state.set(HUDState::CombatWall);
            return;
        }

        let current_phase = combat_state.clone();
        info!("Esc in {:?}", current_phase);

        match current_phase {
            CombatState::SelectionSkill | CombatState::BrowseEnemySheet => {
                let (selected, name) = selected_unit_query.single();

                commands.entity(selected).remove::<Selected>();
                info!("{} is no longer selected", name);

                transition_phase_event.send(TransitionPhaseEvent(CombatState::SelectionCaster));
            }
            CombatState::SelectionCaster | CombatState::SelectionTarget => {
                // Remove last targeted and modify the last action
                match combat_resources.history.last_mut() {
                    None => {
                        if current_phase == CombatState::SelectionTarget {
                            warn!("In TargetSelectionPhase, it should have at least one action");
                            // DEBUG: Error Handle
                            transition_phase_event
                                .send(TransitionPhaseEvent(CombatState::SelectionCaster));
                        } else {
                            // Nothing to undo
                        }
                    }
                    Some(ref mut last_action) => {
                        // give the last_action.caster the selected component
                        if let Ok((selected, name)) = selected_unit_query.get_single() {
                            if selected != last_action.caster {
                                commands.entity(selected).remove::<Selected>();
                                info!(
                                    "{}:{:?} was selected over our last caster {:?}",
                                    name, selected, last_action.caster
                                );
                            } else {
                                info!("{}:{:?} is selected", name, selected);
                            }
                        }
                        // XXX: this command take too long to be processed if we transi to SelectionSkill
                        // [Hard Sync Point](https://github.com/bevyengine/bevy/issues/1613) required to stop waiting the command to be executed
                        // IDEA: New TempoPhase to wait for commands to be processed.
                        commands.entity(last_action.caster).insert(Selected);
                        info!("{:?} is now selected", last_action.caster);

                        match &mut last_action.targets {
                            None => {
                                combat_resources.history.pop();

                                transition_phase_event
                                    .send(TransitionPhaseEvent(CombatState::SelectionSkill));
                            }
                            Some(ref mut targets) => {
                                let old_target = targets.pop().unwrap();
                                commands.entity(old_target).remove::<Targeted>();
                                if targets.is_empty() {
                                    last_action.targets = None;
                                }

                                if current_phase == CombatState::SelectionCaster {
                                    let mut action_count = caster_query
                                        .get_component_mut::<ActionCount>(last_action.caster)
                                        .unwrap();
                                    action_count.current += 1;
                                    info!("action given back, left: {}", action_count.current);

                                    match last_action.skill.target_option {
                                        TargetOption::OneSelf
                                        | TargetOption::All
                                        | TargetOption::AllAlly
                                        | TargetOption::AllEnemy => {
                                            combat_resources.history.pop();

                                            transition_phase_event.send(TransitionPhaseEvent(
                                                CombatState::SelectionSkill,
                                            ));
                                        }
                                        _ => {
                                            transition_phase_event.send(TransitionPhaseEvent(
                                                CombatState::SelectionTarget,
                                            ));
                                        }
                                    }
                                } else {
                                    // In SelectionTarget, the action_count should be correct.
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

/// Button interaction system for ActionDisplayer,
/// In the Initiative Vertical Bar.
///
/// # Behavior
///
/// - Clicked
/// put the one clicked as last (downward the action to be accessed easly)
/// - TODO: Hover Action
/// Visualize action effect
///
/// # Note
///
/// TODO: feat UI - simplify deleting an action by adding a cross to do so.
pub fn action_button(
    mut commands: Commands,
    mut combat_resources: ResMut<CombatResources>,

    mut interaction_query: Query<
        (&Interaction, &ActionDisplayer),
        (Changed<Interaction>, With<Button>),
    >,
    selected_query: Query<Entity, With<Selected>>,
    targeted_query: Query<Entity, With<Targeted>>,
    mut transition_phase_event: EventWriter<TransitionPhaseEvent>,
) {
    for (interaction, action_displayer) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                info!("Action {} clicked", action_displayer.0);

                if combat_resources.history.len() <= action_displayer.0 {
                    warn!(
                        "Action {} is visible even if it shouldn't: {}/{}",
                        action_displayer.0,
                        action_displayer.0,
                        combat_resources.history.len()
                    )
                } else if let Some(last_action) = combat_resources.history.last() {
                    // don't bother to do anything if there is only one action
                    // or if the action clicked was already the last
                    if 1 != combat_resources.history.len()
                        && action_displayer.0 + 1 != combat_resources.history.len()
                    {
                        if !last_action.is_correct(combat_resources.number_of_fighters.clone()) {
                            info!("Abort current action (wasn't complete)");
                            combat_resources.history.pop().unwrap();
                        }

                        // use of remove() to preserve order
                        let action = combat_resources.history.remove(action_displayer.0);
                        combat_resources.history.push(action.clone());

                        // --- Clean Up ---
                        if let Ok(selected) = selected_query.get_single() {
                            if action.clone().caster != selected {
                                commands.entity(selected).remove::<Selected>();
                            }
                        }
                        commands.entity(action.clone().caster).insert(Selected);

                        transition_phase_event
                            .send(TransitionPhaseEvent(CombatState::SelectionSkill));

                        for targeted in targeted_query.iter() {
                            commands.entity(targeted).remove::<Targeted>();
                        }
                    }
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                               Character Sheet                              */
/* -------------------------------------------------------------------------- */

/// TODO: CouldHave - Visual - Zoom in on characterSheet (or just focus)
/// TODO: CouldHave - create a cross button to close it with the mouse (atm there the cancel input: `Esc`)
pub fn mini_character_sheet_interact(
    mini_character_sheets_interaction_query: Query<
        (&Interaction, &MiniCharacterSheet),
        (Changed<Interaction>, Without<Button>),
    >,
    combat_units_query: Query<(Entity, &InCombat)>,
    mut select_event: EventWriter<UpdateUnitSelectedEvent>,
) {
    for (interaction, sheet_number) in mini_character_sheets_interaction_query.iter() {
        match interaction {
            Interaction::Pressed => {
                let mut found = false;
                // OPTIMIZE: the id search (a hash table ? (separated from CharacterSheetElements which represent the unique big CS))
                for (fighter, id) in combat_units_query.iter() {
                    // TOTEST: Should it deref auto ?
                    if id.0 == sheet_number.0 {
                        select_event.send(UpdateUnitSelectedEvent(fighter));
                        found = true;
                        break;
                    }
                }
                // cause no .len() for query
                // or use `let allies = allies_query.iter(&world).collect::<Vec<Entity>>();`
                if !found {
                    // DEBUG: Write a better log (link with the charactersheet entity) = zzzz
                    warn!("No fighter associated with {}", sheet_number.0);
                }
            }
            Interaction::Hovered => {
                // TODO: smooth slight zoom
            }
            Interaction::None => {}
        }
    }
}

/// TODO: Browse among sheets (arrows), especially for Enemy Sheets
pub fn browse_character_sheet(
    keys: Res<Input<KeyCode>>,
    combat_resources: Res<CombatResources>,
    // DEBUG: Print the Phase if no selected
    combat_phase: Res<CombatState>,

    selected_unit_query: Query<&InCombat, With<Selected>>,
    unselected_ally_units_query: Query<(Entity, &InCombat), (With<Recruited>, Without<Selected>)>,
    unselected_enemy_units_query: Query<
        (Entity, &InCombat),
        (Without<Recruited>, Without<Selected>),
    >,

    mut select_event: EventWriter<UpdateUnitSelectedEvent>,
) {
    // XXX: Tempo the phase transi after cancel_input in SelectionSkill/BrowseEnemySheet
    if selected_unit_query.get_single().is_err() {
        warn!("No Selected in {:?}", combat_phase);
        return;
    }

    // TODO: CouldHave - UI Inputs - Hold press handle: `.pressed()`
    // IDEA: UI Inputs - The Pack Of Scrolls could keep the last enemy selected
    if keys.any_just_pressed([KeyCode::Left, KeyCode::Right]) {
        let selected_id = selected_unit_query.single();
        let next_id = if keys.just_pressed(KeyCode::Right) {
            if selected_id.0 == combat_resources.number_of_fighters.ally.total - 1 {
                FIRST_ALLY_ID
            } else if selected_id.0
                == combat_resources.number_of_fighters.enemy.total + MAX_PARTY - 1
            {
                FIRST_ENEMY_ID
            } else {
                selected_id.0 + 1
            }
        } else if selected_id.0 == FIRST_ALLY_ID {
            combat_resources.number_of_fighters.ally.total - 1
        } else if selected_id.0 == FIRST_ENEMY_ID {
            combat_resources.number_of_fighters.enemy.total + MAX_PARTY - 1
        } else {
            selected_id.0 - 1
        };

        if next_id == selected_id.0 {
            return;
        }

        // OPTIMIZE: the id search (a hash table ? (separated from CharacterSheetElements which represent the unique big CS))
        if next_id < MAX_PARTY {
            for (fighter, id) in unselected_ally_units_query.iter() {
                if id.0 == next_id {
                    select_event.send(UpdateUnitSelectedEvent(fighter));
                    break;
                }
            }
        } else {
            for (fighter, id) in unselected_enemy_units_query.iter() {
                if id.0 == next_id {
                    select_event.send(UpdateUnitSelectedEvent(fighter));
                    break;
                }
            }
        }
    }
}

// TODO: PostDemo - equip stuffs

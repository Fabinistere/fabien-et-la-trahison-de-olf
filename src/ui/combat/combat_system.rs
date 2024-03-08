use bevy::prelude::*;

use crate::{
    combat::{
        phases::TransitionPhaseEvent, skills::TargetOption, teamwork::Reputation, AlterationStatus,
        CombatResources, CombatState, CurrentAlterations, InCombat,
    },
    constants::combat::{alteration::SIZE_ALTERATION_ICON, MAX_PARTY},
    ui::combat::{combat_panel::CombatStateDisplayer, player_interaction::Clicked},
};

use super::log_cave::{
    ActionHistoryDisplayer, ActionsLogsDisplayer, HUDLog, LastActionHistoryDisplayer,
};

/* -------------------------------------------------------------------------- */
/*                                UI Components                               */
/* -------------------------------------------------------------------------- */

#[derive(Component)]
pub struct ButtonTargeting;

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct Targeted;

#[derive(Component)]
pub struct HpMeter;

#[derive(Component)]
pub struct MpMeter;

/// Current Action History
#[derive(Resource, Debug, Reflect, Deref, DerefMut, Clone)]
pub struct ActionHistory(pub String);

/// Last turn Action History
#[derive(Resource, Debug, Reflect, Deref, DerefMut, Clone)]
pub struct LastTurnActionHistory(pub String);

/// Logs Action History
#[derive(Resource, Debug, Reflect, Deref, DerefMut, Clone)]
pub struct ActionsLogs(pub String);

/// DOC
#[derive(Event)]
pub struct UpdateUnitSelectedEvent(pub Entity);

/// DOC
#[derive(Event)]
pub struct UpdateUnitTargetedEvent(pub Entity);

/* -------------------------------------------------------------------------- */
/*                                 UI Systems                                 */
/* -------------------------------------------------------------------------- */

/// # Note
pub fn caster_selection(
    mut commands: Commands,

    clicked_units_query: Query<(Entity, &Name), (With<Clicked>, With<InCombat>)>,

    mut update_unit_selected_event: EventWriter<UpdateUnitSelectedEvent>,
) {
    for (entity, _name) in clicked_units_query.iter() {
        update_unit_selected_event.send(UpdateUnitSelectedEvent(entity));

        commands.entity(entity).remove::<Clicked>();
        // info!("DEBUG: {} remove clicked to be selected", _name);
    }
}

/// # Note
pub fn target_selection(
    mut commands: Commands,

    targetable_unit_query: Query<(Entity, &Name), (With<Clicked>, With<InCombat>)>,

    mut update_unit_targeted_event: EventWriter<UpdateUnitTargetedEvent>,
) {
    for (entity, _name) in targetable_unit_query.iter() {
        update_unit_targeted_event.send(UpdateUnitTargetedEvent(entity));

        commands.entity(entity).remove::<Clicked>();
        // info!("DEBUG: {} remove clicked to be targeted", _name);
    }
}

/* -------------------------------------------------------------------------- */
/*                                 UI Updates                                 */
/* -------------------------------------------------------------------------- */

/// Event Handler of UpdateUnitSelectedEvent
///
/// There can only be one entity (ally or enemy) selected.
///
/// # Note
///
/// FIXME: Multiple Entity can be selected if clicked simultaneous (a break would work ?)
/// REFACTOR: Directly Manage Clicked Entity in the update systems (instead of event) (replacing straight forward didn't work)
pub fn update_selected_unit(
    mut event_query: EventReader<UpdateUnitSelectedEvent>,

    mut commands: Commands,

    selected_unit_query: Query<Entity, (With<Selected>, With<InCombat>)>,
    combat_units_query: Query<&InCombat>,
    mut transition_phase_event: EventWriter<TransitionPhaseEvent>,
) {
    for UpdateUnitSelectedEvent(clicked) in event_query.iter() {
        if let Ok(selected) = selected_unit_query.get_single() {
            if selected != *clicked {
                commands.entity(selected).remove::<Selected>();
                // info!("{:?} was selected", selected);
            }
        }
        commands.entity(*clicked).insert(Selected);
        // info!("{:?} is now selected", *clicked);

        // REFACTOR: ? - test ID or test if Recruited or not
        let id = combat_units_query.get(*clicked).unwrap();
        if id.0 < MAX_PARTY {
            transition_phase_event.send(TransitionPhaseEvent(CombatState::SelectionSkill));
        } else {
            transition_phase_event.send(TransitionPhaseEvent(CombatState::BrowseEnemySheet));
        }
    }
}

/// Event Handler of UpdateUnitSelectedEvent.
/// Will accept or not a target depending of the skill currently selected.
///
/// # Note
///
/// REFACTOR: ? - maybe merge Targeted with Selected
/// Differentiation only when selecting a skill
pub fn update_targeted_unit(
    mut commands: Commands,
    mut combat_resources: ResMut<CombatResources>,

    mut event_query: EventReader<UpdateUnitTargetedEvent>,

    unit_selected_query: Query<(Entity, &Reputation), With<Selected>>,
    combat_units_query: Query<(Entity, &Name, &Reputation), With<InCombat>>,

    mut transition_phase_event: EventWriter<TransitionPhaseEvent>,
) {
    for UpdateUnitTargetedEvent(clicked) in event_query.iter() {
        match combat_units_query.get(*clicked) {
            Err(e) => warn!("The entity targeted is invalid: {:?}", e),
            Ok((character, target_name, target_team)) => {
                // BUG: ?
                let last_action = combat_resources.history.last_mut().unwrap();

                // Is it a correct target ?
                match last_action.skill.target_option {
                    TargetOption::Ally(_) => {
                        let (_, caster_team) = unit_selected_query.single();
                        if !target_team.in_the_same_team(caster_team) {
                            info!("The target is not an ally");
                            continue;
                        }
                    }
                    TargetOption::Enemy(_) => {
                        let (_, caster_team) = unit_selected_query.single();
                        if target_team.in_the_same_team(caster_team) {
                            info!("The target is not an enemy");
                            continue;
                        }
                    }
                    TargetOption::AllyButSelf(_) => {
                        let (caster, caster_team) = unit_selected_query.single();
                        if !target_team.in_the_same_team(caster_team) || character == caster {
                            info!("The target is not an ally or is the caster");
                            continue;
                        }
                    }
                    _ => {}
                }

                commands.entity(character).insert(Targeted);
                info!("{} targeted", target_name);

                // Possibility to target multiple depending to the skill selected
                last_action.targets = match last_action.targets.clone() {
                    None => {
                        // Number of target = max targetable
                        match last_action.skill.target_option {
                            TargetOption::Ally(1)
                            | TargetOption::Enemy(1)
                            | TargetOption::OneSelf => transition_phase_event
                                .send(TransitionPhaseEvent(CombatState::default())),
                            _ => {}
                        }
                        Some(vec![character])
                    }
                    Some(mut targets) => {
                        match last_action.skill.target_option {
                            TargetOption::Ally(number)
                            | TargetOption::Enemy(number)
                            | TargetOption::AllyButSelf(number) => {
                                // Only work if we can target muiltiple times one entity
                                // or if the number of available target is < number asked
                                // TODO: can target less if = the max possible

                                match targets.len().cmp(&number) {
                                    std::cmp::Ordering::Less => targets.push(character),
                                    std::cmp::Ordering::Equal => {
                                        targets.push(character);
                                        transition_phase_event
                                            .send(TransitionPhaseEvent(CombatState::default()));
                                    }
                                    std::cmp::Ordering::Greater => {
                                        warn!(
                                            "Error! The number of target is exceeded {}/{:?}",
                                            targets.len(),
                                            last_action.skill.target_option
                                        );
                                        while targets.len() > number {
                                            commands
                                                .entity(targets.pop().unwrap())
                                                .remove::<Targeted>();
                                        }
                                    }
                                }
                            }
                            // managed by phase_transition() or select_skill()
                            TargetOption::OneSelf
                            | TargetOption::AllAlly
                            | TargetOption::AllEnemy
                            | TargetOption::All => {}
                        }
                        Some(targets)
                    }
                };
            }
        }
    }
}

/// TODO: already done? - Update Alterations' icons on characters
///
/// Changed for duration / Added and RemovalDetection for just anim
pub fn update_alterations_status(
    mut commands: Commands,
    asset_server: Res<AssetServer>,

    changed_alterations_query: Query<
        (&CurrentAlterations, &Transform, &Children, &Name),
        (Changed<CurrentAlterations>, With<InCombat>),
    >,
) {
    for (alterations, transform, children, _name) in changed_alterations_query.iter() {
        // info!("{} has some alterations change", _name);
        // "Reset" all alt_displayer
        // REFACTOR: We should verify it is the `AllAlterationStatuses`
        commands.entity(children[0]).despawn_descendants();
        commands.entity(children[0]).with_children(|parent| {
            for (i, alteration) in alterations.iter().enumerate() {
                parent.spawn((
                    SpriteBundle {
                        texture: asset_server.load(alteration.path_icon.clone()),
                        sprite: Sprite {
                            // anchor: bevy::sprite::Anchor::TopCenter,
                            custom_size: Some(Vec2::splat(SIZE_ALTERATION_ICON)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(
                            SIZE_ALTERATION_ICON * (i as f32)
                                - SIZE_ALTERATION_ICON * (alterations.len() as f32 / 2.),
                            -12.5,
                            transform.translation.z,
                        )),
                        ..default()
                    },
                    Name::new(alteration.name.clone()),
                    AlterationStatus,
                ));
            }
        });
    }
}

/* -------------------------------------------------------------------------- */
/*                                   UI Logs                                  */
/* -------------------------------------------------------------------------- */

/// Display the current phase
///
/// # Note
///
/// DEBUG: update_combat_phase_displayer()
pub fn update_combat_phase_displayer(
    combat_state: Res<CombatState>,
    mut combat_state_displayer_query: Query<&mut Text, With<CombatStateDisplayer>>,
) {
    if combat_state.is_changed() {
        if let Ok(mut text) = combat_state_displayer_query.get_single_mut() {
            text.sections[0].value = format!("Combat Phase: {:?}", combat_state);
        }
    }
}

/// Display all combat logs
///
/// # Note
///
/// DEBUG: actions_logs_displayer()
/// IDEA: CouldHave - Character's Event (Died, killed thingy, etc)
pub fn actions_logs_displayer(
    actions_logs: Res<ActionsLogs>,
    log_cave_just_created_query: Query<Entity, Added<HUDLog>>,

    mut actions_logs_query: Query<
        &mut Text,
        (
            With<ActionsLogsDisplayer>,
            Without<LastActionHistoryDisplayer>,
            Without<ActionHistoryDisplayer>,
        ),
    >,
) {
    if actions_logs.is_changed() || !log_cave_just_created_query.is_empty() {
        if let Ok(mut actions_logs_text) = actions_logs_query.get_single_mut() {
            actions_logs_text.sections[0].value = actions_logs.clone().0;
        }
    }
}

/// Format the modified action
///
/// # Note
///
/// IDEA: Atm each turn it resets
/// TODO: Visual - Implicit the caster (and myabe their team with color)
pub fn current_action_formater(
    combat_resources: Res<CombatResources>,
    mut action_history: ResMut<ActionHistory>,

    combat_units_query: Query<(Entity, &Name), With<InCombat>>,
) {
    if combat_resources.is_changed() {
        action_history.0 = String::from("---------------\nCurrent Turn Actions:");

        for (number, action) in combat_resources.history.iter().enumerate() {
            if let Ok((_, caster_name)) = combat_units_query.get(action.caster) {
                let mut targets_name = String::new();
                match &action.targets {
                    None => targets_name = "None".to_string(),
                    Some(targets) => {
                        for (i, target) in targets.iter().enumerate() {
                            if targets.len() > 1 && i != 0 {
                                targets_name.push_str(" and ");
                            }
                            match combat_units_query.get(*target) {
                                Err(_) => targets_name.push_str("Target Err"),
                                Ok((_, name)) => targets_name.push_str(name),
                            }
                        }
                    }
                }

                let action_display = if action.initiative == -1 {
                    format!(
                        "\n{}. {} do {} to {}",
                        number + 1,
                        caster_name,
                        action.skill.name,
                        targets_name
                    )
                } else {
                    format!(
                        "\n{}. {}: {} do {} to {}",
                        number + 1,
                        action.initiative,
                        caster_name,
                        action.skill.name,
                        targets_name
                    )
                };

                action_history.push_str(&action_display);
            }
        }
    }
}

/// Display all current actions
///
/// # Note
///
/// DEBUG: current_action_displayer()
pub fn current_action_displayer(
    action_history: Res<ActionHistory>,
    log_cave_just_created_query: Query<Entity, Added<HUDLog>>,

    mut action_displayer_query: Query<&mut Text, With<ActionHistoryDisplayer>>,
) {
    if action_history.is_changed() || !log_cave_just_created_query.is_empty() {
        if let Ok(mut action_displayer_text) = action_displayer_query.get_single_mut() {
            action_displayer_text.sections[0].value = action_history.clone().0;
        }
    }
}

/// Display the last turn actions
///
/// # Note
///
/// DEBUG: last_action_displayer()
pub fn last_action_displayer(
    last_action_history: Res<LastTurnActionHistory>,
    log_cave_just_created_query: Query<Entity, Added<HUDLog>>,
    mut last_action_displayer_query: Query<&mut Text, With<LastActionHistoryDisplayer>>,
) {
    if last_action_history.is_changed() || !log_cave_just_created_query.is_empty() {
        if let Ok(mut last_action_displayer_text) = last_action_displayer_query.get_single_mut() {
            last_action_displayer_text.sections[0].value = last_action_history.clone().0;
        }
    }
}

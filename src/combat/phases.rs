use bevy::prelude::*;
use rand::Rng;

use crate::{
    combat::{
        alterations::{Alteration, AlterationAction},
        skills::{SkillExecutionQueue, TargetOption},
        stats::{Hp, Initiative, Mana, Shield},
        Action, ActionCount, CombatResources, CombatState, CurrentAlterations, InCombat,
    },
    ui::combat::{
        combat_panel::{CharacterSheet, CharacterSheetElements},
        combat_system::{ActionHistory, ActionsLogs, LastTurnActionHistory, Selected, Targeted},
    },
    HUDState,
};

use super::{skills::SkillToExecute, teamwork::Reputation};

/* -------------------------------------------------------------------------- */
/*                    ----- Transitions Between Phase -----                   */
/* -------------------------------------------------------------------------- */

/// Whenever:
/// - A system ask for a phase transition/change
///
/// Read by:
/// - combat::phases::phase_transition()
///   - Determine which action to be taken,
///   accordingly with (/w.r.t.) to the phase we're currently in,
///   and the phase we want to transit.
#[derive(Event)]
pub struct TransitionPhaseEvent(pub CombatState);

/// Action manager, about phase transition.
/// And Change phase afterall
pub fn phase_transition(
    mut transition_phase_event: EventReader<TransitionPhaseEvent>,

    mut commands: Commands,
    hud_state: Res<State<HUDState>>,
    mut combat_resources: ResMut<CombatResources>,
    current_combat_state: Res<State<CombatState>>,
    mut next_combat_state: ResMut<NextState<CombatState>>,

    mut selected_units_query: Query<Entity, (With<Selected>, With<InCombat>)>,
    targeted_unit_query: Query<(Entity, &Name), With<Targeted>>,
    mut combat_unit_query: Query<(Entity, &mut ActionCount, &Hp, &Reputation), With<InCombat>>,

    mut actions_logs: ResMut<ActionsLogs>,
    action_history: Res<ActionHistory>,
    mut last_action_history: ResMut<LastTurnActionHistory>,

    character_sheet_elements: Res<CharacterSheetElements>,
    mut character_sheet_query: Query<&mut Visibility, With<CharacterSheet>>,
) {
    for TransitionPhaseEvent(phase_requested) in transition_phase_event.iter() {
        let mut next_phase = phase_requested;

        let default_state = CombatState::default();

        match (current_combat_state.get(), phase_requested) {
            (CombatState::SelectionCaster, CombatState::SelectionSkill) => {
                // Might be a cancel action or just a caster being selected
            }
            (CombatState::SelectionSkill, CombatState::SelectionSkill) => {
                // FIXME: there is still some Targeted - While switching Caster to caster after the creation of a action
            }
            (
                CombatState::SelectionSkill | CombatState::SelectionTarget,
                CombatState::SelectionTarget,
            ) => {
                let last_action = combat_resources.history.last_mut().unwrap();

                // - Select Skill in SelectionSkill
                // - Changing Skill while being in SelectionTarget
                if last_action.targets.is_none() {
                    // remove from previous entity the targeted component
                    for (targeted, _) in targeted_unit_query.iter() {
                        commands.entity(targeted).remove::<Targeted>();
                    }

                    // ------ ActionCount ------

                    let _ = selected_units_query.get(last_action.caster).unwrap();
                    let mut action_count = combat_unit_query
                        .get_component_mut::<ActionCount>(last_action.caster)
                        .unwrap();

                    match last_action.skill.target_option {
                        TargetOption::OneSelf
                        | TargetOption::AllAlly
                        | TargetOption::AllEnemy
                        | TargetOption::All => {
                            action_count.current -= 1;
                            info!("action left: {}", action_count.current);

                            next_phase = if action_count.current > 0 {
                                &CombatState::SelectionSkill
                            } else {
                                &default_state
                            };
                        }
                        _ => {}
                    }

                    // ------ Targets ------

                    let caster_team = combat_unit_query
                        .get_component::<Reputation>(last_action.caster)
                        .unwrap();

                    match last_action.skill.target_option {
                        TargetOption::OneSelf => {
                            last_action.targets = Some(vec![last_action.caster]);
                        }
                        TargetOption::AllAlly => {
                            let mut targets: Vec<Entity> = Vec::new();
                            for (entity, _, hp, team) in combat_unit_query.iter() {
                                if hp.current > 0 && team.in_the_same_team(caster_team) {
                                    targets.push(entity);
                                }
                            }
                            last_action.targets = Some(targets);
                        }
                        TargetOption::AllEnemy => {
                            let mut targets: Vec<Entity> = Vec::new();
                            for (entity, _, hp, team) in combat_unit_query.iter() {
                                if hp.current > 0 && !team.in_the_same_team(caster_team) {
                                    targets.push(entity);
                                }
                            }
                            last_action.targets = Some(targets);
                        }
                        TargetOption::All => {
                            let mut targets: Vec<Entity> = Vec::new();
                            for (entity, _, hp, _) in combat_unit_query.iter() {
                                if hp.current > 0 {
                                    targets.push(entity);
                                }
                            }
                            last_action.targets = Some(targets);
                        }
                        _ => {}
                    }
                } else {
                    // WARNING: If we implement TargetOption, do not throw phaseTransiEvent if unauthorized
                    // - Target a entity and there is more to choose left (S.Target -> S.Target)
                }
            }
            // (CombatState::SelectionTarget, CombatState::default())
            (CombatState::SelectionTarget, CombatState::SelectionCaster) => {
                // If there is still some action left for the current caster,
                // skip SelectionCaster (The previous will still have the comp `Selected`)
                let last_action = combat_resources.history.last().unwrap();
                let _ = selected_units_query.get(last_action.caster).unwrap();
                let mut action_count = combat_unit_query
                    .get_component_mut::<ActionCount>(last_action.caster)
                    .unwrap();

                action_count.current -= 1;
                info!("action left: {}", action_count.current);

                next_phase = if action_count.current > 0 {
                    info!("S.Target to S.Caster bypass to S.Skills");
                    &CombatState::SelectionSkill
                } else {
                    &default_state
                };
                // in SelectionSkill we can click another caster to switch
            }

            /* -------------------------------------------------------------------------- */
            /*                              Cancel Transition                             */
            /* -------------------------------------------------------------------------- */
            (CombatState::SelectionSkill, CombatState::SelectionCaster) => {}
            (CombatState::BrowseEnemySheet, CombatState::SelectionCaster) => {}
            (CombatState::SelectionCaster, CombatState::SelectionTarget) => {
                /* - If the action.targets == None: bypass to SelectionSkill
                 * - ElseIf the action was a selfcast: bypass to SelectionSkill
                 * - Else: no bypass - SelectionTarget (rm the last one, still the last action IN)
                 */
            }

            /* -------------------------------------------------------------------------- */
            /*                                 End of Turn                                */
            /* -------------------------------------------------------------------------- */
            (_, CombatState::AIStrategy) => {
                // TODO: Warning if there is still action left
                // XXX: this is a safeguard preventing from double click the `end_of_turn` (wasn't a pb back there)
                if combat_resources.history.is_empty() {
                    info!("End of Turn - Refused (no action)");
                    continue;
                }
                // remove `Selected` from the last potential selected
                // DOC: will trigger all RemovedComponent queries
                if let Ok(selected) = selected_units_query.get_single_mut() {
                    commands.entity(selected).remove::<Selected>();
                }
                // remove all `Targeted`
                for (targeted, _) in targeted_unit_query.iter() {
                    commands.entity(targeted).remove::<Targeted>();
                }
                info!("End of Turn - Accepted");
            }
            (_, CombatState::RollInitiative) => {
                combat_resources.number_of_turn += 1;
            }
            (CombatState::RollInitiative, CombatState::ExecuteSkills) => {
                // --------------------- DEBUG --------------------------
                // REFACTOR: Move these ui lines somewhere else -> [[combat::phases::phase_transition()]]
                // IDEA: Push infinitly but Reverse (start of the string = recent, bottom of the cave = start of the combat)
                // TODO: CouldHave - Turn Count on logs
                actions_logs.0.push_str(&format!(
                    "\n---------------\nTurn: {}\n",
                    combat_resources.number_of_turn
                ));
                // --------------------- DEBUG --------------------------
            }

            /* -------------------------------------------------------------------------- */
            /*                                  New Turn                                  */
            /* -------------------------------------------------------------------------- */
            // replace SelectionCaster by CombatState::default()
            (CombatState::AlterationsExecution, CombatState::SelectionCaster) => {
                // IDEA: add this history into a full-log to permit the player to see it.

                // --------------------- DEBUG --------------------------
                // Save the Sorted Initiative Action Historic
                last_action_history.0 = action_history
                    .clone()
                    .0
                    .replace("Current Turn Actions:", "Last Turn Actions:");
                // --------------------- DEBUG --------------------------

                // Reset the action history
                combat_resources.history = Vec::new();

                // Reset all ActionCounter/Limit
                for (_, mut action_count, _, _) in combat_unit_query.iter_mut() {
                    action_count.current = action_count.base;
                }
            }
            _ => {}
        }

        if hud_state.get() == &HUDState::CombatWall {
            // TODO: CouldHave - Dynamic Input: AutoSwitch Selection to avoid repetitive inpleasant task ("go to next caster")
            let mut character_sheet_visibility = character_sheet_query
                .get_mut(character_sheet_elements.character_sheet.unwrap())
                .unwrap();
            *character_sheet_visibility = if next_phase == &CombatState::SelectionCaster {
                Visibility::Hidden
            } else if current_combat_state.get() == &CombatState::SelectionCaster {
                Visibility::Inherited
            } else {
                *character_sheet_visibility
            };
        }

        // info!(
        //     "Phase: {:?} to {:?} (was requested: {:?})",
        //     combat_state.clone(),
        //     next_phase.clone(),
        //     phase_requested.clone(),
        // );
        next_combat_state.set(next_phase.clone());
    }
}

/* -------------------------------------------------------------------------- */
/*                                Phase Actions                               */
/* -------------------------------------------------------------------------- */

// TODO: ShouldHave - Visual - Display mutable change (dmg, heal) (on the field)

/// # Note
///
/// DOC
pub fn execute_alteration(
    mut character_query: Query<(
        &mut Hp,
        &mut Mana,
        &mut Shield,
        &mut CurrentAlterations,
        &Name,
    )>,

    mut transition_phase_event: EventWriter<TransitionPhaseEvent>,
) {
    for (mut hp, mut mp, mut shield, mut alterations, name) in character_query.iter_mut() {
        let mut new_alterations_vector: Vec<Alteration> = Vec::new();
        for alteration in alterations.iter_mut() {
            info!("DEBUG: Execute Alteration: {} on {}", alteration.name, name);
            // info!(
            //     "duration/turnCount - {}/{}",
            //     alteration.duration, alteration.turn_count
            // );
            if alteration.duration > 0 {
                alteration.duration -= 1;
                alteration.turn_count += 1;
                info!(
                    "DEBUG: new duration: {}", // /{}
                    alteration.duration        //, alteration.turn_count
                );
                new_alterations_vector.push(alteration.clone());
            }

            match alteration.action {
                AlterationAction::StatsFlat | AlterationAction::StatsPercentage => {
                    // occurs only the first time
                    if alteration.turn_count != 0 {
                        continue;
                    }
                }
                _ => {}
            }
            match alteration.action {
                AlterationAction::Dots => {
                    hp.current += alteration.hp;
                    mp.current += alteration.mana;
                    shield.0 += alteration.shield;
                }
                // The action of StatsPercentage will not trigger if its tunn_count != 0
                AlterationAction::StatsPercentage | AlterationAction::PercentageAsDots => {
                    if alteration.hp != 0 {
                        hp.current *= alteration.hp;
                    }
                    if alteration.mana != 0 {
                        mp.current *= alteration.mana;
                    }
                    if alteration.shield != 0 {
                        shield.0 *= alteration.shield;
                    }
                    // if alteration.initiative != 0 {
                    //     initiative.0 *= alteration.initiative;
                    // }
                    if alteration.turn_count != 0 {
                        // At each turn, we increment/decrement the alteration's stats
                        // ----- EX: +10% attack/turn -----
                        // t0: alt.attck = 10;  When the alteration is inserted
                        // t1: 10 + 10/1;       When the first altPhase occurs
                        // ...
                        // t5: 10 + 10/1 + 20/2 + 30/3 + 40/4;
                        alteration.attack += alteration.attack / alteration.turn_count;
                        alteration.attack_spe += alteration.attack_spe / alteration.turn_count;
                        alteration.defense += alteration.defense / alteration.turn_count;
                        alteration.defense_spe += alteration.defense_spe / alteration.turn_count;
                    }
                }
                AlterationAction::StatsFlat => {
                    // no action, the alteration being still in the entity contains all the info.
                }
            }
        }
        // update the set of alteration
        alterations.0 = new_alterations_vector;
    }

    transition_phase_event.send(TransitionPhaseEvent(CombatState::default()));
}

/// Roll for each entity a d100 ranged into +-20 initiative
/// Also Display the final score
///
/// Sort the result in a nice table
/// In case of egality: pick the higher initiative boyo to be on top
pub fn roll_initiative(
    combat_units_query: Query<(&Initiative, &CurrentAlterations), With<InCombat>>,
    mut combat_resources: ResMut<CombatResources>,

    mut transition_phase_event: EventWriter<TransitionPhaseEvent>,
) {
    let mut initiatives: Vec<Action> = Vec::new();

    for action in combat_resources.history.iter_mut() {
        let caster = action.caster;
        // REFACTOR: how the initiative is calculated
        let skill_init = action.skill.initiative;

        match combat_units_query.get(caster) {
            Err(e) => warn!("Invalid Caster are in the History: {}", e),
            Ok((base_init, alterations)) => {
                let mut current_init = base_init.0;

                // ---- Alterations Rules ----
                for alteration in alterations.iter() {
                    if alteration.action == AlterationAction::StatsFlat {
                        current_init += alteration.initiative;
                    }
                }
                // ---- Calculus ----

                // BUG: When applying multiple times Swiftness can lead to an empty range

                let calculated_init = if current_init - 20 <= 0 {
                    rand::thread_rng().gen_range(0..current_init + 20)
                } else if current_init == 100 {
                    100
                } else if current_init + 20 >= 100 {
                    rand::thread_rng().gen_range(current_init - 20..100)
                } else {
                    rand::thread_rng().gen_range(current_init - 20..current_init + 20)
                };

                let skill_number = if skill_init - 20 <= 0 {
                    rand::thread_rng().gen_range(0..skill_init + 20)
                } else if skill_init == 100 {
                    100
                } else if skill_init + 20 >= 100 {
                    rand::thread_rng().gen_range(skill_init - 20..100)
                } else {
                    rand::thread_rng().gen_range(skill_init - 20..skill_init + 20)
                };

                // 0 <= action.initiative <= 200

                // insert these numbers in a vector
                action.initiative = calculated_init + skill_number;
                initiatives.push(action.clone());
            }
        }
    }

    initiatives.sort();
    // Action with the higher initiative first
    initiatives.reverse();

    info!("DEBUG: Initiative: {:#?}", initiatives);

    // Update the actions history
    combat_resources.history = initiatives;

    // info!("DEBUG: history: {:?}", combat_resources.history);

    transition_phase_event.send(TransitionPhaseEvent(CombatState::PreExecuteSkills));
}

pub fn execution_phase(
    combat_resources: Res<CombatResources>,

    mut skill_execution_queue: ResMut<SkillExecutionQueue>,
    mut transition_phase_event: EventWriter<TransitionPhaseEvent>,
) {
    // The result will be pushed into a vector and processed last to first
    let mut action_history = combat_resources.history.clone();
    action_history.reverse();

    for Action {
        caster,
        skill,
        targets,
        initiative: _,
    } in action_history.iter()
    {
        match targets {
            None => warn!(
                "A Skill without any target ! \n caster: {:?} skill: {:?}",
                caster, skill
            ),
            Some(targets) => {
                for target in targets {
                    // we will do a verification anyway (skill's hp_cost)
                    // in the event handler
                    // to control that the caster is alive at the time of the execution
                    skill_execution_queue.push(SkillToExecute {
                        skill: skill.clone(),
                        caster: *caster,
                        target: *target,
                    });

                    // should be in order
                    for combo_skill in skill.skills_queue.iter() {
                        skill_execution_queue.push(SkillToExecute {
                            skill: combo_skill.clone(),
                            caster: *caster,
                            // All skills in the queue will be directed to the same target
                            target: *target,
                        });
                    }
                }
            }
        }
    }

    transition_phase_event.send(TransitionPhaseEvent(CombatState::ExecuteSkills));
}

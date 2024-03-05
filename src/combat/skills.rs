//! Implement SKILLS

use bevy::prelude::*;
use bevy_ecs::query::QueryEntityError;
// use bevy_inspector_egui::prelude::*;

use crate::{
    animations::sprite_sheet_animation::SpriteSheetIndex,
    combat::{
        alterations::*,
        stats::{Attack, AttackSpe, Defense, DefenseSpe, Hp, Mana, Shield},
    },
    constants::combat::skill::*,
    ui::combat::combat_system::ActionsLogs,
};

use super::CurrentAlterations;

#[derive(Default, Debug, Clone, PartialEq, Reflect)]
pub enum SkillType {
    Heal,
    Attack,
    AttackSpe,
    ShieldBreaker,
    Defense,
    DefenseSpe,
    Buff,
    Debuff,
    #[default]
    Pass,
    Flee,
}

/// # Note
///
/// - AllAllyButSelf
#[derive(Default, Debug, Clone, PartialEq, Reflect)]
pub enum TargetOption {
    /// Identity
    #[default]
    OneSelf,
    // Enemy
    // Enemy(usize, ClosestPosition)
    Enemy(usize),
    /// Include the identity (self)
    Ally(usize),
    /// Exclude the identity (self)
    AllyButSelf(usize),
    AllAlly,
    /// The skill affects one by one target
    AllEnemy,
    All,
    // IDEA: Any(usize) ?
}

/// Endure every stats to the target
///
/// - Negative = MALUS
/// - Positive = BONUS
#[derive(Debug, Component, Clone, PartialEq)]
pub struct Skill {
    pub skill_type: SkillType,
    /// Which side the skill is allow to target
    ///
    /// # Example
    ///
    /// - target all enemy party: TargetOption::AllEnemy
    /// - self-target: TargetOption::OneSelf
    /// - targeted heal: TargetOption::Ally(1)
    /// - small explosion: TargetOption::Enemy(1) (but with aoe: true)
    pub target_option: TargetOption,
    /// Wait for the turn delay to execute
    ///
    /// # Note
    ///
    /// Without Canalisation (can act while "waiting")
    pub turn_delay: i32,
    /// initiave: slower; faster
    ///
    /// 0 <= init <= 100
    ///
    /// # Notes
    ///
    /// REFACTOR: use `u8`
    pub initiative: i32,
    /// hp: dmg/heal to the target
    pub hp_dealt: i32,
    /// mana: consume/gain to the target
    pub mana_dealt: i32,
    /// shield: reduce/addition to the target
    ///
    /// # Note
    ///
    /// Can direct
    ///
    /// - a attack to only target shield
    /// - a bonus to regenerate/add shield
    pub shield_dealt: i32,
    /// Self-inflicted Dmg
    ///
    /// # Note
    ///
    /// Shouldn't be used for casual self-healing
    /// REFACTOR: Use `u16` to prevent healing with "cost"
    pub hp_cost: i32,
    /// The Skill's Mana cost
    pub mana_cost: i32,
    // TODO: feature - shield cost ?
    /// Debuff or Buff
    pub alterations: Vec<Alteration>,
    /// The 'list' of skills called after this one
    ///
    /// # Note
    ///
    /// Used for complex skill
    pub skills_queue: Vec<Skill>,
    pub path_icon: String,
    pub vfx_index: SpriteSheetIndex,
    pub description: String,
    pub name: String,
}

impl Default for Skill {
    fn default() -> Self {
        Skill {
            skill_type: Default::default(),
            target_option: TargetOption::OneSelf,
            turn_delay: 0,
            initiative: 0,
            hp_dealt: 0,
            hp_cost: 0,
            mana_dealt: 0,
            mana_cost: 0,
            shield_dealt: 0,
            alterations: vec![],
            skills_queue: vec![],
            description: String::from("..."),
            path_icon: String::from("textures/icons/skills-alterations/Dark_8.png"),
            vfx_index: HOLY_SPELL_02,
            name: String::from("Skill"),
        }
    }
}

/// Happens in
///   - combat::phases::execution_phase
///     - The skill animation ended, the last one in the queue
///     (descending order of action's initiative) has to be executed
///
/// Read in
///   - combat::skills::execute_shill
///     - Execute the skill with the caster's Stats
///     to the target
#[derive(Event)]
pub struct ExecuteSkillEvent;

/// Descending order queue of all Action.
/// Handle by the fx animation first into by the `combat::skills::execute_skill()`
#[derive(Resource, Default, Debug, Deref, DerefMut, Clone)]
pub struct SkillExecutionQueue {
    pub queue: Vec<SkillToExecute>,
}

#[derive(Debug, Clone)]
pub struct SkillToExecute {
    pub skill: Skill,
    pub caster: Entity,
    pub target: Entity,
}

/// Execution of the skill queue to all entity targeted
///
/// - Skill cost
/// - Multiplier Caculus
/// - Skill execution
/// - Insert all the alteration contains in the skill to the target
///   - this state
///
/// # Note
///
/// DOC
/// Carefull with default Skill value
pub fn execute_skill(
    mut execute_skill_event: EventReader<ExecuteSkillEvent>,
    mut skill_execution_queue: ResMut<SkillExecutionQueue>,

    mut combat_unit: Query<(
        &mut Hp,
        &mut Mana,
        &mut Shield,
        &Attack,
        &AttackSpe,
        &Defense,
        &DefenseSpe,
        &mut CurrentAlterations,
        &Name,
    )>,
    mut actions_logs: ResMut<ActionsLogs>,
) {
    for ExecuteSkillEvent in execute_skill_event.iter() {
        let SkillToExecute {
            skill,
            caster,
            target,
        } = skill_execution_queue.pop().unwrap();

        match combat_unit.get_many_mut([caster, target]) {
            // REFACTOR: Handle SelfCast
            Err(e) => {
                match e {
                    // SelfCast
                    QueryEntityError::AliasedMutability(_) => {
                        warn!("TODO: SelfCast is currently not implemented  {:?}", e)
                    }
                    _ => warn!("Caster and/or Target Invalid {:?}", e),
                }
            }
            Ok(
                [(
                    mut caster_hp,
                    mut caster_mp,
                    mut caster_shield,
                    caster_attack,
                    caster_attack_spe,
                    _caster_defense,
                    _caster_defense_spe,
                    caster_alterations,
                    caster_name,
                ), (
                    mut target_hp,
                    mut target_mp,
                    mut target_shield,
                    _target_attack,
                    _target_attack_spe,
                    target_defense,
                    target_defense_spe,
                    mut target_alterations,
                    target_name,
                )],
            ) => {
                info!(
                    "- DEBUG: {}, from {} to {}",
                    skill.name, caster_name, target_name
                );

                // -----------------------------------------------
                // REFACTOR: Move these ui lines somewhere else ?
                actions_logs.0.push_str(&format!(
                    "\n- {}, from {} to {}",
                    skill.name, caster_name, target_name
                ));
                // -----------------------------------------------

                let skill_executed = &skill;

                // TODO: PostDemo - turn delay?

                // ---- COST ----

                // If the caster is already deadge, stop the execution
                // TODO: MustHave - cancel the skill if the mana/shield requirement is not fully satisfied
                // ^^^^^--- in case of a other skill, just before, lower their mana/shield count

                // TODO: PostDemo - feature - reduce cost by stuff and level
                caster_hp.current -= skill_executed.hp_cost;
                caster_mp.current -= skill_executed.mana_cost;
                caster_shield.0 -= skill_executed.shield_dealt;

                // don't execute the rest if the current of the caster is < 0
                if caster_hp.current <= 0 {
                    if caster_hp.current + skill_executed.hp_cost <= 0 {
                        actions_logs
                            .0
                            .push_str(&format!("\n  - Caster is already dead: {}", caster_name));
                    } else {
                        actions_logs.0.push_str(&format!(
                            "\n  - Caster killed him·herself: {}, from {} to {}",
                            caster_name,
                            caster_hp.current + skill_executed.hp_cost,
                            skill_executed.hp_cost
                        ));
                    }
                    continue;
                }

                // if the skill is pre alteration
                // ---- Alterations ----

                // target_alterations.extend(skill.clone().alterations);

                // ---- Multipliers ----

                let mut attack_multiplier: f32 = 100.;
                let mut attack_spe_multiplier: f32 = 100.;
                let mut defense_multiplier: f32 = 100.;
                let mut defense_spe_multiplier: f32 = 100.;
                let mut damage_multiplier: f32 = 100.;
                let mut heal_multiplier: f32 = 100.;

                for alt in target_alterations.iter() {
                    defense_multiplier += alt.defense as f32;
                    defense_spe_multiplier += alt.defense_spe as f32;
                    damage_multiplier += alt.damage_suffered as f32;
                    heal_multiplier += alt.heal_received as f32;
                }
                for alt in caster_alterations.iter() {
                    attack_multiplier += alt.attack as f32;
                    attack_spe_multiplier += alt.attack_spe as f32;
                    // REFACTOR: if damage_inflicted <= -100% should be 0 dmg (even if dmg_suffered > 0)
                    damage_multiplier += alt.damage_inflicted as f32;
                    heal_multiplier += alt.heal_inflicted as f32;
                }

                match skill_executed.skill_type {
                    SkillType::Heal => {
                        // IDEA: no multiplier ? based on attackspe?

                        // Can't revive with a Heal

                        if target_hp.current < 0 {
                            // round to the bottom (to i32)
                            target_hp.current +=
                                (skill_executed.hp_dealt as f32 * heal_multiplier / 100.) as i32;
                            if target_hp.current > target_hp.max {
                                target_hp.current = target_hp.max;
                            }
                        }
                    }
                    SkillType::Attack => {
                        // REFACTOR: the calculus of entity's stats in the skill execution
                        // here having 10 attack is quite inefficent
                        attack_multiplier += caster_attack.base as f32;
                        defense_multiplier += target_defense.base as f32;

                        // x * (caster_attack + caster_alt_att)% / (target_defense + target_alt_def)% * (caster_alt_dmg_inflicted - target_alt_dmg_suffered)%
                        // round to the bottom (i32)
                        let hp_dealt = (skill_executed.hp_dealt as f32
                            * (attack_multiplier / 100.)
                            * (damage_multiplier / 100.)
                            / (defense_multiplier / 100.))
                            as i32;
                        if hp_dealt > 0 {
                            info!("hp dealt: {}", hp_dealt);
                            actions_logs
                                .0
                                .push_str(&format!("\n  - hp dealt: {}", hp_dealt));
                        }

                        // ---- MP ----
                        // x + x*(caster_attack_spe)%
                        let mp_dealt = skill_executed.mana_dealt;
                        if mp_dealt > 0 {
                            info!("mp dealt: {}", mp_dealt);
                            actions_logs
                                .0
                                .push_str(&format!("\n  - mp dealt: {}", mp_dealt));
                        }

                        // ---- EXECUTION ----
                        if target_shield.0 < hp_dealt {
                            target_hp.current -= hp_dealt - target_shield.0;
                            target_shield.0 = 0;
                        } else {
                            // the shield fully tank the attack
                            target_shield.0 -= hp_dealt;
                        }
                        // neagtive hp allowed

                        target_hp.current -= mp_dealt;
                        if target_mp.current < 0 {
                            target_mp.current = 0
                        }
                    }
                    SkillType::AttackSpe => {
                        attack_spe_multiplier += caster_attack_spe.base as f32;
                        defense_spe_multiplier += target_defense_spe.base as f32;

                        // ---- HP ----
                        // x * (caster_att_spe + caster_alt_att_spe)% / (target_def_spe + target_alt_def_spe)% * (caster_alt_dmg_inflicted - target_alt_dmg_suffered)%
                        let hp_dealt = (skill_executed.hp_dealt as f32
                            * (attack_spe_multiplier / 100.)
                            * (damage_multiplier / 100.)
                            / (defense_spe_multiplier / 100.))
                            as i32;
                        if hp_dealt > 0 {
                            info!("hp dealt: {}", hp_dealt);
                            actions_logs
                                .0
                                .push_str(&format!("\n  - hp dealt: {}", hp_dealt));
                        }

                        // ---- MP ----
                        // x + x*(caster_attack_spe)%
                        let mp_dealt = (skill_executed.mana_dealt as f32 * attack_spe_multiplier
                            / 100.) as i32;
                        if mp_dealt > 0 {
                            info!("mp dealt: {}", mp_dealt);
                            actions_logs
                                .0
                                .push_str(&format!("\n  - mp dealt: {}", mp_dealt));
                        }

                        // ---- EXECUTION ----
                        target_hp.current -= hp_dealt;
                        // neagtive hp allowed

                        target_mp.current -= mp_dealt;
                        if target_mp.current < 0 {
                            target_mp.current = 0
                        }
                    }
                    // shield_dealt is neagtive when harmfull or positive when bonus
                    SkillType::ShieldBreaker | SkillType::Defense => {
                        target_shield.0 += skill_executed.shield_dealt;
                        if target_shield.0 < 0 {
                            target_shield.0 = 0
                        }
                    }
                    SkillType::DefenseSpe => {
                        // TODO: Magic Shield
                    }
                    SkillType::Pass => {
                        // force action: Pass to the target next turn
                        // IDEA: The next action of this entity is mute or the next time won't choose an action ?

                        // atm: an blank action
                    }
                    _ => {}
                }

                // if the skill is post alteration
                // ---- Alterations ----

                target_alterations.extend(skill.clone().alterations);
            }
        }
    }
}

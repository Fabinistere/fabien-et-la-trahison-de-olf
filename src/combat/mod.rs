//! Combat Implementation
//!
//! Handle
//!   - Combat Initialisation
//!   - Comabt System / Phases
//!     - Stand On
//!     - Open HUD
//!       - Display potential npc's catchphrase (*opening*)
//!       - Display Answers Choices
//!     - Select Approach in the HUD
//!       - talk
//!         - Initialize dialogue
//!       - fight
//!
//!         ```mermaid
//!         graph
//!             Observation-->ManageStuff;
//!             ManageStuff-->Observation;
//!             Observation-->Skills;
//!             Skills-->Observation;
//!             Skills-->Target;
//!             Target-->Skills;
//!             Target-->RollInitiative;
//!             RollInitiative-->Target;
//!             RollInitiative-->ExecuteSkills-->RollInitiative;
//!             ExecuteSkills-->Observation;
//!         ```
//!
//!     - Reward-s (gift or loot)
//!   - Combat Evasion (quit)

use bevy::prelude::*;
use std::{cmp::Ordering, time::Duration};

use crate::{characters::player::Player, constants::combat::BASE_ACTION_COUNT, ui, HUDState};
use self::{
    alterations::Alteration, skills::{Skill, SkillExecutionQueue, TargetOption}, stats::{Hp, StatBundle},
    stuff::{Equipements, Job, JobsMasteries}, teamwork::{Recruited, Reputation},
};

pub mod alteration_list;
pub mod alterations;
pub mod item_list;
pub mod phases;
pub mod skill_list;
pub mod skills;
pub mod stats;
pub mod stuff;
pub mod tactical_position;
pub mod teamwork;
pub mod weapons_list;

/// Just help to create a ordered system in the app builder
///
/// REFACTOR: Turn `Res<CombatState>` into States
#[derive(Default, SystemSet, PartialEq, Eq, Hash, Clone, Debug, Reflect, Resource)]
pub enum CombatState {
    /// TOTEST: URGENT - connect transitions
    #[default]
    NonCombat,
    /// REFACTOR: Useless atm
    Initialisation,
    AlterationsExecution,
    SelectionCaster,
    /// There is one (ally or enemy) selected, and a CS focused
    SelectionSkill,
    /// There is at least one action in the history
    /// The one selected is the exact same one from SelectionSkill
    SelectionTarget,
    /// `Strategic AI Assessment`,
    /// `Intelligent Combat Decision`,
    /// `Algorithmic Battle Planning`,
    /// `NPC Tactical Evaluation`,
    /// `Automated Decision-Making`.
    AIStrategy,
    RollInitiative,
    /// prepare the full vector of Skill to execute
    PreExecuteSkills,
    /// FX Phase (SkillsAnimation) too
    ExecuteSkills,

    BrowseEnemySheet,
    Logs,

    // ShowExecution,
    Evasion,
}

impl CombatState {
    pub fn new_turn() -> Self {
        CombatState::AlterationsExecution
    }
}

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CombatState::default())
            
            .insert_resource(SkillExecutionQueue::default())
            .init_resource::<CombatResources>()
            .init_resource::<JobsMasteries>()
            
            .add_event::<self::CombatEvent>()
            .add_event::<teamwork::RecruitmentEvent>()
            .add_event::<phases::TransitionPhaseEvent>()
            .add_event::<skills::ExecuteSkillEvent>()
            .add_event::<tactical_position::UpdateCharacterPositionEvent>()
            
            .configure_set(
                Update,
                CombatState::Initialisation
                    .run_if(in_initialisation_phase)
            )
            .configure_set(
                Update,
                CombatState::AlterationsExecution
                    .run_if(in_alteration_phase)
            )
            .configure_set(
                Update,
                CombatState::SelectionCaster
                    .run_if(in_caster_phase)
            )
            .configure_set(
                Update,
                CombatState::SelectionSkill
                    .run_if(in_skill_phase)
            )
            .configure_set(
                Update,
                CombatState::SelectionTarget
                    .run_if(in_target_phase)
            )
            .configure_set(
                Update,
                CombatState::AIStrategy
                    .run_if(in_ai_strategy_phase)
            )
            .configure_set(
                Update,
                CombatState::RollInitiative
                    .run_if(in_initiative_phase)
            )
            .configure_set(
                Update,
                CombatState::PreExecuteSkills
                    .run_if(in_pre_executive_phase)
            )
            .configure_set(
                Update,
                CombatState::ExecuteSkills
                    .run_if(in_executive_phase)
            )
            .configure_set(
                Update,
                CombatState::BrowseEnemySheet
                    .run_if(in_browsing_enemy_sheet_phase)
            )
            .configure_set(
                Update,
                CombatState::Evasion
                    .run_if(in_evasive_phase)
            )

            .add_systems(Startup, stuff::spawn_stuff)
            .add_systems(OnEnter(HUDState::CombatWall), update_number_of_fighters.before(ui::combat::combat_panel::combat_wall_setup))

            .add_systems(
                Update,
                (
                    phases::phase_transition,
                    update_number_of_fighters,
                    teamwork::recruit_event_handler,
                )
            )
            .add_systems(
                Update, 
                phases::execute_alteration
                    .in_set(CombatState::AlterationsExecution)
            )
            .add_systems(
                Update, 
                phases::roll_initiative
                    .in_set(CombatState::RollInitiative)
            )
            .add_systems(
                Update,
                (
                    phases::execution_phase,
                )
                    .in_set(CombatState::PreExecuteSkills)
            )
            .add_systems(
                Update,
                (
                    skills::execute_skill,
                )
                    .in_set(CombatState::ExecuteSkills)
            )
            ;
    }
}

/* -------------------------------------------------------------------------- */
/*                           -- Combat Components --                          */
/* -------------------------------------------------------------------------- */

/// Contains the fighter's id.
/// Used to Select a unit using the character sheet in the combat HUD.
#[derive(Component, Reflect, Default, Clone, Copy, Deref)]
pub struct InCombat(pub usize);

#[derive(Bundle)]
pub struct CombatBundle {
    pub karma: Karma,
    pub reputation: Reputation,
    pub job: Job,
    pub alterations: CurrentAlterations,
    pub skills: Skills,
    pub equipements: Equipements,
    pub action_count: ActionCount,
    /// NOTE: Maybe an `Option<TacticalPosition>` to compute it later
    pub tactical_position: TacticalPosition,

    pub stats: StatBundle,
}

impl Default for CombatBundle {
    fn default() -> Self {
        CombatBundle {
            karma: Karma(0),
            reputation: Reputation::default(),
            job: Job::default(),
            alterations: CurrentAlterations::default(),
            skills: Skills(Vec::new()),
            equipements: Equipements { weapon: None, armor: None },
            action_count: ActionCount::default(),
            tactical_position: TacticalPosition::default(),
            stats: StatBundle::default()
        }
    }
}

#[derive(Component, Default)]
pub struct Karma(pub i32);

#[derive(Component, Reflect)]
pub struct ActionCount {
    pub current: usize,
    /// Number of action given each new turn
    pub base: usize,
}

impl ActionCount {
    pub fn new(base: usize) -> Self {
        ActionCount { current: base, base }
    }
}

impl Default for ActionCount {
    fn default() -> Self {
        ActionCount { current: BASE_ACTION_COUNT, base: BASE_ACTION_COUNT }
    }
}

/// Ongoing alterations, Debuff or Buff
/// 
/// DOC: The "Current" was added to Differenciate with the simple "Alteration" - new name ?
#[derive(Default, Component, Deref, DerefMut)]
pub struct CurrentAlterations(Vec<Alteration>);

/// Marker: Child of a fighter, has as child all the alteration's icon of the fighter
/// 
/// Can be removed
#[derive(Component)]
pub struct AllAlterationStatuses;

/// Alterations are also put into an entity which is fighter's child (+ its icon)
#[derive(Component)]
pub struct AlterationStatus;

/// Basic/Natural skills own by the entity  
#[derive(Component, Deref, DerefMut)]
pub struct Skills(pub Vec<Skill>);

/* -------------------------------------------------------------------------- */
/*                         -- Position in the Group --                        */
/* -------------------------------------------------------------------------- */

#[derive(Default, Reflect, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum TacticalPlace {
    #[default]
    Left,
    Middle,
    Right,
}

#[derive(Component, Reflect, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum TacticalPosition {
    FrontLine(TacticalPlace),
    MiddleLine(TacticalPlace),
    BackLine(TacticalPlace),
}

impl Default for TacticalPosition {
    fn default() -> Self {
        TacticalPosition::FrontLine(TacticalPlace::default())
    }
}

/* -------------------------------------------------------------------------- */
/*                         -- Combat Core Operation --                        */
/* -------------------------------------------------------------------------- */

#[derive(Resource, Reflect, Debug)]
pub struct CombatResources {
    pub history: Vec<Action>,
    pub number_of_fighters: GlobalFighterStats,
    pub number_of_turn: usize,
}

impl FromWorld for CombatResources {
    fn from_world(
        world: &mut World,
    ) -> Self {
        let mut allies_query = world.query_filtered::<Entity, (With<Recruited>, With<InCombat>)>();
        let allies = allies_query.iter(world).collect::<Vec<Entity>>();

        let mut enemies_query = world.query_filtered::<Entity, (Without<Recruited>, With<InCombat>)>();
        let enemies = enemies_query.iter(world).collect::<Vec<Entity>>();

        CombatResources {
            history: Vec::new(),
            // `allies.len() + 1` to include the player
            number_of_fighters: GlobalFighterStats::new(allies.len() + 1, enemies.len()),
            number_of_turn: 0
        }
    }
}

/// TODO: Count down knockout people
#[derive(Default, Reflect, Debug, Clone)]
pub struct GlobalFighterStats {
    /// (alive, knockout)
    pub ally: FightersCount,
    /// (alive, knockout)
    pub enemy: FightersCount,
}

impl GlobalFighterStats {
    pub fn new(number_of_allies: usize, number_of_enemies: usize) -> Self {
        GlobalFighterStats {
            ally: FightersCount::new(number_of_allies),
            enemy: FightersCount::new(number_of_enemies),
        }
    }
}

/// Default = { alive: 0, total: 0 }
#[derive(Default, Reflect, Debug, Clone)]
pub struct FightersCount{
    pub alive: usize, 
    pub total: usize
}

impl FightersCount {
    pub fn new(total: usize) -> Self {
        FightersCount {
            alive: total,
            total,
        }
    }
}

#[derive(Reflect, Debug, Clone)]
pub struct Action {
    pub caster: Entity,
    pub skill: Skill,
    /// Optional only to allow selecting skill before the target
    pub targets: Option<Vec<Entity>>,
    /// From caster + skill calculus
    ///
    /// Default: -1
    pub initiative: i32,
}

// impl fmt::Display for Action {
//     fn fmt(&self, f: &mut fmt::Formatter, unit_query: Query<Entity, &Name>) -> fmt::Result {
//         match self {
//             Action {caster, skill, target} => {
//                 match unit_query.get(caster) {
//                     (_, catser_name) => {

//                     }
//                 }
//                 write!(f, "initialisation")
//             }
//         }
//     }
// }

impl Action {
    pub fn new(caster: Entity, skill: Skill, targets: Option<Vec<Entity>>) -> Action {
        Action {
            caster,
            skill,
            targets,
            initiative: -1,
        }
    }

    /// Verify if the action has the good number of target depending its skill
    /// 
    /// # Note
    ///
    /// is finished/complete/full
    pub fn is_correct(&self, number_of_fighters: GlobalFighterStats) -> bool {
        match self.skill.target_option {
            TargetOption::All => {
                match &self.targets {
                    Some(targets) => targets.len() == (number_of_fighters.enemy.alive + number_of_fighters.ally.alive),
                    None => false
                }
            }
            TargetOption::AllEnemy => {
                match &self.targets {
                    Some(targets) => targets.len() == number_of_fighters.enemy.alive,
                    None => false
                }
            }
            TargetOption::AllAlly => {
                match &self.targets {
                    Some(targets) => targets.len() == number_of_fighters.ally.alive,
                    None => false
                }
            }
            TargetOption::AllyButSelf(number) | TargetOption::Ally(number) | TargetOption::Enemy(number) => {
                match &self.targets {
                    Some(targets) => targets.len() == number,
                    None => false
                }
            }
            TargetOption::OneSelf => {
                match &self.targets {
                    Some(targets) => targets.len() == 1,
                    None => false
                }
            }
        }
    }
}

impl Ord for Action {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.initiative).cmp(&(other.initiative))
    }
}

impl PartialOrd for Action {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Action {
    /// compare with just the initiative
    fn eq(&self, other: &Self) -> bool {
        (self.initiative) == (other.initiative)
    }
}

impl Eq for Action {}

/* -------------------------------------------------------------------------- */
/*                                   Basic                                    */
/* -------------------------------------------------------------------------- */

#[derive(Component)]
pub struct FairPlayTimer {
    /// (non-repeating timer)
    /// Let the enemy go when reached/left behind
    timer: Timer,
}

impl FairPlayTimer {
    pub fn new(secs: u64) -> Self {
        FairPlayTimer {
            timer: Timer::new(Duration::from_secs(secs), TimerMode::Once),
        }
    }

    pub fn get_mut_timer(&mut self) -> &mut Timer {
        &mut self.timer
    }
}

/* -------------------------------------------------------------------------- */
/*                                   Events                                   */
/* -------------------------------------------------------------------------- */

/// # Conditions
///
/// Happens when:
///   - characters::npcs::movement::chase_management
///     - target is reach
///
/// Read in:
///   - ui::dialog_panel::create_combat_panel_on_combat_event
///     - open combat ui
#[derive(Event)]
pub struct CombatEvent {
    pub attacker: Option<Entity>,
}

/* -------------------------------------------------------------------------- */
/*                               Systems Update                               */
/* -------------------------------------------------------------------------- */

/// If there is any Hp change in the frame, update the number of fighter alive
pub fn update_number_of_fighters(
    mut combat_panel: ResMut<CombatResources>,

    // REFACTOR: Change these triggers to send an event in another system to update this one
    // Triggers
    created_units_query: Query<Entity, Added<InCombat>>,
    updated_units_query: Query<Entity, (Changed<Hp>, With<InCombat>)>,

    player_query : Query<&Hp, With<Player>>,
    ally_units_query: Query<&Hp, (With<Recruited>, Without<Player>, With<InCombat>)>,
    enemy_units_query: Query<&Hp, (Without<Recruited>, Without<Player>, With<InCombat>)>,
) {
    if !updated_units_query.is_empty() || !created_units_query.is_empty() {
        // info!("Update Combat Global Stats");
        
        let player_hp = player_query.single();

        combat_panel.number_of_fighters.ally = FightersCount::default();
        combat_panel.number_of_fighters.enemy = FightersCount::default();

        // see the discord thread about [Fabien's Death](https://discord.com/channels/692439766485958767/990369916785930300/1114261607825019031)
        if player_hp.current > 0 {
            combat_panel.number_of_fighters.ally.alive += 1;
        } else {
            warn!("Player is Dead");
        }
        combat_panel.number_of_fighters.ally.total += 1;
        
        for npc_hp in ally_units_query.iter() {
            if npc_hp.current > 0 {
                combat_panel.number_of_fighters.ally.alive += 1;
            }
            combat_panel.number_of_fighters.ally.total += 1;
        }
        for npc_hp in enemy_units_query.iter() {
            if npc_hp.current > 0 {
                combat_panel.number_of_fighters.enemy.alive += 1;
            }
            combat_panel.number_of_fighters.enemy.total += 1;
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                             -- Run Criteria --                             */
/* -------------------------------------------------------------------------- */

// REFACTOR: Change CombatState to be GlobalState (in the world)

pub fn in_initialisation_phase(combat_state: Res<CombatState>) -> bool {
    *combat_state == CombatState::Initialisation
}

pub fn in_alteration_phase(combat_state: Res<CombatState>) -> bool {
    *combat_state == CombatState::AlterationsExecution
}

pub fn in_caster_phase(combat_state: Res<CombatState>) -> bool {
    *combat_state == CombatState::SelectionCaster
}

pub fn in_skill_phase(combat_state: Res<CombatState>) -> bool {
    *combat_state == CombatState::SelectionSkill
}

pub fn in_target_phase(combat_state: Res<CombatState>) -> bool {
    *combat_state == CombatState::SelectionTarget
}

pub fn in_ai_strategy_phase(combat_state: Res<CombatState>) -> bool {
    *combat_state == CombatState::AIStrategy
}

pub fn in_initiative_phase(combat_state: Res<CombatState>) -> bool {
    *combat_state == CombatState::RollInitiative
}

pub fn in_pre_executive_phase(combat_state: Res<CombatState>) -> bool {
    *combat_state == CombatState::PreExecuteSkills
}

pub fn in_executive_phase(combat_state: Res<CombatState>) -> bool {
    *combat_state == CombatState::ExecuteSkills
}

pub fn in_browsing_enemy_sheet_phase(combat_state: Res<CombatState>) -> bool {
    *combat_state == CombatState::BrowseEnemySheet
}

pub fn in_evasive_phase(combat_state: Res<CombatState>) -> bool {
    *combat_state == CombatState::Evasion
}

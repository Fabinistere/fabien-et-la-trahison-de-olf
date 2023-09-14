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
//!

use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use std::time::Duration;

use crate::{
    characters::{
        npcs::{NPCSystems, NPC},
        player::Player,
    },
    constants::character::npcs::movement::EVASION_TIMER,
    HUDState,
};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnCombatFoesEvent>()
            .add_event::<CombatEvent>()
            .add_systems(
                Update,
                (
                    spawn_party_members.before(CombatState::Initiation),
                    enter_combat.in_set(CombatState::Initiation),
                ),
            )
            .add_systems(
                OnExit(HUDState::CombatWall),
                exit_combat
                    .in_set(CombatState::Evasion)
                    .before(CombatState::Observation),
            )
            .add_systems(
                FixedUpdate,
                (
                    freeze_in_combat.after(CombatState::Evasion),
                    fair_play_wait.after(NPCSystems::StopChase),
                ),
            );
    }
}

/// Just help to create a ordered system in the app builder
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
enum CombatState {
    Initiation,
    Observation,
    // ManageStuff,
    // SelectionSkills,
    // SelectionTarget,
    // RollInitiative,
    // ExecuteSkills,

    // ShowExecution,
    Evasion,
}

/* -------------------------------------------------------------------------- */
/*                                 Components                                 */
/* -------------------------------------------------------------------------- */

/// The reputation an entity got from one another team
#[derive(Copy, Clone, PartialEq, Eq, Component)]
pub struct Reputation {
    supreme_god: usize,
    olf: usize,
}

impl Reputation {
    pub fn new(supreme_god: usize, olf: usize) -> Self {
        Reputation { supreme_god, olf }
    }

    pub fn is_in_supreme_god_team(&self) -> bool {
        self.supreme_god > 50
    }

    pub fn is_in_olf_team(&self) -> bool {
        self.olf > 50
    }

    pub fn is_neutral(&self) -> bool {
        !self.is_in_supreme_god_team() && !self.is_in_olf_team()
    }

    /// Two neutral entity aren't in the "same" team
    pub fn in_the_same_team(&self, other: &Reputation) -> bool {
        (self.is_in_supreme_god_team() && other.is_in_supreme_god_team())
            || (self.is_in_olf_team() && other.is_in_olf_team())
    }
}

#[derive(Component)]
pub struct Karma(pub i32);

#[derive(Component)]
pub struct InCombat;

#[derive(Clone, Copy, Component)]
pub struct Leader;

/// One aggressive npc can hide 5 others.
/// This number exclude the 'leader'/representant of the grp
///
/// - Could Give info on the type of group ?
///   - (All fabicurion or else)
///
/// Min = 0
/// Max = 5
///
/// Examples :
///
/// - Fabicurion who represent a group of 3
/// - Fabicurion who represent a group of 6
#[derive(Copy, Clone, PartialEq, Eq, Component)]
pub struct GroupSize(usize);

impl GroupSize {
    /// 0 < `size` < 5
    pub fn new(size: usize) -> Self {
        if size > 5 {
            GroupSize(5)
        } else {
            GroupSize(size)
        }
    }
}

/// The player can recruted some friendly npc
/// Can be called, TeamPlayer
#[derive(Copy, Clone, PartialEq, Eq, Component)]
pub struct Recruted;

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

/// Happens when:
///   - npc::movement::pursue
///     - target is reach
/// Read in
///   - ui::dialog_panel::create_dialog_panel_on_combat_event
///     - open combat ui
///   - combat::mod::freeze_in_combat
///     - freeze all entities involved in the starting combat
#[derive(Event)]
pub struct CombatEvent {
    pub entity: Entity,
}

/// Happens when:
///   - combat::mod::combat
///     - A aggressive npc encountered the player's group
///
/// Read in:
///   - combat::mod::spawn_party_members
///     - Spawn every foes hidden behind the initial
///       aggressive npc
#[derive(Event)]
pub struct SpawnCombatFoesEvent {
    pub leader: Entity,
    pub group_size: usize,
}

/* -------------------------------------------------------------------------- */
/*                                   Systems                                  */
/* -------------------------------------------------------------------------- */

/// Decrement the fair play Timer
/// while doing other things (don't **exclude** entity With<FairPlayTimer>)
/// remove the FairPlayTimer if the entity is in the player's team
pub fn fair_play_wait(
    mut commands: Commands,

    time: Res<Time>,
    mut npc_query: Query<
        (
            Entity,
            &mut FairPlayTimer,
            &mut Velocity,
            &Reputation,
            &Name,
        ),
        (
            With<NPC>,
            // Without<InCombat>
        ),
    >,
) {
    for (npc, mut fair_play_timer, mut _rb_vel, reputation, name) in npc_query.iter_mut() {
        fair_play_timer.timer.tick(time.delta());

        // not required to control velocity because it is managed elsewhere

        // REFACTOR: compare NPC's team with Player's Team instead of a global Player Team CST
        // query player to get his TEAM (it's the player who switch team not all npc)
        if fair_play_timer.timer.finished() || reputation.is_in_supreme_god_team() {
            info!("{:?}, {} can now aggro", npc, name);

            commands.entity(npc).remove::<FairPlayTimer>();
        }
    }
}

/// Emulate the Combat phase
///
///   - Talk
///   - Fight
///
/// Freeze all entity involved
///
///   - Player
///     - all companie members (recruted)
///   - Foe who caught player
pub fn enter_combat(
    mut commands: Commands,

    mut ev_combat_enter: EventReader<CombatEvent>,
    mut ev_spawn_fabicurion: EventWriter<SpawnCombatFoesEvent>,

    mut player_query: Query<Entity, (With<Player>, Without<NPC>)>,
    mut player_companie: Query<Entity, (With<NPC>, With<Recruted>)>,
    mut foes_query: Query<(Entity, Option<&GroupSize>), (With<NPC>, Without<Recruted>)>,
) {
    for CombatEvent { entity } in ev_combat_enter.iter() {
        info!("Combat Event");
        let player = player_query.single_mut();

        commands.entity(player).insert(InCombat);

        for member in player_companie.iter_mut() {
            commands.entity(member).insert(InCombat);

            // TODO: (CANCELED) - display / spawn them in the ui
        }

        let (foe, potential_group_size) = foes_query.get_mut(*entity).unwrap();

        commands.entity(foe).insert(InCombat);

        if let Some(GroupSize(size)) = potential_group_size {
            ev_spawn_fabicurion.send(SpawnCombatFoesEvent {
                leader: foe,
                group_size: *size,
            });
        }

        // display / spawn them in the ui
        // or
        // spawn them in the temple during combat (PREFERED)
    }
}

/// For each entity in combat, freeze their movement
pub fn freeze_in_combat(mut characters_query: Query<(Entity, &mut Velocity), With<InCombat>>) {
    // TOTEST: QUESTION: Maybe be not for the member of the company
    // to let them reach the player

    for (_character, mut rb_vel) in characters_query.iter_mut() {
        rb_vel.linvel.x = 0.;
        rb_vel.linvel.y = 0.;
    }
}

/// Event Handler of SpawnCombatFoesEvent
pub fn spawn_party_members(
    // mut commands: Commands,
    mut ev_spawn_party_members: EventReader<SpawnCombatFoesEvent>,
) {
    for _ev in ev_spawn_party_members.iter() {
        // ev.group_size
        // TODO: Spawn Party Member
    }
}

/// Occurs `OnExit(HUDState::CombatWall)`
///
/// apply to all npc involved in a interaction the IdleBehavior
pub fn exit_combat(
    mut commands: Commands,

    allies_query: Query<
        (Entity, &Name),
        (
            Or<(With<Player>, (With<NPC>, With<Recruted>))>,
            With<InCombat>,
        ),
    >,

    foes_query: Query<(Entity, &Name), (With<NPC>, With<InCombat>, Without<Recruted>)>,
) {
    info!("DEBUG: Combat Exit");

    for (allie, _name) in allies_query.iter() {
        commands.entity(allie).remove::<InCombat>();
    }

    // foes AND being an enemy
    // With InCombat and Without Recruted mean that these entities are enemies.
    for (foes, _name) in foes_query.iter() {
        commands
            .entity(foes)
            .insert(FairPlayTimer::new(EVASION_TIMER));

        commands.entity(foes).remove::<InCombat>();
    }
}

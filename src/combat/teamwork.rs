use bevy_ecs::{
    component::Component,
    entity::Entity,
    event::{Event, EventReader},
    system::Commands,
};

/* -------------------------------------------------------------------------- */
/*                                 Components                                 */
/* -------------------------------------------------------------------------- */

#[derive(Clone, Copy, Component)]
pub struct Leader;

/// The player can Recruited some friendly npc
/// Can be called, TeamPlayer
#[derive(Component)]
pub struct Recruited;

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
impl Default for Reputation {
    fn default() -> Self {
        Reputation::new(50, 30)
    }
}

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

/* -------------------------------------------------------------------------- */
/*                                   Events                                   */
/* -------------------------------------------------------------------------- */

/// # Conditions
///
/// Happens when:
///   - `ui::dialog_systems::trigger_event_handler`
///     - The `WorldEvent::FollowPlayer` has been triggered.
///       The `NPC` will follow and work for the `Player`
///
/// Read in:
///   - ui::dialog_panel::create_combat_panel_on_combat_event
///     - open combat ui
#[derive(Event)]
pub struct RecruitmentEvent {
    pub npc_recruited: Entity,
}

/* -------------------------------------------------------------------------- */
/*                                   Systems                                  */
/* -------------------------------------------------------------------------- */

pub fn recruit_event_handler(
    mut commands: Commands,
    mut recruitment_events: EventReader<RecruitmentEvent>,
) {
    for RecruitmentEvent { npc_recruited } in recruitment_events.iter() {
        commands.entity(*npc_recruited).insert(Recruited);
    }
}

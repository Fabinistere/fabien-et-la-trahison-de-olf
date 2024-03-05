//! Implement all Combat stats

use bevy::prelude::*;
// use bevy_inspector_egui::prelude::*;

use crate::characters::npcs::NPC;

/// Each entity which can be involved in a combat has this Bundle
#[derive(Bundle, Default)]
pub struct StatBundle {
    pub hp: Hp,
    pub mana: Mana,
    pub shield: Shield,
    pub initiative: Initiative,
    pub attack: Attack,
    pub attack_spe: AttackSpe,
    /// Physical Resistance
    pub defense: Defense,
    /// Magical Resistance
    pub defense_spe: DefenseSpe,
}

/// ----------Hp----------
///
/// Start of the Game: 50hp -> End of the Game: 1 000hp.
///
/// Can be modified by level, item, buff, debuff, technics.
///
/// # Note
///
/// At the moment, current <= max
#[derive(Component, Reflect)]
pub struct Hp {
    pub current: i32,
    pub max: i32,
}

impl Default for Hp {
    fn default() -> Self {
        Hp {
            current: 50,
            max: 50,
        }
    }
}

// TODO: feature - a hp bar close to the entity
pub fn show_hp(npc_query: Query<(&Hp, &Name), With<NPC>>) {
    for (npc_hp, npc_name) in npc_query.iter() {
        info!(
            "DEBUG: {}'s Hp: {}/{},",
            npc_name, npc_hp.current, npc_hp.max
        );
    }
}

/// ----------Mana----------
///
/// Start of the Game: 0-100mana -> End of the Game: 10 000mana.
///
/// Can be modified by level, item, buff, debuff, technics.
///
/// # Note
///
/// At the moment, current <= max
#[derive(Component, Reflect)]
pub struct Mana {
    pub current: i32,
    pub max: i32,
}

impl Default for Mana {
    fn default() -> Self {
        Mana {
            current: 50,
            max: 50,
        }
    }
}

// TODO: feature - a mana bar close to the entity
pub fn show_mana(npc_query: Query<(&Mana, &Name), With<NPC>>) {
    for (npc_mana, npc_name) in npc_query.iter() {
        info!(
            "DEBUG: {}'s Mana: {}/{},",
            npc_name, npc_mana.current, npc_mana.max
        );
    }
}

/// ----------Shield----------
///
/// Start of the Game: 0-100shield -> End of the Game: 10 000shield.
///
/// Can be modified by level, item, buff, debuff, technics.
#[derive(Component, Deref, DerefMut, Reflect, Debug)]
pub struct Shield(pub i32);

impl Default for Shield {
    fn default() -> Self {
        Shield(0)
    }
}

/// ----------Attack----------
///
/// Start of the Game: 10-20 -> End of the Game: ~.
///
/// Can be modified by level, item, buff, debuff, technics.
///
/// This statistic is fix, it increment the martial technic's power.
///
/// # Modifiers
///
/// (base + modifier_flat) * modifer_percent%
#[derive(Component, Debug, Clone, Reflect)]
pub struct Attack {
    pub base: i32,
}

impl Default for Attack {
    fn default() -> Self {
        Attack { base: 10 }
    }
}

/// ----------Attack Spe----------
///
/// Start of the Game: 0-30 -> End of the Game: ~
///
/// Can be modified by level, item, buff, debuff, technics.
///
/// This statistic is fix, it increment the magic technic's power.
///
/// # Modifiers
///
/// (base + modifier_flat) * modifer_percent%
#[derive(Component, Debug, Clone, Reflect)]
pub struct AttackSpe {
    pub base: i32,
}

impl Default for AttackSpe {
    fn default() -> Self {
        AttackSpe { base: 0 }
    }
}

/// ----------Defense----------
///
/// Start of the Game: 0-10 -> End of the Game: ~
///
/// Can be modified by level, item, buff, debuff, technics.
///
/// This statistic has a logarithmic behavior.
///
/// Used to calculate the reduced damage (in percentage)
/// taken from basic attacks and abilities that deal physical damage.
///
/// Calculated by armor ÷ (armor + 100).
///
/// # Modifiers
///
/// (base + modifier_flat) * modifer_percent%
#[derive(Component, Debug, Clone, Reflect)]
pub struct Defense {
    pub base: i32,
}

impl Default for Defense {
    fn default() -> Self {
        Defense { base: 10 }
    }
}

/// ----------Defense Spe----------
///
/// Start of the Game: 0-10 -> End of the Game: ~
///
/// Can be modified by level, item, buff, debuff, technics.
///
/// This statistic has a logarithmic behavior.
///
/// Used to calculate the reduced damage (in percentage)
/// taken from basic attacks and abilities that deal magical damage.
///
/// Calculated by MR ÷ (MR + 100).
///
/// # Modifiers
///
/// (base + modifier_flat) * modifer_percent%
#[derive(Component, Debug, Clone, Reflect)]
pub struct DefenseSpe {
    pub base: i32,
}

impl Default for DefenseSpe {
    fn default() -> Self {
        DefenseSpe { base: 0 }
    }
}

/// ----------INITIATIVE----------
///
/// Minimun initiative: 0 -> Maximun initiative: 100
///
/// Indicate the speed of initiative, the entity has.
/// The more they has, the more likly they will start their turn first.
#[derive(Component, Clone, Deref, DerefMut, Reflect, Debug)]
pub struct Initiative(pub i32);

impl Default for Initiative {
    fn default() -> Self {
        Initiative(20)
    }
}

/// ----------ACCURACY----------
///
/// Used to calculate if the technic will hit (in percentage).
///
/// # Note
///
/// - Unsed
/// - TODO: CouldHave - Impl Accuracy
#[derive(Component, Deref, DerefMut, Reflect)]
pub struct Accuracy(pub i32);

impl Default for Accuracy {
    fn default() -> Self {
        Accuracy(95)
    }
}

/// ----------CRITICAL----------
///
/// Used to calculate if the technic will be critical (in percentage).
///
/// A Critical technic has its dmg inflicted multiplied by 300%
///
/// ONLY allow critics on hit
///
/// # Note
///
/// - Unsed
/// - TODO: CouldHave - Impl Critical
#[derive(Component, Deref, DerefMut, Reflect)]
pub struct Critical(pub i32);

impl Default for Critical {
    fn default() -> Self {
        Critical(1)
    }
}

//! List all the technic possible

use crate::combat::skills::TargetOption;

use super::alterations::{Alteration, AlterationAction};

impl Alteration {
    // TODO: ShouldHave - a Counter/Alteration and turn updated in the description
    /* -------------------------------------------------------------------------- */
    /*                            ------ Debuff ------                            */
    /* -------------------------------------------------------------------------- */

    // --- IO / InflictSuffer ---

    pub fn honte() -> Self {
        Alteration {
            action: AlterationAction::StatsPercentage,
            duration: 2,
            target_option: TargetOption::Enemy(1),
            damage_suffered: 25,
            description: String::from("+25% damage suffered for 2turns"),
            name: String::from("Honte"),
            path_icon: String::from("textures/icons/skills-alterations/Dark/Dark_9.png"),
            ..Default::default()
        }
    }

    /// Is a Buff without the debuff Honte
    /// IDEA: if have Honte -> debuff: cry and turn skip
    /// order ? or whatever
    /// IDEA: can only use Skill which attack, or Pass
    pub fn anger() -> Self {
        Alteration {
            action: AlterationAction::StatsPercentage,
            duration: 3,
            target_option: TargetOption::Enemy(1),
            damage_inflicted: 25,
            description: String::from("+25% damage inflicted for 2turns"),
            name: String::from("Anger"),
            path_icon: String::from("textures/icons/skills-alterations/Dark/Dark_11.png"),
            ..Default::default()
        }
    }

    /* -------------------------------------------------------------------------- */
    /*                             ------ Buff ------                             */
    /* -------------------------------------------------------------------------- */

    // --- IO / InflictSuffer ---

    pub fn harmonize() -> Self {
        Alteration {
            action: AlterationAction::StatsPercentage,
            duration: 2,
            target_option: TargetOption::Ally(1),
            heal_received: 25,
            description: String::from("+25% received heal for 2turns"),
            name: String::from("Harmonize"),
            path_icon: String::from("textures/icons/skills-alterations/Holy/Holy_5.png"),
            ..Default::default()
        }
    }

    // --- Heal ---

    pub fn regenerate() -> Self {
        Alteration {
            action: AlterationAction::Dots,
            duration: 3,
            target_option: TargetOption::Ally(1),
            hp: 10,
            description: String::from("10hp per turn for 3turns"),
            name: String::from("Regenerate"),
            path_icon: String::from("textures/icons/skills-alterations/Nature/Nature_9.png"),
            ..Default::default()
        }
    }

    // --- Stats ---

    pub fn swiftness() -> Self {
        Alteration {
            action: AlterationAction::StatsFlat,
            duration: 3,
            target_option: TargetOption::Ally(1),
            initiative: 30,
            description: "Grant +30initiative for 3turns".to_string(),
            name: "Swiftness".to_string(),
            path_icon: String::from("textures/icons/skills-alterations/Nature/Nature_2.png"),
            ..Default::default()
        }
    }

    pub fn hardness() -> Self {
        Alteration {
            action: AlterationAction::StatsFlat,
            duration: 3,
            target_option: TargetOption::Ally(1),
            defense: 15,
            description: "Grant +15defense for 3turns".to_string(),
            name: "Hardness".to_string(),
            path_icon: String::from("textures/icons/skills-alterations/Holy/Holy_10.png"),
            ..Default::default()
        }
    }

    /* -------------------------------------------------------------------------- */
    /*                            ------ Neutral ------                           */
    /* -------------------------------------------------------------------------- */

    pub fn stale_odour() -> Self {
        Alteration {
            // TODO: AlterationAction::Nothing
            action: AlterationAction::StatsFlat,
            duration: 2,
            target_option: TargetOption::Ally(1),
            description: "You stink to high heaven".to_string(),
            name: "Stale Odour".to_string(),
            path_icon: String::from("textures/icons/skills-alterations/Nature/Nature_6.png"),
            ..Default::default()
        }
    }
}

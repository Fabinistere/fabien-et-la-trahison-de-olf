use bevy::prelude::Name;

use super::{
    skills::Skill,
    stuff::{Equipement, SkillTiers, WeaponBundle, WeaponType},
};

impl WeaponBundle {
    pub fn bass() -> Self {
        WeaponBundle {
            name: Name::new("Bass"),
            weapon_type: WeaponType::Instrument,
            skill_tiers: SkillTiers {
                tier_2: vec![Skill::swing(), Skill::solo()],
                tier_1: vec![Skill::melody()],
                tier_0: vec![],
            },
            equipement: Equipement {
                // TODO: ownership
                owner: None,
                icon_path: String::from("textures/icons/weapons/sunsword-hs2020.png"),
            },
            ..Default::default()
        }
    }

    pub fn smallmouth_bass() -> Self {
        WeaponBundle {
            name: Name::new("Smallmouth Bass"),
            weapon_type: WeaponType::Improvised,
            skill_tiers: SkillTiers {
                tier_2: vec![Skill::fish_slam(), Skill::surf()],
                tier_1: vec![Skill::throw_the_fish()],
                tier_0: vec![Skill::eat_the_fish()],
            },
            ..Default::default()
        }
    }

    // Bocal à gros cornichons
    pub fn pickle_jar() -> Self {
        WeaponBundle {
            weapon_type: WeaponType::Improvised,
            name: Name::new("Bocal à gros cornichons"),
            skill_tiers: SkillTiers {
                tier_2: vec![Skill::jar_selfdestruction()],
                tier_1: vec![Skill::eat_a_pickle()],
                tier_0: vec![],
            },
            ..Default::default()
        }
    }
}

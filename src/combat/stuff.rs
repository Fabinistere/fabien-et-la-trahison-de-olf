//! # Stuffs
//!
//! Stuff is more than just some stats changes.
//!
//! Equip a weapon will give you a full set of skill.
//!
//! ## Content
//!
//! - Job
//! - MasteryTier (for skills)
//! - JobsMasteries (links a job with a set of MasteryTier for each skill)
//! - Equipements and weapons
//! - SkillTier

use bevy::{prelude::*, utils::HashMap};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use super::{skills::Skill, stats::StatBundle};

// --- Jobs ---

/// Class
///
/// # Note
///
/// See [Jobs' skills](https://github.com/Fabinistere/FABIENs_Brain/blob/main/FTO/Combat/FTO_Jobs.md#jobs-skills)
#[derive(Component, Reflect, Default, PartialEq, Eq, Hash, Clone, Copy, Debug, EnumIter)]
pub enum Job {
    /// The perfect job
    ///
    /// Tend to imitate other with observation
    /// Or Use Big and Stunning Illusion to feint an attack.
    /// Certain enemies are immune to such illusion.
    #[default]
    Faker,
    /// Combines well with Illusionist/Faker => Flautist
    Musician,
    MartialArt,
    Healer,
    /// The Technomancy is the research of the antique and futuristic items.
    /// They will use their mystic power and more.
    Technomancian,
    /// Special Evolution of Technomancy, unique for Flo
    Logician,
    /// SwordsMaster / Sword Fighter
    Fencer,
    // Fabien's Army
    Fabicurion,
}

#[derive(Reflect, Default, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum MasteryTier {
    /// Will use it upsidedown
    #[default]
    Zero,
    /// Know some tricks with it
    One,
    /// Master the tool
    Two,
}

/// Link a Job and a weapon with a mastery tier
///
/// # Note
///
/// This aHashMap is not designed for cryptoSecurity but for performance (from bevy)
#[derive(Resource, Reflect, Debug, Deref, DerefMut, Clone)]
pub struct JobsMasteries(pub HashMap<(Job, WeaponType), MasteryTier>);

/// Correspond with the default for the initialisation of the resource
impl FromWorld for JobsMasteries {
    fn from_world(_world: &mut World) -> Self {
        let mut jobs_mastries = JobsMasteries(HashMap::new());

        // --- Logician ---
        jobs_mastries.insert((Job::Logician, WeaponType::Cryptic), MasteryTier::Two);
        // --- Musician ---
        jobs_mastries.insert((Job::Musician, WeaponType::Instrument), MasteryTier::Two);
        jobs_mastries.insert((Job::Musician, WeaponType::Improvised), MasteryTier::Two);
        jobs_mastries.insert((Job::Musician, WeaponType::Cryptic), MasteryTier::One);
        // --- Technomancy ---
        jobs_mastries.insert((Job::Technomancian, WeaponType::Cryptic), MasteryTier::Two);
        jobs_mastries.insert(
            (Job::Technomancian, WeaponType::Instrument),
            MasteryTier::One,
        );
        // --- Fabicurion ---
        jobs_mastries.insert((Job::Fabicurion, WeaponType::Club), MasteryTier::Two);
        jobs_mastries.insert((Job::Fabicurion, WeaponType::Improvised), MasteryTier::Two);
        jobs_mastries.insert((Job::Fabicurion, WeaponType::Sword), MasteryTier::One);

        // forall not implied/inserted, create the association with MasteryTier::Zero
        for job in Job::iter() {
            for weapon_type in WeaponType::iter() {
                if !jobs_mastries.contains_key(&(job, weapon_type)) {
                    jobs_mastries.insert((job, weapon_type), MasteryTier::Zero);
                }
            }
        }

        jobs_mastries
    }
}

// --- Character Component ---

/// On a character
#[derive(Component, Default, Reflect)]
pub struct Equipements {
    /// Max One
    pub weapon: Option<Entity>,
    pub armor: Option<Entity>,
}

// --- Equipement Components ---

#[derive(Default, Bundle)]
pub struct WeaponBundle {
    pub equipement: Equipement,
    pub weapon_type: WeaponType,
    pub skill_tiers: SkillTiers,
    /// TODO: Link these stats with something (weapon skill / owner's stats / etc)
    pub stats: StatBundle,
    pub name: Name,
}

/// Contains the user if in use (in case of weapons are droped in the floor)
#[derive(Component)]
pub struct Equipement {
    pub owner: Option<Entity>,
    pub icon_path: String,
}

impl Default for Equipement {
    fn default() -> Self {
        Equipement {
            owner: None,
            icon_path: String::from("textures/icons/weapons/fish_01b.png"),
        }
    }
}

#[derive(Component, Default, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect, EnumIter)]
pub enum WeaponType {
    #[default]
    Improvised,
    Instrument,
    Cryptic,
    /// Mace
    Club,
    Sword,
}

/// A Job being
///
/// - tier 2 have access to all tier 1 and tier 0
/// - tier 1 have access to all tier 0
/// - tier 0 only have access to its tier
#[derive(Default, Component)]
pub struct SkillTiers {
    pub tier_2: Vec<Skill>,
    pub tier_1: Vec<Skill>,
    pub tier_0: Vec<Skill>,
    // pub extra: Vec<Skill>,
}

pub fn spawn_stuff(mut commands: Commands) {
    // Bocal à gros cornichons
    commands.spawn(WeaponBundle::pickle_jar());
}

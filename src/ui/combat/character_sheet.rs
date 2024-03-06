//! Display the Character Sheet
//! All Stats and Skills

use bevy::prelude::*;

use crate::{
    characters::{FabiensInfos, PersonalInfos},
    combat::{
        alterations::AlterationAction,
        skills::Skill,
        stats::{Attack, AttackSpe, Defense, DefenseSpe, Hp, Initiative, Mana, Shield},
        stuff::{Equipement, Equipements, Job, JobsMasteries, MasteryTier, SkillTiers, WeaponType},
        ActionCount, CurrentAlterations, InCombat, Skills,
    },
    constants::ui::*,
    ui::combat::{
        combat_panel::{SkillBar, SkillDisplayer, TargetMeter},
        combat_system::{HpMeter, MpMeter, Selected, Targeted},
    },
};

use super::combat_panel::{CharacterSheetElements, Portrait, WeaponDisplayer};

/* -------------------------------------------------------------------------- */
/*                                   Headers                                  */
/* -------------------------------------------------------------------------- */

/// Update all character sheet with the infos of each ally
/// (Name, Sprite, Title, Job).
///
/// Only run once at the start of the combat.
/// These infos won't change afterwards
/// TODO: unless the sprite? - can be deadge too
pub fn update_headers(
    asset_server: Res<AssetServer>,
    fabiens_infos: Res<FabiensInfos>,

    newly_selected_unit_query: Query<
        (&Job, &Name, &TextureAtlasSprite),
        (Added<Selected>, With<InCombat>),
    >,
    character_sheet: Res<CharacterSheetElements>,

    mut portrait_query: Query<&mut UiImage, With<Portrait>>,
    mut text_query: Query<&mut Text>,
) {
    // sort recruted by Recruted(usize) to keep the order straight
    for (job, name, _sprite) in newly_selected_unit_query.iter() {
        let mut portrait = portrait_query
            .get_mut(character_sheet.portrait.unwrap())
            .unwrap();
        let [mut fabien_name_text, mut title_text, mut job_text] = text_query
            .get_many_mut([
                character_sheet.name.unwrap(),
                character_sheet.title.unwrap(),
                character_sheet.job.unwrap(),
            ])
            .unwrap();

        // portrait.index = sprite.index;
        if let Some(PersonalInfos { title, sprite_path }) = fabiens_infos.get(&name.to_string()) {
            title_text.sections[0].value = title.to_string();
            portrait.texture = asset_server.load(sprite_path);
        } else {
            warn!("{} Not Found/Associated in the FabienDataBase", name);
            title_text.sections[0].value = "Fabien".to_string();
            portrait.texture = asset_server.load("textures/character/idle/idle_Fabien_Loyal.png");
        };
        job_text.sections[0].value = format!("{:?}", job);
        fabien_name_text.sections[0].value = name.to_string();
    }
}

/* -------------------------------------------------------------------------- */
/*                                    Stats                                   */
/* -------------------------------------------------------------------------- */

/// # Note
///
/// TODO: Add Damage Multiplier (Suffered/Inflicted)
pub fn update_caster_stats_panel(
    character_sheet: Res<CharacterSheetElements>,

    selected_unit_query: Query<
        (
            &Hp,
            &Mana,
            &Shield,
            &Initiative,
            &Attack,
            &AttackSpe,
            &Defense,
            &DefenseSpe,
            &CurrentAlterations,
            // &Equipements,
        ),
        (
            Or<(Added<Selected>, Changed<Hp>, Changed<Mana>)>,
            With<Selected>,
            With<InCombat>,
        ),
    >,

    mut text_query: Query<&mut Text>,
) {
    if let Ok((
        hp,
        mp,
        shield,
        initiative,
        attack,
        attack_spe,
        defense,
        defense_spe,
        alterations,
        // equipments,
    )) = selected_unit_query.get_single()
    {
        let [mut hp_text, mut mp_text, mut shield_text, mut initiative_text, mut attack_text, mut attack_spe_text, mut defense_text, mut defense_spe_text] =
            text_query
                .get_many_mut([
                    character_sheet.health.unwrap(),
                    character_sheet.mana.unwrap(),
                    character_sheet.shield.unwrap(),
                    character_sheet.initiative.unwrap(),
                    character_sheet.attack.unwrap(),
                    character_sheet.attack_spe.unwrap(),
                    character_sheet.defense.unwrap(),
                    character_sheet.defense_spe.unwrap(),
                ])
                .unwrap();

        let mut attack_percentage: f32 = 100.;
        let mut attack_spe_percentage: f32 = 100.;
        let mut defense_percentage: f32 = 100.;
        let mut defense_spe_percentage: f32 = 100.;

        for alt in alterations.iter() {
            match alt.action {
                AlterationAction::StatsPercentage | AlterationAction::StatsFlat => {
                    attack_percentage += alt.attack as f32;
                    attack_spe_percentage += alt.attack_spe as f32;
                    defense_percentage += alt.defense as f32;
                    defense_spe_percentage += alt.defense_spe as f32;
                }
                _ => {}
            }
        }

        hp_text.sections[0].value = format!("Health: {}/{}", hp.current, hp.max);
        mp_text.sections[0].value = format!("Mana: {}/{}", mp.current, mp.max);
        shield_text.sections[0].value = format!("Shield: {}", shield.0);
        initiative_text.sections[0].value = format!("Initiative: {}", (initiative.0 as f32));
        attack_text.sections[0].value = format!(
            "Attack: {}",
            (attack.base as f32) * attack_percentage / 100.
        );
        attack_spe_text.sections[0].value = format!(
            "AttackSpe: {}",
            (attack_spe.base as f32) * attack_spe_percentage / 100.
        );
        defense_text.sections[0].value = format!(
            "Defense: {}",
            (defense.base as f32) * defense_percentage / 100.
        );
        defense_spe_text.sections[0].value = format!(
            "DefenseSpe: {}",
            (defense_spe.base as f32) * defense_spe_percentage / 100.
        );
    }
}

/// # Note
///
/// DEBUG
/// XXX: A proper clone of update_caster_stats_panel but just for target instead of caster
pub fn update_target_stats_panel(
    targeted_query: Query<
        (&Name, &Hp, &Mana),
        (
            Or<(Changed<Targeted>, Changed<Hp>, Changed<Mana>)>,
            With<Targeted>,
            With<InCombat>,
        ),
    >,

    mut target_removals: RemovedComponents<Targeted>,

    mut hp_query: Query<&mut Text, (With<HpMeter>, Without<MpMeter>, With<TargetMeter>)>,
    mut mp_query: Query<&mut Text, (Without<HpMeter>, With<MpMeter>, With<TargetMeter>)>,
) {
    for (name, hp, mana) in targeted_query.iter() {
        let mut hp_text = hp_query.single_mut();
        let mut mp_text = mp_query.single_mut();

        let hp_display = format!("Target {} hp: {}", name, &hp.current.to_string());
        hp_text.sections[0].value = hp_display;

        let mp_display = format!("Target {} mp: {}", name, &mana.current.to_string());
        mp_text.sections[0].value = mp_display;
    }

    for _entity in target_removals.iter() {
        let mut hp_text = hp_query.single_mut();
        let mut mp_text = mp_query.single_mut();

        let hp_display = String::from("Target hp: ??");
        hp_text.sections[0].value = hp_display;

        let mp_display = String::from("Target mp: ??");
        mp_text.sections[0].value = mp_display;
    }
}

/* -------------------------------------------------------------------------- */
/*                               Weapon Section                               */
/* -------------------------------------------------------------------------- */

/// Update the sprite with the weapon of the Selected,
/// at each equipement change.
///
/// REFACTOR: ? - Only run once at the start of the combat. These infos won't change afterwards (Demo)
pub fn update_weapon_displayer(
    asset_server: Res<AssetServer>,

    selected_unit_query: Query<
        &Equipements,
        (
            Or<(Changed<Equipements>, Added<Selected>)>,
            With<Selected>,
            With<InCombat>,
        ),
    >,
    character_sheet: Res<CharacterSheetElements>,

    mut weapon_displayer_query: Query<(&mut UiImage, &mut Visibility), With<WeaponDisplayer>>,
    weapon_query: Query<&Equipement, With<WeaponType>>,
) {
    if let Ok(Equipements { weapon, armor: _ }) = selected_unit_query.get_single() {
        let (mut weapon_image, mut visibility) = weapon_displayer_query
            .get_mut(character_sheet.weapon.unwrap())
            .unwrap();

        match weapon {
            None => *visibility = Visibility::Hidden,
            Some(weapon_entity) => {
                *visibility = Visibility::Inherited;
                let Equipement {
                    owner: _,
                    icon_path,
                } = weapon_query.get(*weapon_entity).unwrap();

                weapon_image.texture = asset_server.load(icon_path)
            }
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                                 Skill Menu                                 */
/* -------------------------------------------------------------------------- */

/// Determine the visibility of the 6 skills
///
/// Update these values when the entity selected changed
///
/// # Note
///
/// REFACTOR: Maybe find some new ways to sequence these affectations better
/// OPTIMIZE: Trigger Only one time
pub fn skill_visibility(
    mut selection_removal_query: RemovedComponents<Selected>,
    selected_unit_query: Query<(&Equipements, &Skills, &Job), (Added<Selected>, With<InCombat>)>,
    character_sheet: Res<CharacterSheetElements>,

    weapon_query: Query<(&WeaponType, &SkillTiers), With<Equipement>>,

    jobs_masteries_resource: Res<JobsMasteries>,

    skill_menu: Query<&Children>,
    mut skill_bar_query: Query<
        (&SkillDisplayer, &mut Skill, &mut Visibility, &Children),
        With<SkillBar>,
    >,
    mut text_query: Query<&mut Text>,
) {
    for _ in selection_removal_query.iter() {
        // ------ Reset all Skill ------
        for (_, mut skill, mut visibility, children) in skill_bar_query.iter_mut() {
            // --- Text ---
            let mut text = text_query.get_mut(children[0]).unwrap();
            text.sections[0].value = "Pass".to_string();
            *skill = Skill::pass();

            // --- Visibility ---
            *visibility = Visibility::Hidden;
        }
    }

    // ------ Set the visibility w.r.t. the newly selected caster ------
    if let Ok((Equipements { weapon, armor: _ }, skills, job)) = selected_unit_query.get_single() {
        let base_skills = skill_menu
            .get(character_sheet.base_skills.unwrap())
            .unwrap();

        /* -------------------------------------------------------------------------- */
        /*                               Base Skill Bar                               */
        /* -------------------------------------------------------------------------- */

        for entity in base_skills.iter() {
            let (skill_number, mut skill, mut visibility, children) =
                skill_bar_query.get_mut(*entity).unwrap();

            // --- Text ---
            let mut text = text_query.get_mut(children[0]).unwrap();

            if skill_number.0 < skills.len() {
                // --- Visibility ---
                *visibility = Visibility::Inherited;

                text.sections[0].value = skills[skill_number.0].clone().name;
                *skill = skills[skill_number.0].clone();
            } else {
                // --- Visibility ---
                *visibility = Visibility::Hidden;

                // vv-- "useless" --vv
                text.sections[0].value = "Pass".to_string();
                *skill = Skill::pass();
            }
        }

        /* -------------------------------------------------------------------------- */
        /*                              Weapon Skill Bar                              */
        /* -------------------------------------------------------------------------- */

        match weapon {
            None => {
                // info!("No weapon on the entity")
            }
            Some(weapon_entity) => {
                if let Ok((
                    weapon_type,
                    SkillTiers {
                        tier_2,
                        tier_1,
                        tier_0,
                    },
                )) = weapon_query.get(*weapon_entity)
                {
                    let tier_2_skills = skill_menu
                        .get(character_sheet.tier_2_skills.unwrap())
                        .unwrap();
                    let tier_1_skills = skill_menu
                        .get(character_sheet.tier_1_skills.unwrap())
                        .unwrap();
                    let tier_0_skills = skill_menu
                        .get(character_sheet.tier_0_skills.unwrap())
                        .unwrap();

                    let mastery_tier: Option<&MasteryTier> =
                        jobs_masteries_resource.get(&(*job, *weapon_type));

                    // info!(
                    //     "Job {:?} is {:?} with {:?}",
                    //     *job, mastery_tier, *weapon_type
                    // );

                    if mastery_tier.is_none() {
                        warn!("Job {:?} is not associated with {:?}", *job, *weapon_type);
                    }

                    if Some(MasteryTier::Two) == mastery_tier.copied() {
                        for tier_2_displayer in tier_2_skills.iter() {
                            let (skill_number, mut skill, mut visibility, children) =
                                skill_bar_query.get_mut(*tier_2_displayer).unwrap();
                            // --- Text ---
                            let mut text = text_query.get_mut(children[0]).unwrap();

                            if skill_number.0 < tier_2.len() {
                                // --- Visibility ---
                                *visibility = Visibility::Inherited;

                                text.sections[0].value = tier_2[skill_number.0].clone().name;
                                *skill = tier_2[skill_number.0].clone();
                            } else {
                                // --- Visibility ---
                                *visibility = Visibility::Hidden;

                                // vv-- "useless" --vv
                                text.sections[0].value = "Pass".to_string();
                                *skill = Skill::pass();
                            };
                        }
                    }

                    if Some(MasteryTier::Two) == mastery_tier.copied()
                        || Some(MasteryTier::One) == mastery_tier.copied()
                    {
                        for tier_1_displayer in tier_1_skills.iter() {
                            let (skill_number, mut skill, mut visibility, children) =
                                skill_bar_query.get_mut(*tier_1_displayer).unwrap();
                            // --- Text ---
                            let mut text = text_query.get_mut(children[0]).unwrap();

                            if skill_number.0 < tier_1.len() {
                                // --- Visibility ---
                                *visibility = Visibility::Inherited;

                                text.sections[0].value = tier_1[skill_number.0].clone().name;
                                *skill = tier_1[skill_number.0].clone();
                            } else {
                                // --- Visibility ---
                                *visibility = Visibility::Hidden;

                                // vv-- "useless" --vv
                                text.sections[0].value = "Pass".to_string();
                                *skill = Skill::pass();
                            };
                        }
                    }

                    // Two, One, Zero or None
                    // None => warn!("There is no combinaison between {:?} and {:?}", job, weapon_type),
                    // if _ == mastery_tier {
                    // ----- Tier0 Skill Bar -----
                    for tier_0_displayer in tier_0_skills.iter() {
                        let (skill_number, mut skill, mut visibility, children) =
                            skill_bar_query.get_mut(*tier_0_displayer).unwrap();
                        // --- Text ---
                        let mut text = text_query.get_mut(children[0]).unwrap();

                        if skill_number.0 < tier_0.len() {
                            // --- Visibility ---
                            *visibility = Visibility::Inherited;

                            text.sections[0].value = tier_0[skill_number.0].clone().name;
                            *skill = tier_0[skill_number.0].clone();
                        } else {
                            // --- Visibility ---
                            *visibility = Visibility::Hidden;

                            // vv-- "useless" --vv
                            text.sections[0].value = "Pass".to_string();
                            *skill = Skill::pass();
                        };
                    }
                }
            }
        }
    }
}

/// Updates the color of the skill,
/// whenever the Selected entity changed or their ActionCount change
pub fn skill_color(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (With<Interaction>, With<Button>, With<SkillDisplayer>),
    >,

    changed_selected_query: Query<
        (Entity, &Name, &ActionCount),
        (With<Selected>, Or<(Added<Selected>, Changed<ActionCount>)>),
    >,
) {
    if let Ok((_, _, action_count)) = changed_selected_query.get_single() {
        for (interaction, mut color) in &mut interaction_query {
            match *interaction {
                Interaction::Pressed => {
                    *color = if action_count.current == 0 {
                        INACTIVE_BUTTON.into()
                    } else {
                        PRESSED_BUTTON.into()
                    };
                }
                Interaction::Hovered => {
                    *color = if action_count.current == 0 {
                        INACTIVE_HOVERED_BUTTON.into()
                    } else {
                        HOVERED_BUTTON.into()
                    };
                }
                Interaction::None => {
                    *color = if action_count.current == 0 {
                        INACTIVE_BUTTON.into()
                    } else {
                        NORMAL_BUTTON.into()
                    };
                }
            }
        }
    }
}

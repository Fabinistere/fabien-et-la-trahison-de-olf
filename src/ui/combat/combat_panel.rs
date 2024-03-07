use bevy::prelude::*;
use bevy_tweening::{lens::UiPositionLens, Animator, EaseFunction, Tween};
use std::time::Duration;

use crate::{
    characters::player::Player,
    combat::{
        skills::Skill,
        stats::{Attack, AttackSpe, Defense, DefenseSpe, Hp, Initiative, Mana, Shield},
        stuff::Job,
        CombatEvent, CombatResources, CombatState, InCombat, Recruted,
    },
    constants::{
        combat::{FIRST_ALLY_ID, FIRST_ENEMY_ID},
        ui::{style::*, *},
    },
    ui::{dialog::dialog_systems::CurrentInterlocutor, HUDWallsSection},
    CombatWallStage, HUDState,
};

use super::{initiative_bar::InitiativeBar, log_cave::CombatLogResources};

/* -------------------------------------------------------------------------- */
/*                                UI Resources                                */
/* -------------------------------------------------------------------------- */

/// DOC : new name ? CombatWallAssetsResources
#[derive(Resource)]
pub struct CombatWallResources {
    pub base_combat_wall: Handle<Image>,
    pub pack_of_scroll: Handle<Image>,
    pub weapons: Handle<Image>,
    pub allies_scroll: Vec<Handle<Image>>,
}

impl FromWorld for CombatWallResources {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let mut allies_scroll = Vec::new();
        for i in 0..6 {
            allies_scroll.push(asset_server.load(format!(
                "textures/UI/HUD/combat/wall/sheets/sheet_{}.png",
                i
            )));
        }

        CombatWallResources {
            base_combat_wall: asset_server.load("textures/UI/HUD/combat/wall/base_combat_wall.png"),
            pack_of_scroll: asset_server.load("textures/UI/HUD/combat/wall/scrolls_pack.png"),
            weapons: asset_server.load("textures/UI/HUD/combat/wall/stuffs.png"),
            allies_scroll,
        }
    }
}

/// DOC : new name ? CharacterSheetAssetsResources
#[derive(Resource)]
pub struct CharacterSheetAssetsResources {
    // pub base_scroll: Handle<Image>,
    // pub base_headers: Handle<Image>,
    /// DOC: rename to base_scroll
    pub base_full_scroll: Handle<Image>,
    pub top_left_corner: Handle<Image>,
    pub weapon_frame: Handle<Image>,
}

impl FromWorld for CharacterSheetAssetsResources {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        CharacterSheetAssetsResources {
            // base_scroll: asset_server.load("textures/UI/HUD/combat/character_sheet/base_scroll.png"),
            // base_headers: asset_server.load("textures/UI/HUD/combat/character_sheet/base_headers.png"),
            base_full_scroll: asset_server
                .load("textures/UI/HUD/combat/character_sheet/base_full_scroll.png"),
            top_left_corner: asset_server
                .load("textures/UI/HUD/combat/character_sheet/top_left_corner.png"),
            weapon_frame: asset_server.load("textures/UI/border/border_05_nobackground.png"),
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                                UI Components                               */
/* -------------------------------------------------------------------------- */

#[derive(Component)]
pub struct Ladder;

#[derive(Component)]
pub struct CombatWall;

/// XXX: Useless component used to differentiate Hp/MpMeters of a target or a caster
#[derive(Component)]
pub struct TargetMeter;

#[derive(Component)]
pub struct CombatStateDisplayer;

#[derive(Default, Component, Reflect, Deref, DerefMut)]
pub struct ActionDisplayer(pub usize);

/* -------------------------------------------------------------------------- */
/*                               Character Sheet                              */
/* -------------------------------------------------------------------------- */

#[derive(Component)]
pub struct CharacterSheet;

/// REFACTOR: Is the id still needed ? Associate entities at the Combat Initiation - no, we want to get from the CSh and from the fighter.
///
/// Still image showing a mini character sheet. (6 of allies and the pack of scrolls)
/// Contains its id
#[derive(Default, Component, Reflect, Deref, DerefMut)]
pub struct MiniCharacterSheet(pub usize);

#[derive(Default, Component, Reflect, Deref, DerefMut)]
pub struct SkillDisplayer(pub usize);

#[derive(Component)]
pub struct WeaponDisplayer;

/// REFACTOR: SkillBar Structure
#[derive(Component, Reflect, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum SkillBar {
    Base,
    Tier2,
    Tier1,
    Tier0,
    /// TODO: ShouldHave - Job's Skills
    Job,
    /// TODO: PostDemo - Unlock by the Job XpTree
    Extra,
}

#[derive(Component)]
pub struct Portrait;

#[derive(Component)]
pub struct FabienName;

#[derive(Component)]
pub struct Title;

/// # Note
///
/// I have to insert the resource
/// (init from_world is a bad idea: it's stupid to have a character sheet outside of the hud combat)
/// and to avoid abstract one level up with `CharacterSheetResource(Option<CharacterSheetElements>)`
/// (but in a case of a HashMap association to implement minis charsheets, it's mandatory to abstract)
/// I choose to put all field to `Option<Entity>`.
#[derive(Resource, Default, PartialEq, Eq, Hash, Clone, Debug, Reflect)]
pub struct CharacterSheetElements {
    pub character_sheet: Option<Entity>,

    pub portrait: Option<Entity>,
    pub name: Option<Entity>,
    pub title: Option<Entity>,
    pub job: Option<Entity>,
    pub weapon: Option<Entity>,
    pub health: Option<Entity>,
    pub mana: Option<Entity>,
    pub shield: Option<Entity>,
    pub initiative: Option<Entity>,
    pub attack: Option<Entity>,
    pub attack_spe: Option<Entity>,
    pub defense: Option<Entity>,
    pub defense_spe: Option<Entity>,
    pub base_skills: Option<Entity>,
    pub tier_2_skills: Option<Entity>,
    pub tier_1_skills: Option<Entity>,
    pub tier_0_skills: Option<Entity>,
}

/* -------------------------------------------------------------------------- */
/*                                 UI Creation                                */
/* -------------------------------------------------------------------------- */

/// Press `C` to open the combat panel or to close it if already open.
pub fn create_combat_panel_on_key_press(
    keyboard_input: Res<Input<KeyCode>>,
    mut ev_combat: EventWriter<CombatEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::C) {
        ev_combat.send(CombatEvent { attacker: None });
    }
}

/// Handle the CombatEvent to open (not close) the combat wall.
///
/// # Notes
///
/// REFACTOR: We might want to just switch `CombatWallStage` and create a system which change auto
/// the `HUDState`. At the moment, it's just here that we change `CombatWallStage`.
pub fn create_combat_panel_on_combat_event(
    mut commands: Commands,

    mut combat_events: EventReader<CombatEvent>,
    combat_wall_query: Query<(Entity, &Animator<Style>, &Style), With<CombatWall>>,
    mut initiative_bar_query: Query<&mut Visibility, With<InitiativeBar>>,

    allies_query: Query<Entity, Or<(With<Player>, With<Recruted>)>>,

    mut current_interlocutor: ResMut<CurrentInterlocutor>,

    mut combat_state: ResMut<CombatState>,
    mut next_game_state: ResMut<NextState<HUDState>>,
    current_combat_wall_state: Res<State<CombatWallStage>>,
    mut next_combat_wall_state: ResMut<NextState<CombatWallStage>>,
) {
    for CombatEvent { attacker } in combat_events.iter() {
        if let Ok((_entity, animator, _style)) = combat_wall_query.get_single() {
            // We can only close the `CombatWall` if we were in `CombatWallStage::Preparation`
            if attacker.is_none()
                && current_combat_wall_state.get() != &CombatWallStage::InCombat
                && animator.tweenable().progress() >= 1.
            {
                next_game_state.set(HUDState::Closed);
                next_combat_wall_state.set(CombatWallStage::Closed);
            }
            // FIXME: MustHave - We can avoid getting attack by entering the
            // `CombatWallStage::Preparation`
        } else {
            *combat_state = CombatState::SelectionCaster;
            current_interlocutor.interlocutor = None;
            next_game_state.set(HUDState::CombatWall);

            let mut init_bar_visibility = initiative_bar_query.single_mut();
            *init_bar_visibility = Visibility::Inherited;

            match attacker {
                None => next_combat_wall_state.set(CombatWallStage::Preparation),
                Some(enemy) => {
                    next_combat_wall_state.set(CombatWallStage::InCombat);
                    // TODO: MustHave - Spawn Enemy's company
                    commands.entity(*enemy).insert(InCombat(FIRST_ENEMY_ID));
                }
            }
            for (i, ally) in allies_query.iter().enumerate() {
                commands.entity(ally).insert(InCombat(FIRST_ALLY_ID + i));
            }
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                                 UI CleanUp                                 */
/* -------------------------------------------------------------------------- */

/// The Wall will despawn at the end of the animation in [[ui::mod]].
pub fn cleanup(
    mut commands: Commands,
    combat_wall_query: Query<(Entity, &Style), (With<CombatWall>, With<Animator<Style>>)>,
    mut initiative_bar_query: Query<&mut Visibility, With<InitiativeBar>>,

    in_combat_query: Query<Entity, With<InCombat>>,
    hud_state: Res<State<HUDState>>,
) {
    if hud_state.get() != &HUDState::LogCave {
        let mut init_bar_visibility = initiative_bar_query.single_mut();
        *init_bar_visibility = Visibility::Hidden;
        for fighter in &in_combat_query {
            commands.entity(fighter).remove::<InCombat>();
        }
    }

    // The current State where `cleanup()` is called is the upcoming transition state
    let end_position = if hud_state.get() == &HUDState::LogCave {
        UiRect {
            left: Val::Px(0.),
            top: Val::Px(HUD_PANEL_ANIMATION_OFFSET),
            right: Val::Px(0.),
            bottom: Val::Auto,
        }
    } else {
        UiRect {
            left: Val::Auto,
            top: Val::Px(0.),
            right: Val::Px(HUD_PANEL_ANIMATION_OFFSET),
            bottom: Val::Px(0.),
        }
    };

    if let Ok((entity, style)) = combat_wall_query.get_single() {
        let combat_panel_tween = Tween::new(
            EaseFunction::QuadraticIn,
            Duration::from_millis(HUD_PANEL_ANIMATION_TIME_MS),
            UiPositionLens {
                start: UiRect {
                    left: style.left,
                    right: style.right,
                    top: style.top,
                    bottom: style.bottom,
                },
                end: end_position,
            },
        )
        .with_completed_event(0);

        // Replace any animator with the new one created
        commands
            .entity(entity)
            .remove::<Animator<Style>>()
            .insert(Animator::new(combat_panel_tween));
    }
}

/* -------------------------------------------------------------------------- */
/*                                  UI Setup                                  */
/* -------------------------------------------------------------------------- */

pub fn combat_wall_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,

    combat_resources: Res<CombatResources>,
    hud_walls_section_query: Query<Entity, With<HUDWallsSection>>,

    character_sheet_resources: Res<CharacterSheetAssetsResources>,
    combat_wall_resources: Res<CombatWallResources>,
    combat_log_resources: Res<CombatLogResources>,

    mut character_sheet_elements: ResMut<CharacterSheetElements>,
) {
    /* -------------------------------------------------------------------------- */
    /*                                 UI Elements                                */
    /* -------------------------------------------------------------------------- */

    // TODO: Upgrade when Available - use Spritesheet
    // TODO: Apply a mask to conceal the legs of the portrait (or simply change asset but meh.)
    // Or put the portrait between the baseScroll and the base Header (but doesn't work)
    let portrait = commands
        .spawn((ImageBundle::default(), Name::new("Portrait"), Portrait))
        .id();

    let name = commands
        .spawn((
            TextBundle::from_section("Name", get_text_style(&asset_server, 40.))
                .with_style(TEXT_STYLE),
            Label,
            Name::new("Name"),
            FabienName,
        ))
        .id();

    let title = commands
        .spawn((
            TextBundle::from_section("Fabien", get_text_style(&asset_server, 20.))
                .with_style(TEXT_STYLE),
            Label,
            Name::new("Title"),
            Title,
        ))
        .id();

    let job = commands
        .spawn((
            TextBundle::from_section("Chill", get_text_style(&asset_server, 20.))
                .with_style(TEXT_STYLE),
            Label,
            Name::new("Job"),
            Job::default(),
        ))
        .id();

    let health = commands
        .spawn((
            TextBundle::from_section("Health: ???/???", get_text_style(&asset_server, 20.))
                .with_style(TEXT_STYLE),
            Label,
            Name::new("Health"),
            Hp::default(),
        ))
        .id();

    let mana = commands
        .spawn((
            TextBundle::from_section("Mana: ???/???", get_text_style(&asset_server, 20.))
                .with_style(TEXT_STYLE),
            Label,
            Name::new("Mana"),
            Mana::default(),
        ))
        .id();

    let shield = commands
        .spawn((
            TextBundle::from_section("Shield: ???", get_text_style(&asset_server, 20.))
                .with_style(TEXT_STYLE),
            Label,
            Name::new("Shield"),
            Shield::default(),
        ))
        .id();

    let initiative = commands
        .spawn((
            TextBundle::from_section("Initiative: ???", get_text_style(&asset_server, 20.))
                .with_style(TEXT_STYLE),
            Label,
            Name::new("Initiative"),
            Initiative::default(),
        ))
        .id();

    let attack = commands
        .spawn((
            TextBundle::from_section("Attack: ???", get_text_style(&asset_server, 20.))
                .with_style(TEXT_STYLE),
            Label,
            Name::new("Attack"),
            Attack::default(),
        ))
        .id();

    let attack_spe = commands
        .spawn((
            TextBundle::from_section("AttackSpe: ???", get_text_style(&asset_server, 20.))
                .with_style(TEXT_STYLE),
            Label,
            Name::new("AttackSpe"),
            AttackSpe::default(),
        ))
        .id();

    let defense = commands
        .spawn((
            TextBundle::from_section("Defense: ???", get_text_style(&asset_server, 20.))
                .with_style(TEXT_STYLE),
            Label,
            Name::new("Defense"),
            Defense::default(),
        ))
        .id();

    let defense_spe = commands
        .spawn((
            TextBundle::from_section("DefenseSpe: ???", get_text_style(&asset_server, 20.))
                .with_style(TEXT_STYLE),
            Label,
            Name::new("DefenseSpe"),
            DefenseSpe::default(),
        ))
        .id();

    let weapon = commands
        .spawn((
            ImageBundle {
                style: Style {
                    width: Val::Px(50.),
                    height: Val::Px(50.),
                    align_self: AlignSelf::Center,
                    ..default()
                },
                visibility: Visibility::Hidden,
                ..default()
            },
            Name::new("Weapon"),
            WeaponDisplayer,
        ))
        .id();

    let base_skills = commands
        .spawn((
            NodeBundle {
                style: Style {
                    // height: Val::Percent(42.),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            },
            Name::new("Base Skills"),
        ))
        .with_children(|parent| {
            // 6 Base skill max

            for skill_count in 0..6 {
                parent
                    .spawn((
                        ButtonBundle {
                            style: SKILL_BUTTON_STYLE,
                            background_color: NORMAL_BUTTON.into(),
                            visibility: Visibility::Hidden,
                            ..default()
                        },
                        Name::new(format!("Skill {}", skill_count)),
                        Skill::pass(),
                        // --- UI ---
                        SkillDisplayer(skill_count),
                        SkillBar::Base,
                        // Draggable,
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            format!("Skill {}", skill_count),
                            get_text_style(&asset_server, 20.),
                        ));
                    });
            }
        })
        .id();

    let tier_2_skills = commands
        .spawn((
            NodeBundle {
                style: Style {
                    // height: Val::Percent(42.),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            },
            Name::new("Tier2 Skills"),
        ))
        .with_children(|parent| {
            // 3 Tier2 skill max

            for skill_count in 0..3 {
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(150.0),
                                height: Val::Px(65.0),
                                // center button
                                margin: UiRect::all(Val::Auto),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: NORMAL_BUTTON.into(),
                            visibility: Visibility::Hidden,
                            ..default()
                        },
                        Name::new(format!("Skill {}", skill_count)),
                        Skill::pass(),
                        // --- UI ---
                        SkillDisplayer(skill_count),
                        SkillBar::Tier2,
                        // Draggable,
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            format!("Skill {}", skill_count),
                            get_text_style(&asset_server, 20.),
                        ));
                    });
            }
        })
        .id();

    let tier_1_skills = commands
        .spawn((
            NodeBundle {
                style: Style {
                    // height: Val::Percent(42.),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            },
            Name::new("Tier1 Skills"),
        ))
        .with_children(|parent| {
            // 3 Tier1 skill max

            for skill_count in 0..3 {
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(150.0),
                                height: Val::Px(65.0),
                                // center button
                                margin: UiRect::all(Val::Auto),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: NORMAL_BUTTON.into(),
                            visibility: Visibility::Hidden,
                            ..default()
                        },
                        Name::new(format!("Skill {}", skill_count)),
                        Skill::pass(),
                        // --- UI ---
                        SkillDisplayer(skill_count),
                        SkillBar::Tier1,
                        // Draggable,
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            format!("Skill {}", skill_count),
                            get_text_style(&asset_server, 20.),
                        ));
                    });
            }
        })
        .id();

    let tier_0_skills = commands
        .spawn((
            NodeBundle {
                style: Style {
                    // height: Val::Percent(42.),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            },
            Name::new("Tier0 Skills"),
        ))
        .with_children(|parent| {
            // 3 Tier0 skill max

            for skill_count in 0..3 {
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(150.0),
                                height: Val::Px(65.0),
                                // center button
                                margin: UiRect::all(Val::Auto),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: NORMAL_BUTTON.into(),
                            visibility: Visibility::Hidden,
                            ..default()
                        },
                        Name::new(format!("Skill {}", skill_count)),
                        Skill::pass(),
                        // --- UI ---
                        SkillDisplayer(skill_count),
                        SkillBar::Tier0,
                        // Draggable,
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            format!("Skill {}", skill_count),
                            get_text_style(&asset_server, 20.),
                        ));
                    });
            }
        })
        .id();

    /* -------------------------------------------------------------------------- */
    /*                              Character' Sheet                              */
    /* -------------------------------------------------------------------------- */

    let character_sheet = commands
        .spawn((
            NodeBundle {
                // image: character_sheet_resources.base_full_scroll.clone().into(),
                style: Style {
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    flex_shrink: 0.,
                    bottom: Val::Percent(100.),
                    ..default()
                },
                visibility: Visibility::Hidden,
                ..default()
            },
            Name::new("Character Sheet"),
            Interaction::default(),
            CharacterSheet,
        ))
        .with_children(|parent| {
            // Headers
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            height: Val::Percent(20.),
                            flex_direction: FlexDirection::Row,
                            ..default()
                        },
                        background_color: Color::DARK_GRAY.into(),
                        ..default()
                    },
                    Name::new("Headers"),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(30.),
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                background_color: Color::CRIMSON.into(),
                                ..default()
                            },
                            Name::new("Sprite Border"),
                        ))
                        .push_children(&[portrait]);

                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(70.),
                                    flex_direction: FlexDirection::Column,
                                    ..default()
                                },
                                background_color: Color::GRAY.into(),
                                ..default()
                            },
                            Name::new("Titles"),
                        ))
                        .with_children(|parent| {
                            // TODO: Update Titles
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            height: Val::Percent(50.),
                                            flex_direction: FlexDirection::Row,
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    Name::new("Full Name"),
                                ))
                                .push_children(&[name, title]);

                            parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            height: Val::Percent(50.),
                                            flex_direction: FlexDirection::Row,
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    Name::new("Job Section"),
                                ))
                                .push_children(&[job]);
                        });
                });

            // Stats + weapon
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            height: Val::Percent(40.),
                            flex_direction: FlexDirection::Row,
                            ..default()
                        },
                        background_color: Color::CRIMSON.into(),
                        ..default()
                    },
                    Name::new("Scanner"),
                ))
                .with_children(|parent| {
                    // TODO: Update Stats and Weapon equiped
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(60.),
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                background_color: Color::GRAY.into(),
                                ..default()
                            },
                            Name::new("Stats"),
                        ))
                        .push_children(&[
                            health,
                            mana,
                            shield,
                            initiative,
                            attack,
                            attack_spe,
                            defense,
                            defense_spe,
                        ]);

                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(40.),
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                // background_color: Color::BISQUE.into(),
                                ..default()
                            },
                            Name::new("Equipements Section"),
                        ))
                        .with_children(|parent| {
                            // TODO: add frame underneath
                            parent
                                .spawn((
                                    ImageBundle {
                                        image: character_sheet_resources
                                            .weapon_frame
                                            .clone()
                                            .into(),
                                        style: Style {
                                            width: Val::Px(100.),
                                            height: Val::Px(100.),
                                            align_self: AlignSelf::Center,
                                            justify_content: JustifyContent::Center,
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    Name::new("Frame"),
                                ))
                                .push_children(&[weapon]);
                        });
                });

            // Skill Menu
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            height: Val::Percent(40.),
                            flex_direction: FlexDirection::Column,
                            // align_content: AlignContent::SpaceAround,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        background_color: Color::AZURE.into(),
                        ..default()
                    },
                    Name::new("Skill Menu"),
                ))
                // A catalogue, one row for basic skill, a row for tier2 ,etc (simplify a lot skill_visibility)
                .push_children(&[base_skills, tier_2_skills, tier_1_skills, tier_0_skills]);

            // parent
            //     .spawn((
            //         NodeBundle {
            //             style: Style {
            //                 flex_direction: FlexDirection::Column,
            //                 // flex_wrap: FlexWrap::NoWrap,
            //                 height: Val::Percent(100.),
            //                 width: Val::Percent(100.),
            //                 ..default()
            //             },
            //             ..default()
            //         },
            //         Name::new("Content"),
            //     ))
            //     .with_children(|parent| {

            //     });

            // TODO: Render Top Decoration
            // parent
            //     .spawn((NodeBundle::default(), Name::new("Top Decoration")))
            //     .with_children(|parent| {
            //         // Top Decoration
            //         parent.spawn((
            //             ImageBundle {
            //                 image: character_sheet_resources
            //                     .top_left_corner
            //                     .clone()
            //                     .into(),
            //                 style: Style {
            //                     width: Val::Px(500.),
            //                     height: Val::Percent(100.),
            //                     ..default()
            //                 },
            //                 ..default()
            //             },
            //             Name::new("Decoration - Top Left Corner"),
            //         ));
            //     });
        })
        .id();

    *character_sheet_elements = CharacterSheetElements {
        character_sheet: Some(character_sheet),
        portrait: Some(portrait),
        name: Some(name),
        title: Some(title),
        job: Some(job),
        weapon: Some(weapon),
        health: Some(health),
        mana: Some(mana),
        shield: Some(shield),
        initiative: Some(initiative),
        attack: Some(attack),
        attack_spe: Some(attack_spe),
        defense: Some(defense),
        defense_spe: Some(defense_spe),
        base_skills: Some(base_skills),
        tier_2_skills: Some(tier_2_skills),
        tier_1_skills: Some(tier_1_skills),
        tier_0_skills: Some(tier_0_skills),
    };

    let combat_panel_tween = Tween::new(
        EaseFunction::QuadraticOut,
        Duration::from_millis(HUD_PANEL_ANIMATION_TIME_MS),
        UiPositionLens {
            start: UiRect {
                left: Val::Auto,
                top: Val::Px(0.),
                right: Val::Px(HUD_PANEL_ANIMATION_OFFSET),
                bottom: Val::Px(0.),
            },
            end: UiRect {
                left: Val::Auto,
                top: Val::Px(0.),
                right: Val::Px(0.),
                bottom: Val::Px(0.),
            },
        },
    );

    let hud_walls_section = hud_walls_section_query.single();
    commands.entity(hud_walls_section).with_children(|parent| {
        /* -------------------------------------------------------------------------- */
        /*                                  HUD Wall                                  */
        /* -------------------------------------------------------------------------- */
        parent
            .spawn((
                ImageBundle {
                    image: combat_wall_resources.base_combat_wall.clone().into(),
                    style: Style {
                        display: Display::Flex,
                        position_type: PositionType::Relative,
                        top: Val::Px(0.),
                        right: Val::Px(HUD_PANEL_ANIMATION_OFFSET),
                        bottom: Val::Px(0.),
                        margin: UiRect {
                            left: Val::Auto,
                            right: Val::Px(0.),
                            top: Val::Px(0.),
                            bottom: Val::Px(0.),
                        },
                        width: Val::Auto,
                        height: Val::Percent(100.),
                        aspect_ratio: Some(284. / 400.),
                        // -- Children --

                        // align_items: AlignItems::Center,
                        // justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                },
                Name::new("Combat Wall"),
                Animator::new(combat_panel_tween),
                CombatWall,
            ))
            .with_children(|parent| {
                /* First Side of the HUD Wall
                 * - Each Allied CharacterSheet (SubPanels) (Fixed Image)
                 *   - First Sub-Panel
                 *     - Headers: Sprite, Name, Title, Job
                 *     - Stats, Weapon Equiped
                 *     - Skill Menu²
                 * - TODO: "Bestiary" (Book of Enemy's characterSheet)
                 * - TODO: Logs
                 * - TODO: Team's Inventory
                 * - IDEA: If we block access to a certain number of members - Show empty sheets (with no text) to represent free space
                 */

                parent
                    .spawn((
                        NodeBundle {
                            // background_color: Color::DARK_GRAY.into(),
                            style: Style {
                                flex_shrink: 0.,
                                flex_direction: FlexDirection::Column,
                                height: Val::Percent(100.),

                                ..default()
                            },
                            ..default()
                        },
                        Name::new("Interactive items"),
                    ))
                    .with_children(|parent| {
                        // REFACTOR: put the custom size directly on the sprite (gap yes but no pos on the "root")
                        parent
                            .spawn((
                                NodeBundle {
                                    // background_color: Color::GRAY.into(),
                                    style: ALLIES_SHEET_STYLE,
                                    ..default()
                                },
                                Name::new("Allies' Scroll"),
                            ))
                            .with_children(|parent| {
                                parent
                                    .spawn((
                                        NodeBundle {
                                            style: ROW_SHEETS_STYLE,
                                            ..default()
                                        },
                                        Name::new("First Row of Scrolls"),
                                    ))
                                    .with_children(|parent| {
                                        for i in 0..3 {
                                            parent.spawn((
                                                ImageBundle {
                                                    image: combat_wall_resources.allies_scroll[i]
                                                        .clone()
                                                        .into(),
                                                    visibility: if i < combat_resources
                                                        .number_of_fighters
                                                        .ally
                                                        .total
                                                    {
                                                        Visibility::Inherited
                                                    } else {
                                                        Visibility::Hidden
                                                    },
                                                    style: MINI_CHARACTER_SHEET_STYLE,
                                                    ..default()
                                                },
                                                Name::new(format!("Ally's Scroll {}", i)),
                                                Interaction::default(),
                                                MiniCharacterSheet(i),
                                            ));
                                        }
                                    });

                                parent
                                    .spawn((
                                        NodeBundle {
                                            style: ROW_SHEETS_STYLE,
                                            ..default()
                                        },
                                        Name::new("Second Row of Scrolls"),
                                    ))
                                    .with_children(|parent| {
                                        for i in 3..6 {
                                            parent.spawn((
                                                ImageBundle {
                                                    image: combat_wall_resources.allies_scroll[i]
                                                        .clone()
                                                        .into(),
                                                    visibility: if i < combat_resources
                                                        .number_of_fighters
                                                        .ally
                                                        .total
                                                    {
                                                        Visibility::Inherited
                                                    } else {
                                                        Visibility::Hidden
                                                    },
                                                    style: MINI_CHARACTER_SHEET_STYLE,
                                                    ..default()
                                                },
                                                Name::new(format!("Ally's Scroll {}", i)),
                                                Interaction::default(),
                                                MiniCharacterSheet(i),
                                            ));
                                        }
                                    });
                            });

                        parent.spawn((
                            ImageBundle {
                                image: combat_wall_resources.pack_of_scroll.clone().into(),
                                style: Style {
                                    flex_shrink: 0.,
                                    width: Val::Percent(17.),
                                    left: Val::Percent(54.),
                                    top: Val::Percent(26.6),
                                    ..default()
                                },
                                ..default()
                            },
                            // DOC: Pack of scrolls = "Bestiary"
                            Name::new("Scrolls Pack"),
                            Interaction::default(),
                            // points to the first enemy
                            MiniCharacterSheet(FIRST_ENEMY_ID),
                        ));

                        // TODO: Hide it behind the altar
                        parent.spawn((
                            ImageBundle {
                                image: combat_log_resources.ladder.clone().into(),
                                style: Style {
                                    flex_shrink: 0.,
                                    width: Val::Percent(32.),
                                    left: Val::Percent(9.7),
                                    top: Val::Percent(20.5),
                                    ..default()
                                },
                                ..default()
                            },
                            Name::new("Downwards Ladder"),
                            Interaction::default(),
                            Ladder,
                        ));

                        // parent
                        //     .spawn((
                        //         NodeBundle {
                        //             background_color: Color::DARK_GRAY.into(),
                        //             style: Style {
                        //                 flex_shrink: 0.,
                        //                 flex_direction: FlexDirection::Row,
                        //                 //  height: Val::Percent(55.2),
                        //                 ..default()
                        //             },
                        //             ..default()
                        //         },
                        //         Name::new("Lower Screen"),
                        //     ))
                        //     .with_children(|parent| {

                        //         // parent
                        //         //     .spawn((
                        //         //         NodeBundle {
                        //         //             style: Style {
                        //         //                 width: Val::Percent(50.),
                        //         //                 ..default()
                        //         //             },
                        //         //             ..default()
                        //         //         },
                        //         //         Name::new("Lower Left Screen"),
                        //         //     ))
                        //         //     .with_children(|parent| {
                        //         //     });
                        //         // parent
                        //         //     .spawn((
                        //         //         NodeBundle {
                        //         //             style: Style {
                        //         //                  width: Val::Percent(50.),
                        //         //                 ..default()
                        //         //             },
                        //         //             ..default()
                        //         //         },
                        //         //         Name::new("Lower Right Screen"),
                        //         //     ))
                        //         //     .with_children(|parent| {
                        //         //     });
                        //     });
                    });

                // TODO: Spawn the ladder at the left box of the pack of scroll
            })
            .push_children(&[character_sheet]);
    });
}

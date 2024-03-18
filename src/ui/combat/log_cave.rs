//! Handle all spawn and component whihc are only present in the LogCave

use std::time::Duration;

use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    prelude::*,
};
use bevy_tweening::{lens::UiPositionLens, Animator, EaseFunction, Tween};

use crate::{
    constants::ui::{style::*, HUD_PANEL_ANIMATION_OFFSET, HUD_PANEL_ANIMATION_TIME_MS},
    ui::{
        combat::{combat_panel::Ladder, player_interaction::ScrollingList},
        HUDWallsSection,
    },
    HUDState,
};

/* -------------------------------------------------------------------------- */
/*                                UI Resources                                */
/* -------------------------------------------------------------------------- */

/// DOC : new name ? CombatLogAssetsResources
#[derive(Resource)]
pub struct CombatLogResources {
    pub base_log_cave: Handle<Image>,
    pub ladder: Handle<Image>,
}

impl FromWorld for CombatLogResources {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        CombatLogResources {
            base_log_cave: asset_server.load("textures/UI/HUD/combat/log/log_cave.png"),
            ladder: asset_server.load("textures/UI/HUD/combat/log/ladder.png"),
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                                UI Components                               */
/* -------------------------------------------------------------------------- */

/// DOC: rename to `LogCave`
#[derive(Component)]
pub struct HUDLog;

/// Points to the UI Text which display Current Action History
#[derive(Component)]
pub struct ActionHistoryDisplayer;

/// Points to the UI Text which display Last Turn Action History
#[derive(Component)]
pub struct LastActionHistoryDisplayer;

/// Points to the UI Text which display Last Turn Actions Precise Logs
#[derive(Component)]
pub struct ActionsLogsDisplayer;

/* -------------------------------------------------------------------------- */
/*                              Enter In the Log                              */
/* -------------------------------------------------------------------------- */

/// REFACTOR: Move to ui::player_interaction ?
pub fn cave_ladder(
    game_state: Res<State<HUDState>>,
    mut next_state: ResMut<NextState<HUDState>>,
    ladder_query: Query<&Interaction, (Changed<Interaction>, With<Ladder>)>,
) {
    if let Ok(Interaction::Pressed) = ladder_query.get_single() {
        match game_state.get() {
            HUDState::CombatWall => {
                next_state.set(HUDState::LogCave);
            }
            HUDState::LogCave => {
                next_state.set(HUDState::CombatWall);
            }
            _ => {}
        }
    }
    // TODO: Visual - Hover = outline (see README todos)
}

/* -------------------------------------------------------------------------- */
/*                                 UI CleanUp                                 */
/* -------------------------------------------------------------------------- */

/// The Fighting Scene and Initiative Bar are preserved
/// The current State where `cleanup()` is called is the upcoming transition state
/// Here It should always be `HUDState::CombatWall`.
pub fn cleanup(
    mut commands: Commands,
    log_cave_query: Query<(Entity, &Style), (With<HUDLog>, With<Animator<Style>>)>,
) {
    let end_position = UiRect {
        left: Val::Px(0.),
        right: Val::Px(0.),
        top: Val::Auto,
        bottom: Val::Px(HUD_PANEL_ANIMATION_OFFSET),
    };

    if let Ok((log_cave, style)) = log_cave_query.get_single() {
        let log_cave_tween = Tween::new(
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

        commands
            .entity(log_cave)
            .remove::<Animator<Style>>()
            .insert(Animator::new(log_cave_tween));
    }
    // commands.entity(log_cave).despawn_recursive();
}

/* -------------------------------------------------------------------------- */
/*                                  UI Setup                                  */
/* -------------------------------------------------------------------------- */

/// TODO: Must included FightingZone, InitiativeBar
/// IDEA: Spawn all FightingAre, InitiativeBar on startup, only despawn HUDWall or LogCave
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,

    combat_log_resources: Res<CombatLogResources>,
    hud_walls_section_query: Query<Entity, With<HUDWallsSection>>,
) {
    let log_cave_tween = Tween::new(
        EaseFunction::QuadraticOut,
        Duration::from_millis(HUD_PANEL_ANIMATION_TIME_MS),
        UiPositionLens {
            start: UiRect {
                left: Val::Px(0.),
                right: Val::Px(0.),
                top: Val::Px(HUD_PANEL_ANIMATION_OFFSET),
                bottom: Val::Auto,
            },
            end: UiRect {
                left: Val::Px(0.),
                right: Val::Px(0.),
                top: Val::Px(0.),
                bottom: Val::Auto,
            },
        },
    );

    let hud_walls_section = hud_walls_section_query.single();
    commands.entity(hud_walls_section).with_children(|parent| {
        /* -------------------------------------------------------------------------- */
        /*                                  LOG Cave                                  */
        /* -------------------------------------------------------------------------- */
        // TODO: Visual - Infinite scroll with the logCave sprite (like in CatDestroyer2000 cinematic)
        parent
            .spawn((
                ImageBundle {
                    image: combat_log_resources.base_log_cave.clone().into(),
                    style: Style {
                        display: Display::Flex,
                        position_type: PositionType::Relative,
                        top: Val::Px(HUD_PANEL_ANIMATION_OFFSET),
                        bottom: Val::Auto,
                        margin: UiRect {
                            left: Val::Auto,
                            right: Val::Px(0.),
                            top: Val::Px(0.),
                            bottom: Val::Px(0.),
                        },
                        width: Val::Auto,
                        height: Val::Percent(100.),
                        aspect_ratio: Some(284. / 400.),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                },
                Name::new("HUD Log"),
                HUDLog,
                Animator::new(log_cave_tween),
            ))
            .with_children(|parent| {
                // TODO: Scroll the logWall and ladder - (The ladder breaks the log scrolling)
                parent.spawn((
                    ImageBundle {
                        image: UiImage {
                            texture: combat_log_resources.ladder.clone(),
                            flip_y: true,
                            ..default()
                        },
                        style: Style {
                            // it could be this linethat break the scrolling
                            flex_shrink: 0.,
                            // NOT QUITE RIGHT
                            width: Val::Percent(27.5),
                            left: Val::Percent(7.3), // -0.5
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("Upwards Ladder"),
                    Interaction::default(),
                    // AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                    Ladder,
                ));

                parent
                    .spawn((
                        NodeBundle {
                            style: Style {
                                // flex_shrink: 0.,
                                width: Val::Percent(82.),
                                height: Val::Percent(100.),
                                flex_direction: FlexDirection::Column,
                                align_self: AlignSelf::Center,
                                overflow: Overflow::clip_y(),
                                ..default()
                            },
                            // background_color: Color::GRAY.into(),
                            ..default()
                        },
                        Name::new("Logs"),
                    ))
                    .with_children(|parent| {
                        // Moving panel
                        parent
                            .spawn((
                                NodeBundle {
                                    style: MOVING_PANEL_STYLE,
                                    ..default()
                                },
                                ScrollingList::default(),
                                AccessibilityNode(NodeBuilder::new(Role::List)),
                                Name::new("Moving Panel"),
                            ))
                            .with_children(|parent| {
                                // TODO: UI - Title that's stick to the top while scrolling their section
                                // FIXME: Line Width in Log needs to be dynamic
                                // List items

                                parent.spawn((
                                    TextBundle::from_section(
                                        "---------------\nCurrent Turn Actions:",
                                        get_text_style(&asset_server, 20.),
                                    )
                                    .with_style(Style {
                                        flex_wrap: FlexWrap::Wrap,
                                        width: Val::Auto,
                                        height: Val::Auto,
                                        // margin: UiRect {
                                        //     left: Val::Auto,
                                        //     right: Val::Auto,
                                        //     ..default()
                                        // },
                                        ..default()
                                    }),
                                    ActionHistoryDisplayer,
                                    Name::new("Actions History"),
                                    // -- UI --
                                    Label,
                                    AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                ));

                                parent.spawn((
                                    TextBundle::from_section(
                                        "---------------\nLast Actions:",
                                        get_text_style(&asset_server, 20.),
                                    )
                                    .with_style(Style {
                                        flex_wrap: FlexWrap::Wrap,
                                        width: Val::Auto,
                                        height: Val::Auto,
                                        // margin: UiRect {
                                        //     left: Val::Auto,
                                        //     right: Val::Auto,
                                        //     ..default()
                                        // },
                                        ..default()
                                    }),
                                    LastActionHistoryDisplayer,
                                    Name::new("Last Actions History"),
                                    // -- UI --
                                    Label,
                                    AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                ));

                                parent.spawn((
                                    TextBundle::from_section(
                                        "---------------\nActions Logs:",
                                        get_text_style(&asset_server, 20.),
                                    )
                                    .with_style(Style {
                                        flex_wrap: FlexWrap::Wrap,
                                        width: Val::Auto,
                                        height: Val::Auto,
                                        // margin: UiRect {
                                        //     left: Val::Auto,
                                        //     right: Val::Auto,
                                        //     ..default()
                                        // },
                                        ..default()
                                    }),
                                    ActionsLogsDisplayer,
                                    Name::new("Actions Logs"),
                                    // -- UI --
                                    Label,
                                    AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                ));

                                parent.spawn((
                                    TextBundle::from_section(
                                        "---------------",
                                        get_text_style(&asset_server, 20.),
                                    )
                                    .with_style(Style {
                                        flex_wrap: FlexWrap::Wrap,
                                        width: Val::Auto,
                                        height: Val::Auto,
                                        ..default()
                                    }),
                                    Name::new("----"),
                                    // -- UI --
                                    Label,
                                    AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                ));
                            });
                    });
            });
    });
}

use bevy::a11y::accesskit::{NodeBuilder, Role};
use bevy::a11y::AccessibilityNode;
use bevy::{prelude::*, winit::WinitSettings};
use bevy_tweening::TweenCompleted;

use crate::constants::ui::{
    style::{get_text_style, ACTION_BUTTON_STYLE, LIST_HIDDEN_OVERFLOW_STYLE, MOVING_PANEL_STYLE},
    FIGHTING_HALL_WIDTH, HUD_WALL_WIDTH, INITIATIVE_BAR_WIDTH, NORMAL_BUTTON,
};
use crate::GameState;

use self::combat::combat_panel::ActionDisplayer;
use self::combat::initiative_bar::InitiativeBar;
use self::combat::player_interaction::ScrollingList;
use self::combat::UiCombatPlugin;
use self::dialog::UiDialogPlugin;

pub mod combat;
pub mod dialog;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            // OPTIMIZE: Only run the app when there is user input. This will significantly reduce CPU/GPU use.
            .insert_resource(WinitSettings::game())
            .add_plugins((UiDialogPlugin, UiCombatPlugin))
            .add_systems(OnEnter(GameState::Playing), global_ui_setup)
            .add_systems(Update, despawn_hud_panel);
    }
}

pub fn despawn_hud_panel(mut commands: Commands, mut completed_event: EventReader<TweenCompleted>) {
    for TweenCompleted { entity, user_data } in completed_event.iter() {
        if *user_data == 0 {
            commands.entity(*entity).despawn_recursive();
        }
    }
}

/// Contains two parts. The rightmost is reserved for `HUDWall`s (CombatWall, LogCave, DialogWall).
/// And the rest of the UIScene can contain the `FightingScene` and the `InitiativeBar`.
#[derive(Component)]
pub struct UIScene;

/// The space not taken by the HUD
#[derive(Component)]
pub struct FreeScene;

/// Will contains the HUD walls
#[derive(Component)]
pub struct HUDWallsSection;

/// REFACTOR: Upgrade UiImage to spritesheet UI when [Available](https://github.com/bevyengine/bevy/pull/5070)
pub fn global_ui_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    /* -------------------------------------------------------------------------- */
    /*                                  UI Scene                                  */
    /* -------------------------------------------------------------------------- */
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    justify_content: JustifyContent::SpaceBetween,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            },
            Name::new("UI Scene"),
            UIScene,
        ))
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(FIGHTING_HALL_WIDTH),
                        ..default()
                    },
                    background_color: Color::rgba(0., 0., 0., 0.).into(),
                    ..default()
                },
                Name::new("Free Scene"),
                FreeScene,
            ));

            /* -------------------------------------------------------------------------- */
            /*                            Initiative Bar Order                            */
            /* -------------------------------------------------------------------------- */
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(INITIATIVE_BAR_WIDTH),
                            ..default()
                        },
                        background_color: Color::OLIVE.into(),
                        visibility: Visibility::Hidden,
                        ..default()
                    },
                    Name::new("Initiative Vertical Bar"),
                    InitiativeBar,
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            NodeBundle {
                                style: LIST_HIDDEN_OVERFLOW_STYLE,
                                ..default()
                            },
                            Name::new("List of Actions"),
                        ))
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: MOVING_PANEL_STYLE,
                                        ..default()
                                    },
                                    Name::new("Moving Panel"),
                                    // -- UI --
                                    ScrollingList::default(),
                                    AccessibilityNode(NodeBuilder::new(Role::List)),
                                ))
                                .with_children(|parent| {
                                    // 36 max actions (12entities playing thrice)

                                    for action_count in 0..36 {
                                        // each Button contains, as child, text and its sprite (caster's head)
                                        parent
                                            .spawn((
                                                ButtonBundle {
                                                    style: ACTION_BUTTON_STYLE,
                                                    background_color: NORMAL_BUTTON.into(),
                                                    visibility: Visibility::Hidden,
                                                    ..default()
                                                },
                                                Name::new(format!("Action {}", action_count)),
                                                // or put the action in it - space but better time comp
                                                ActionDisplayer(action_count),
                                                // -- UI --
                                                AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                            ))
                                            .with_children(|parent| {
                                                parent.spawn(TextBundle::from_section(
                                                    format!("Action {}", action_count),
                                                    get_text_style(&asset_server, 20.),
                                                ));

                                                parent.spawn((
                                                    ImageBundle {
                                                        image: UiImage {
                                                            flip_x: true,
                                                            ..default()
                                                        },
                                                        ..default()
                                                    },
                                                    Name::new(format!("Sprite {}", action_count)),
                                                ));
                                            });
                                    }
                                });
                        });
                });

            /* -------------------------------------------------------------------------- */
            /*                            HUD Walls will spawn                            */
            /*                        inEnter(HUDState::CombatWall)                       */
            /*                         inEnter(HUDState::LogCave)                         */
            /*                        inEnter(HUDState::DialogWall)                       */
            /* -------------------------------------------------------------------------- */

            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(HUD_WALL_WIDTH),
                        ..default()
                    },
                    // background_color: Color::OLIVE.into(),
                    ..default()
                },
                Name::new("HUD Walls section"),
                HUDWallsSection,
            ));
        });
}

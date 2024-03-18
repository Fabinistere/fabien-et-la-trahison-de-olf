use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{lens::UiPositionLens, Animator, EaseFunction, Tween};

use crate::{
    constants::ui::{
        style::{get_text_style, TEXT_STYLE},
        HUD_PANEL_ANIMATION_OFFSET, HUD_PANEL_ANIMATION_TIME_MS, NORMAL_BUTTON,
    },
    ui::FreeScene,
    HUDState,
};

use super::{
    combat_panel::{CombatStateDisplayer, TargetMeter},
    combat_system::{HpMeter, MpMeter},
    player_interaction::EndOfTurnButton,
};

#[derive(Component)]
pub struct FightingScene;

/* -------------------------------------------------------------------------- */
/*                                 UI CleanUp                                 */
/* -------------------------------------------------------------------------- */

/// The entity will despawn at the end of the animation in [[ui::mod]].
pub fn cleanup(
    mut commands: Commands,
    fighting_scene_query: Query<(Entity, &Style), (With<FightingScene>, With<Animator<Style>>)>,

    hud_state: Res<State<HUDState>>,
) {
    // The current State where `cleanup()` is called is the upcoming transition state
    if hud_state.get() == &HUDState::LogCave {
        return;
    }

    if let Ok((entity, style)) = fighting_scene_query.get_single() {
        let combat_panel_tween = Tween::new(
            EaseFunction::QuadraticIn,
            Duration::from_millis(HUD_PANEL_ANIMATION_TIME_MS),
            UiPositionLens {
                start: UiRect {
                    top: style.top,
                    bottom: style.bottom,
                    right: style.right,
                    left: style.left,
                },
                end: UiRect {
                    top: Val::Px(0.),
                    bottom: Val::Px(0.),
                    right: Val::Auto,
                    left: Val::Px(HUD_PANEL_ANIMATION_OFFSET),
                },
            },
        )
        .with_completed_event(0);

        commands
            .entity(entity)
            .remove::<Animator<Style>>()
            .insert(Animator::new(combat_panel_tween));
    }
}

/* -------------------------------------------------------------------------- */
/*                                  UI Setup                                  */
/* -------------------------------------------------------------------------- */

/// The `FightingScene` only spawn in `CombatWallStage::InCombat`
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    free_scene_query: Query<Entity, With<FreeScene>>,
    fighting_scene_query: Query<(Entity, &Style), (With<FightingScene>, With<Animator<Style>>)>,
) {
    if !fighting_scene_query.is_empty() {
        return;
    }

    let free_scene = free_scene_query.single();

    let scene_tween = Tween::new(
        EaseFunction::QuadraticOut,
        Duration::from_millis(HUD_PANEL_ANIMATION_TIME_MS),
        UiPositionLens {
            start: UiRect {
                top: Val::Px(0.),
                bottom: Val::Px(0.),
                right: Val::Auto,
                left: Val::Px(HUD_PANEL_ANIMATION_OFFSET),
            },
            end: UiRect {
                top: Val::Px(0.),
                bottom: Val::Px(0.),
                left: Val::Px(0.),
                right: Val::Auto,
            },
        },
    );

    commands.entity(free_scene).with_children(|parent| {
        /* -------------------------------------------------------------------------- */
        /*                               Fighting Scene                               */
        /*                             Where the npcs are                             */
        /* -------------------------------------------------------------------------- */

        parent
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    background_color: Color::DARK_GRAY.with_a(0.5).into(),
                    ..default()
                },
                Name::new("Fighting Scene"),
                FightingScene,
                Animator::new(scene_tween),
            ))
            .with_children(|parent| {
                // END OF YOUR TURN
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(200.0),
                                height: Val::Px(65.0),
                                margin: UiRect::all(Val::Auto),
                                top: Val::Percent(5.),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: NORMAL_BUTTON.into(),
                            ..default()
                        },
                        Name::new("EndTurn Button"),
                        EndOfTurnButton,
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "End of Turn",
                            get_text_style(&asset_server, 40.),
                        ));
                    });

                // Stats - Caster / Target
                parent
                    .spawn((
                        NodeBundle {
                            style: Style {
                                top: Val::Percent(5.),
                                flex_direction: FlexDirection::Column,
                                flex_grow: 1.0,
                                ..default()
                            },
                            ..default()
                        },
                        Name::new("Stats"),
                    ))
                    .with_children(|parent| {
                        // List items

                        // ----- DEBUG: Basic Stats -----
                        parent.spawn((
                            TextBundle::from_section(
                                "Target hp: ???",
                                get_text_style(&asset_server, 20.),
                            )
                            .with_style(TEXT_STYLE),
                            Label,
                            HpMeter,
                            TargetMeter,
                            Name::new("Target Hp"),
                        ));

                        parent.spawn((
                            TextBundle::from_section(
                                "Target mp: ???",
                                get_text_style(&asset_server, 20.),
                            )
                            .with_style(TEXT_STYLE),
                            Label,
                            MpMeter,
                            TargetMeter,
                            Name::new("Target Mp"),
                        ));

                        parent.spawn((
                            TextBundle::from_section(
                                "Combat Phase: ???",
                                get_text_style(&asset_server, 20.),
                            )
                            .with_style(Style {
                                flex_shrink: 0.,
                                width: Val::Px(0.),
                                height: Val::Px(20.),
                                margin: UiRect {
                                    left: Val::Auto,
                                    right: Val::Auto,
                                    ..default()
                                },
                                ..default()
                            }),
                            CombatStateDisplayer,
                            Name::new("Combat Phase"),
                            // -- UI --
                            // Because this is a distinct label widget and
                            // not button/list item text, this is necessary
                            // for accessibility to treat the text accordingly.
                            Label,
                        ));
                    });
            });
    });
}

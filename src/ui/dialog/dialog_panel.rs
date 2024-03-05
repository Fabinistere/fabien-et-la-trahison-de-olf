//! All base method involved in creating the UI ingame
//!
//! EventHandler:
//!
//! - Enter in Combat
//! - Exit in Combat
//! - Open HUD manually (pressing 'o')
//! - Scolls Gestion
//!   - Update Dialog Tree
//!   - Update each Scroll
//!   - Update Dialog Box / Text

use bevy::prelude::*;
use bevy_tweening::{lens::UiPositionLens, *};
use std::time::Duration;

use crate::{
    characters::player::Player,
    constants::ui::{dialogs::*, *},
    ui::dialog::dialog_scrolls::{
        ButtonChoice, MonologPanel, PlayerChoicePanel, Scroll, ScrollTimer,
    },
    HUDState,
};

use super::dialog_systems::CurrentInterlocutor;

#[derive(Resource)]
pub struct DialogPanelResources {
    text_font: Handle<Font>,
    appartements: Handle<Image>,
    stained_glass_panels: Handle<Image>,
    background: Handle<Image>,
    _stained_glass_closed: Handle<Image>,
    stained_glass_opened: Handle<Image>,
    _stained_glass_bars: Handle<Image>,
    chandelier: Handle<Image>,
    pub scroll_animation: Vec<Handle<Image>>,
}

#[derive(Component)]
pub struct DialogPanel;

pub fn load_textures(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // let scroll_texture = asset_server.load("textures/UI/HUD/dialog/scroll_animation.png");
    // let scroll_atlas = TextureAtlas::from_grid(scroll_texture, SCROLL_SIZE.into(), 1, 45);

    let mut scroll_animation_frames = vec![];
    for i in 0..SCROLL_ANIMATION_FRAMES_NUMBER {
        scroll_animation_frames.push(asset_server.load(&format!(
            "textures/UI/HUD/dialog/scroll_animation/frame_{}.png",
            i
        )));
    }

    commands.insert_resource(DialogPanelResources {
        text_font: asset_server.load("fonts/dpcomic.ttf"),
        appartements: asset_server.load("textures/UI/HUD/dialog/papier_paint.png"),
        background: asset_server.load("textures/UI/HUD/dialog/dialog_background.png"),
        scroll_animation: scroll_animation_frames,
        chandelier: asset_server.load("textures/UI/HUD/dialog/chandelier.png"),
        _stained_glass_closed: asset_server.load("textures/UI/HUD/dialog/stained_glass_closed.png"),
        stained_glass_opened: asset_server.load("textures/UI/HUD/dialog/stained_glass_opened.png"),
        _stained_glass_bars: asset_server.load("textures/UI/HUD/dialog/stained_glass_bars.png"),
        stained_glass_panels: asset_server.load("textures/UI/HUD/dialog/stained_glass_panels.png"),
    });
}

/// # Note
///
/// TODO: feature - exit the personal thought or any tab when being touch by aggro
///
/// FIXME: inactive - PB Spamming the ui key 'o'; ?throws an error
pub fn create_dialog_panel_on_key_press(
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<(Entity, &Animator<Style>, &Style), With<DialogPanel>>,

    mut current_interlocutor: ResMut<CurrentInterlocutor>,
    player_query: Query<Entity, With<Player>>,

    mut next_game_state: ResMut<NextState<HUDState>>,
) {
    if keyboard_input.just_pressed(KeyCode::O) {
        if let Ok((_entity, animator, _style)) = query.get_single() {
            if animator.tweenable().progress() >= 1. {
                next_game_state.set(HUDState::Closed);
            }
        } else {
            let player = player_query.single();

            current_interlocutor.interlocutor = Some(player);
            next_game_state.set(HUDState::DialogWall);
        }
    }
}

/// The Panel will despawn at the end of the animation
pub fn close_dialog_panel(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Animator<Style>, &Style), With<DialogPanel>>,
) {
    // info!("close dialog event");
    if let Ok((entity, mut _animator, style)) = query.get_single_mut() {
        let dialog_panel_tween = Tween::new(
            EaseFunction::QuadraticIn,
            Duration::from_millis(HUD_PANEL_ANIMATION_TIME_MS),
            UiPositionLens {
                start: UiRect {
                    left: style.left,
                    right: style.right,
                    top: style.top,
                    bottom: style.bottom,
                },
                end: UiRect {
                    left: Val::Auto,
                    top: Val::Px(0.),
                    right: Val::Px(HUD_PANEL_ANIMATION_OFFSET),
                    bottom: Val::Px(0.),
                },
            },
        )
        .with_completed_event(0);

        // Replace any animator with the new one created
        commands
            .entity(entity)
            .remove::<Animator<Style>>()
            .insert(Animator::new(dialog_panel_tween));
    }
}

pub fn create_dialog_panel(
    mut commands: Commands,
    mut _meshes: ResMut<Assets<Mesh>>,
    _texture_atlases: Res<Assets<TextureAtlas>>,
    dialog_panel_resources: Res<DialogPanelResources>,
    asset_server: Res<AssetServer>,
) {
    // info!("open dialog event");

    let dialog_panel_tween = Tween::new(
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

    let panels_tween = Tween::new(
        EaseMethod::Linear,
        Duration::from_millis(1000),
        UiPositionLens {
            start: UiRect {
                top: Val::Px(0.),
                ..UiRect::default()
            },
            end: UiRect {
                top: Val::Px(-194.),
                ..UiRect::default()
            },
        },
    );

    commands
        .spawn((
            // We spawn the paper wall background.
            // To hide the windows' panels when reaching
            // the top of the window.
            // Because the main Wall Background is above these panels.
            ImageBundle {
                image: dialog_panel_resources.appartements.clone().into(),
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
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
                    ..Style::default()
                },
                ..ImageBundle::default()
            },
            Name::new("Dialog Wall"),
            Animator::new(dialog_panel_tween),
            DialogPanel,
        ))
        .with_children(|parent| {
            let child_sprite_style = Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..Style::default()
            };

            // panels under the wall to prevent them from sticking out of the window after being lifted.
            parent.spawn((
                ImageBundle {
                    image: dialog_panel_resources.stained_glass_panels.clone().into(),
                    style: child_sprite_style.clone(),
                    ..ImageBundle::default()
                },
                Animator::new(panels_tween),
                Name::new("Stained Glass Panel"),
            ));

            parent.spawn((
                ImageBundle {
                    image: dialog_panel_resources.background.clone().into(),
                    style: child_sprite_style.clone(),
                    ..ImageBundle::default()
                },
                Name::new("Wall Background"),
            ));

            parent.spawn((
                ImageBundle {
                    image: dialog_panel_resources.stained_glass_opened.clone().into(),
                    style: child_sprite_style.clone(),
                    ..ImageBundle::default()
                },
                Name::new("Stained Glass Static"),
            ));

            parent.spawn((
                ImageBundle {
                    image: dialog_panel_resources.chandelier.clone().into(),
                    style: child_sprite_style.clone(),
                    ..ImageBundle::default()
                },
                Name::new("Light"),
            ));

            /* -------------------------------------------------------------------------- */
            /*                                Upper Scroll                                */
            /* -------------------------------------------------------------------------- */

            parent
                .spawn((
                    ImageBundle {
                        // REFACTOR: Replace by a spritesheet
                        image: dialog_panel_resources.scroll_animation[0].clone().into(),
                        style: Style {
                            position_type: PositionType::Absolute,
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::FlexStart,
                            justify_content: JustifyContent::FlexEnd,
                            ..Style::default()
                        },
                        ..ImageBundle::default()
                    },
                    Scroll {
                        current_frame: 0,
                        reverse: false,
                    },
                    MonologPanel,
                    ScrollTimer(Timer::from_seconds(
                        SCROLL_ANIMATION_DELTA_S,
                        TimerMode::Once,
                    )),
                    Name::new("Upper Scroll"),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "",
                            TextStyle {
                                font: dialog_panel_resources.text_font.clone(),
                                font_size: 30.,
                                color: Color::BLACK,
                            },
                        )
                        .with_alignment(TextAlignment::Left),
                        style: Style {
                            flex_wrap: FlexWrap::Wrap,
                            top: Val::Px(565.),
                            margin: UiRect {
                                left: Val::Percent(24.),
                                ..UiRect::default()
                            },
                            // Percent ?
                            width: Val::Px(300.),
                            height: Val::Percent(100.),
                            ..Style::default()
                        },
                        ..TextBundle::default()
                    });
                })
                // .insert(DialogBox::new(dialog[0].clone(), DIALOG_BOX_UPDATE_DELTA_S))
                ;

            // parent
            //     .spawn(ImageBundle {
            //         image: texture_atlases
            //             .get(dialog_panel_resources.scroll_animation.clone())
            //             .unwrap()
            //             .texture
            //             .clone_weak()
            //             .into(),
            //         style: child_sprite_style.clone(),
            //         ..ImageBundle::default()
            //     });

            /* -------------------------------------------------------------------------- */
            /*                                Player Scroll                               */
            /* -------------------------------------------------------------------------- */

            let player_scroll_img =
                asset_server.load("textures/UI/HUD/dialog/HUD_1px_parchemin_MC_ouvert.png");

            parent
                .spawn((
                    ImageBundle {
                        image: player_scroll_img.clone().into(),
                        style: Style {
                            // REFACTOR: Player Choice Panel's Style
                            position_type: PositionType::Absolute,
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::FlexStart,
                            justify_content: JustifyContent::FlexEnd,
                            ..default()
                        },
                        ..default()
                    },
                    Scroll {
                        current_frame: 0,
                        reverse: false,
                    },
                    PlayerChoicePanel,
                    ScrollTimer(Timer::from_seconds(
                        SCROLL_ANIMATION_DELTA_S,
                        TimerMode::Once,
                    )),
                    Name::new("Player Scroll"),
                ))
                .with_children(|parent| {
                    // TODO: feature - 3 ButtonChoice is enough, to have much reuse theses three in another page

                    for i in 0..3 {
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        // TODO: custom size ? (text dependent)
                                        width: Val::Px(300.),
                                        height: Val::Px(30.),
                                        margin: UiRect::all(Val::Auto),
                                        top: Val::Px(
                                            FIRST_BUTTON_TOP_VAL - BUTTON_SPACING * i as f32,
                                        ),
                                        left: Val::Px(BUTTON_LEFT_VAL),
                                        ..default()
                                    },
                                    background_color: TRANSPARENT_BUTTON.into(),
                                    visibility: Visibility::Hidden,
                                    ..default()
                                },
                                Name::new(format!("Choice n°{i}")),
                                ButtonChoice::new(i),
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle {
                                    text: Text::from_section(
                                        "",
                                        TextStyle {
                                            font: dialog_panel_resources.text_font.clone(),
                                            // TODO: Find the correct value for the choice font size
                                            font_size: 25.,
                                            color: Color::BLACK,
                                        },
                                    )
                                    .with_alignment(TextAlignment::Left),
                                    style: Style {
                                        flex_wrap: FlexWrap::Wrap,
                                        max_width: Val::Px(300.),
                                        max_height: Val::Percent(100.),
                                        ..default()
                                    },
                                    ..default()
                                });
                            });
                    }
                });
        });
}

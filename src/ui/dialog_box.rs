use crate::constants::ui::dialogs::*;

use bevy::prelude::*;
// render::RenderWorld,
// sprite::{MaterialMesh2dBundle, Mesh2dHandle},
// ui::{ExtractedUiNode, ExtractedUiNodes},
use bevy_tweening::{lens::UiPositionLens, *};
use std::time::Duration;

#[derive(Component)]
pub struct DialogPanel;

#[derive(Debug, Component)]
pub struct DialogBox {
    text: String,
    progress: usize,
    finished: bool,
    update_timer: Timer,
}

impl DialogBox {
    pub fn new(text: String, update_time: f32) -> Self {
        DialogBox {
            text,
            update_timer: Timer::from_seconds(update_time, true),
            finished: false,
            progress: 0,
        }
    }
}

#[derive(Component)]
pub struct DialogBoxText;
#[derive(Component)]
pub struct Scroll {
    current_frame: usize,
    reverse: bool,
}
#[derive(Component, Deref, DerefMut)]
pub struct ScrollTimer(Timer);

pub struct CreateDialogBoxEvent {
    dialog: String,
}
pub struct CloseDialogBoxEvent;

pub struct DialogBoxResources {
    text_font: Handle<Font>,
    background: Handle<Image>,
    chandelier: Handle<Image>,
    stained_glass_closed: Handle<Image>,
    stained_glass_opened: Handle<Image>,
    stained_glass_bars: Handle<Image>,
    stained_glass_panels: Handle<Image>,
    scroll_animation: Vec<Handle<Image>>,
}

pub fn load_textures(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // let scroll_texture = asset_server.load("textures/hud/scroll_animation.png");
    // let scroll_atlas = TextureAtlas::from_grid(scroll_texture, SCROLL_SIZE.into(), 1, 45);

    let mut scroll_animation_frames = vec![];
    for i in 0..SCROLL_ANIMATION_FRAMES_NUMBER {
        scroll_animation_frames
            .push(asset_server.load(&format!("textures/hud/scroll_animation/frame_{}.png", i)));
    }

    commands.insert_resource(DialogBoxResources {
        text_font: asset_server.load("fonts/dpcomic.ttf"),
        background: asset_server.load("textures/hud/dialog_background.png"),
        scroll_animation: scroll_animation_frames,
        chandelier: asset_server.load("textures/hud/chandelier.png"),
        stained_glass_closed: asset_server.load("textures/hud/stained_glass_closed.png"),
        stained_glass_opened: asset_server.load("textures/hud/stained_glass_opened.png"),
        stained_glass_bars: asset_server.load("textures/hud/stained_glass_bars.png"),
        stained_glass_panels: asset_server.load("textures/hud/stained_glass_panels.png"),
    });
}

pub fn create_dialog_box_on_key_press(
    mut create_dialog_box_event: EventWriter<CreateDialogBoxEvent>,
    mut close_dialog_box_event: EventWriter<CloseDialogBoxEvent>,
    query: Query<(Entity, &Animator<Style>, &Style), With<DialogPanel>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::O) {
        if let Ok((_entity, animator, _style)) = query.get_single() {
            if animator.tweenable().unwrap().progress() >= 1.0 {
                close_dialog_box_event.send(CloseDialogBoxEvent);
            }
        } else {
            info!("here second");
            create_dialog_box_event.send(CreateDialogBoxEvent {
                dialog: "Bonjour Florian. Comment vas-tu ? J'ai faim.".to_owned(),
            });
        }
    }
}

pub fn close_dialog_box(
    mut commands: Commands,
    mut close_dialog_box_events: EventReader<CloseDialogBoxEvent>,
    mut query: Query<(Entity, &mut Animator<Style>, &Style), With<DialogPanel>>,
) {
    for CloseDialogBoxEvent in close_dialog_box_events.iter() {
        info!("close dialog event");
        if let Ok((entity, mut _animator, style)) = query.get_single_mut() {
            let dialog_box_tween = Tween::new(
                EaseFunction::QuadraticIn,
                TweeningType::Once,
                Duration::from_millis(DIALOG_BOX_ANIMATION_TIME_MS),
                UiPositionLens {
                    start: style.position,
                    end: UiRect {
                        left: Val::Auto,
                        top: Val::Px(0.0),
                        right: Val::Px(DIALOG_BOX_ANIMATION_OFFSET),
                        bottom: Val::Px(0.0),
                    },
                },
            )
            .with_completed_event(0);

            commands
                .entity(entity)
                .remove::<Animator<Style>>()
                .insert(Animator::new(dialog_box_tween));
        }
    }
}

pub fn despawn_dialog_box(
    mut commands: Commands,
    mut completed_event: EventReader<TweenCompleted>,
) {
    for TweenCompleted { entity, user_data } in completed_event.iter() {
        if *user_data == 0 {
            commands.entity(*entity).despawn_recursive();
        }
    }
}

pub fn create_dialog_box(
    mut create_dialog_box_events: EventReader<CreateDialogBoxEvent>,
    mut commands: Commands,
    mut _meshes: ResMut<Assets<Mesh>>,
    _texture_atlases: Res<Assets<TextureAtlas>>,
    dialog_box_resources: Res<DialogBoxResources>,
) {
    for CreateDialogBoxEvent { dialog } in create_dialog_box_events.iter() {
        info!("open dialog event");
        let dialog_box_tween = Tween::new(
            EaseFunction::QuadraticOut,
            TweeningType::Once,
            Duration::from_millis(DIALOG_BOX_ANIMATION_TIME_MS),
            UiPositionLens {
                start: UiRect {
                    left: Val::Auto,
                    top: Val::Px(0.0),
                    right: Val::Px(DIALOG_BOX_ANIMATION_OFFSET),
                    bottom: Val::Px(0.0),
                },
                end: UiRect {
                    left: Val::Auto,
                    top: Val::Px(0.0),
                    right: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                },
            },
        );

        let panels_tween = Tween::new(
            EaseMethod::Linear,
            TweeningType::Once,
            Duration::from_millis(1000),
            UiPositionLens {
                start: UiRect {
                    top: Val::Px(0.0),
                    ..UiRect::default()
                },
                end: UiRect {
                    top: Val::Px(-160.0),
                    ..UiRect::default()
                },
            },
        );

        commands
            .spawn_bundle(ImageBundle {
                image: dialog_box_resources.background.clone().into(),
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Relative,
                    position: UiRect {
                        top: Val::Px(0.0),
                        left: Val::Auto,
                        right: Val::Px(DIALOG_BOX_ANIMATION_OFFSET),
                        bottom: Val::Px(0.0),
                    },
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Px(0.0),
                        top: Val::Px(0.0),
                        bottom: Val::Px(0.0),
                    },
                    size: Size::new(Val::Auto, Val::Percent(100.0)),
                    aspect_ratio: Some(284.0 / 400.0),
                    ..Style::default()
                },
                ..ImageBundle::default()
            })
            .with_children(|parent| {
                let child_sprite_style = Style {
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    ..Style::default()
                };

                parent.spawn_bundle(ImageBundle {
                    image: dialog_box_resources.chandelier.clone().into(),
                    style: child_sprite_style.clone(),
                    ..ImageBundle::default()
                });

                parent
                    .spawn_bundle(ImageBundle {
                        image: dialog_box_resources.stained_glass_panels.clone().into(),
                        style: child_sprite_style.clone(),
                        ..ImageBundle::default()
                    })
                    .insert(Animator::new(panels_tween));

                parent.spawn_bundle(ImageBundle {
                    image: dialog_box_resources.stained_glass_opened.clone().into(),
                    style: child_sprite_style.clone(),
                    ..ImageBundle::default()
                });

                parent
                    .spawn_bundle(ImageBundle {
                        image: dialog_box_resources.scroll_animation[0].clone().into(),
                        style: Style {
                            position_type: PositionType::Absolute,
                            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::FlexStart,
                            justify_content: JustifyContent::FlexEnd,
                            ..Style::default()
                        },
                        ..ImageBundle::default()
                    })
                    .insert(Scroll {
                        current_frame: 0,
                        reverse: false,
                    })
                    .insert(ScrollTimer(Timer::from_seconds(
                        SCROLL_ANIMATION_DELTA_S,
                        false,
                    )))
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            text: Text::from_section(
                                "",
                                TextStyle {
                                    font: dialog_box_resources.text_font.clone(),
                                    font_size: 30.0,
                                    color: Color::BLACK,
                                },
                            )
                            .with_alignment(TextAlignment {
                                vertical: VerticalAlign::Top,
                                horizontal: HorizontalAlign::Left,
                            }),
                            style: Style {
                                flex_wrap: FlexWrap::Wrap,
                                margin: UiRect {
                                    top: Val::Percent(74.0),
                                    left: Val::Percent(24.0),
                                    ..UiRect::default()
                                },
                                max_size: Size::new(Val::Px(450.0), Val::Percent(100.0)),
                                ..Style::default()
                            },
                            ..TextBundle::default()
                        });
                    })
                    .insert(DialogBox::new(dialog.clone(), DIALOG_BOX_UPDATE_DELTA_S));

                // parent.spawn_bundle(ImageBundle {
                //     image: texture_atlases
                //         .get(dialog_box_resources.scroll_animation.clone())
                //         .unwrap()
                //         .texture
                //         .clone_weak()
                //         .into(),
                //     style: child_sprite_style.clone(),
                //     ..ImageBundle::default()
                // });
            })
            .insert(DialogPanel)
            .insert(Animator::new(dialog_box_tween));
    }
}

pub fn update_dialog_box(
    time: Res<Time>,
    mut dialog_box_query: Query<(&mut DialogBox, &Children)>,
    mut text_query: Query<&mut Text>,
) {
    for (mut dialog_box, children) in dialog_box_query.iter_mut() {
        dialog_box.update_timer.tick(time.delta());

        if dialog_box.update_timer.finished() && !dialog_box.finished {
            let mut text = text_query.get_mut(children[0]).unwrap();
            let next_letter = dialog_box.text.chars().nth(dialog_box.progress).unwrap();
            text.sections[0].value.push(next_letter);

            dialog_box.progress += 1;
            if dialog_box.progress >= dialog_box.text.len() {
                dialog_box.finished = true;
            }
        }
    }
}

pub fn animate_scroll(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    dialog_box_resources: Res<DialogBoxResources>,
    mut commands: Commands,
    mut scroll_query: Query<(&mut UiImage, &mut Scroll, &mut ScrollTimer, Entity)>,
) {
    for (mut image, mut scroll, mut timer, entity) in scroll_query.iter_mut() {
        timer.tick(time.delta());

        if timer.finished() {
            if scroll.reverse {
                scroll.current_frame -= 1;

                if scroll.current_frame == 0 {
                    commands.entity(entity).remove::<ScrollTimer>();
                }
            } else {
                scroll.current_frame += 1;

                if scroll.current_frame >= SCROLL_ANIMATION_FRAMES_NUMBER - 1 {
                    commands.entity(entity).remove::<ScrollTimer>();
                }
            }

            image.0 = dialog_box_resources.scroll_animation[scroll.current_frame].clone();
        }
    }
}

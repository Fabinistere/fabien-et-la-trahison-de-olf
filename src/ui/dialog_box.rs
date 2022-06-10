use crate::constants::ui::dialogs::DIALOG_BOX_UPDATE_DELTA;
use bevy::prelude::*;
use bevy_tweening::{lens::UiPositionLens, *};
use std::time::Duration;

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

pub struct DialogBoxResources {
    text_font: Handle<Font>,
    background: Handle<Image>,
}

pub fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    let background = asset_server.load("textures/hud/dialog_background.png");
    let text_font = asset_server.load("fonts/dpcomic.ttf");

    commands.insert_resource(DialogBoxResources {
        background,
        text_font,
    });
}

pub fn create_dialog_box_on_key_press(
    mut commands: Commands,
    dialog_box_resources: Res<DialogBoxResources>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(Entity, &mut Animator<Style>), With<DialogBox>>,
) {
    if keyboard_input.just_pressed(KeyCode::O) {
        if let Ok((entity, mut animator)) = query.get_single_mut() {
            animator.rewind();
        } else {
            create_dialog_box(
                commands,
                dialog_box_resources,
                "Bonjour Florian\nComment vas-tu ?\nJ'ai faim.".to_string(),
            );
        }
    }
}

pub fn create_dialog_box(
    mut commands: Commands,
    dialog_box_resources: Res<DialogBoxResources>,
    dialog: String,
) {
    let start_right_offset = -1000.0;

    let tween = Tween::new(
        EaseFunction::QuadraticOut,
        TweeningType::Once,
        Duration::from_millis(500),
        UiPositionLens {
            start: Rect {
                left: Val::Auto,
                top: Val::Px(0.0),
                right: Val::Px(-1000.0),
                bottom: Val::Px(0.0),
            },
            end: Rect {
                left: Val::Auto,
                top: Val::Px(0.0),
                right: Val::Px(0.0),
                bottom: Val::Px(0.0),
            },
        },
    );

    commands
        .spawn_bundle(ImageBundle {
            image: dialog_box_resources.background.clone().into(),
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Relative,
                position: Rect {
                    top: Val::Px(0.0),
                    left: Val::Auto,
                    right: Val::Px(start_right_offset),
                    bottom: Val::Px(0.0),
                },
                margin: Rect {
                    left: Val::Auto,
                    right: Val::Px(0.0),
                    top: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                },
                size: Size::new(Val::Auto, Val::Percent(100.0)),
                aspect_ratio: Some(284.0 / 400.0),
                overflow: Overflow::Hidden,
                ..Style::default()
            },
            // transform: Transform::from_translation(Vec3::new(100.0, 100.0, 0.0)),
            ..ImageBundle::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "",
                    TextStyle {
                        font: dialog_box_resources.text_font.clone(),
                        font_size: 50.0,
                        color: Color::BLACK,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                style: Style {
                    position_type: PositionType::Relative,
                    // margin: Rect::all(Val::Auto),
                    // size: Size::new(Val::Auto, Val::Percent(100.0)),
                    // aspect_ratio: Some(284.0 / 400.0),
                    // position: Rect::all(Val::Percent(100.0)),
                    ..Style::default()
                },
                // transform: Transform::from_translation(Vec3::new(-100.0, -100.0, 5.0)),
                ..TextBundle::default()
            });
        })
        .insert(DialogBox::new(dialog, DIALOG_BOX_UPDATE_DELTA))
        .insert(Animator::new(tween));
    // .insert(UiSlide::new(
    //     Duration::from_millis(500),
    //     UiSlideType::ToLeft,
    //     -start_right_offset,
    //     start_right_offset,
    //     ease_out_sine,
    // ));
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

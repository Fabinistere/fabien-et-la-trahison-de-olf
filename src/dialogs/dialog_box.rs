use crate::constants::ui::dialogs::DIALOG_BOX_UPDATE_DELTA;
use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct DialogBox {
    text: String,
    progress: usize,
    finished: bool,
    update_timer: Timer,
}

#[derive(Component)]
pub struct DialogBoxText;
#[derive(Component)]
pub struct UiCamera;

pub fn create_dialog_box_on_key_press(
    commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::O) {
        create_dialog_box(
            commands,
            asset_server,
            "Bonjour Florian\nComment vas-tu ?\nJ'ai faim.".to_string(),
        );
    }
}

pub fn destroy_dialog_box(
    mut commands: Commands,
    mut query: ParamSet<(
        Query<Entity, With<DialogBox>>,
        Query<Entity, With<UiCamera>>,
    )>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::E) {
        for entity in query.p0().iter() {
            commands.entity(entity).despawn_recursive();
        }

        for entity in query.p1().iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn create_dialog_box(mut commands: Commands, asset_server: Res<AssetServer>, dialog: String) {
    commands
        .spawn_bundle(ImageBundle {
            image: asset_server.load("textures/dialog_box.png").into(),
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(10.0),
                    left: Val::Px(10.0),
                    right: Val::Px(10.0),
                    bottom: Val::Auto,
                },
                size: Size::new(Val::Auto, Val::Px(400.0)),
                ..Style::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)),
            ..ImageBundle::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "",
                    TextStyle {
                        font: asset_server.load("fonts/dpcomic.ttf"),
                        font_size: 50.0,
                        color: Color::BLACK,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
                ..TextBundle::default()
            });
        })
        .insert(DialogBox {
            text: dialog,
            progress: 0,
            finished: false,
            update_timer: Timer::from_seconds(DIALOG_BOX_UPDATE_DELTA, true),
        });

    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(UiCamera);
}

pub fn update_dialog_box(
    time: Res<Time>,
    mut dialog_box_query: Query<(&mut DialogBox, &Children)>,
    mut text_query: Query<&mut Text>,
) {
    if let Ok((mut dialog_box, children)) = dialog_box_query.get_single_mut() {
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

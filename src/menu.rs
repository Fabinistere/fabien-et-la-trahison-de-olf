use crate::{DialogId, Dialogs, GameState, Language};
use bevy::{input::keyboard::KeyboardInput, prelude::*};
use strum::IntoEnumIterator;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LanguageChangedEvent>()
            .init_resource::<LanguagesButtonColors>()
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup_menu))
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(destroy_menu))
            .add_system(language_button_interactions)
            .add_system(game_start)
            .add_system(language_changed);
    }
}

#[derive(Component)]
struct Menu;
#[derive(Component)]
struct LanguageChangedEvent;
#[derive(Component)]
struct Selected(bool);

struct LanguagesButtonColors {
    normal: Color,
    hovered: Color,
    selected: Color,
    hovered_selected: Color,
}

impl Default for LanguagesButtonColors {
    fn default() -> Self {
        LanguagesButtonColors {
            normal: Color::rgb(0.9, 0.9, 0.9),
            hovered: Color::rgb(0.8, 0.8, 0.8),
            selected: Color::rgb(1.0, 0.9, 0.0),
            hovered_selected: Color::rgb(0.9, 0.8, 0.0),
        }
    }
}

fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    dialogs: Res<Dialogs>,
    languages_button_colors: Res<LanguagesButtonColors>,
    language: Res<Language>,
) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(Menu);

    let font = asset_server.load("fonts/dpcomic.ttf");
    let background_image = asset_server.load("textures/Monkey_ULTIME.png");

    let background_image = ImageBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect::all(Val::Px(0.0)),
            aspect_ratio: Some(10.0 / 9.0),
            ..Style::default()
        },
        image: background_image.into(),
        ..ImageBundle::default()
    };

    let title = TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: format!("{}\n", dialogs.get(DialogId::MenuTitle01, *language)),
                    style: TextStyle {
                        font: font.clone(),
                        font_size: 100.0,
                        color: Color::WHITE,
                    },
                },
                TextSection {
                    value: dialogs.get(DialogId::MenuTitle02, *language),
                    style: TextStyle {
                        font: font.clone(),
                        font_size: 60.0,
                        color: Color::RED,
                    },
                },
            ],
            alignment: TextAlignment {
                horizontal: HorizontalAlign::Center,
                ..TextAlignment::default()
            },
            ..Text::default()
        },
        ..TextBundle::default()
    };

    let play_text = TextBundle {
        style: Style {
            margin: Rect {
                top: Val::Auto,
                bottom: Val::Percent(5.0),
                ..Rect::default()
            },
            ..Style::default()
        },
        text: Text::with_section(
            dialogs.get(DialogId::MenuPlay, *language),
            TextStyle {
                font: font.clone(),
                font_size: 30.0,
                color: Color::YELLOW,
            },
            TextAlignment::default(),
        ),
        ..TextBundle::default()
    };

    let mut languages_buttons: Vec<(ButtonBundle, TextBundle, bool, Language)> = Vec::new();
    for (i, language) in Language::iter().enumerate() {
        languages_buttons.push((
            ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(100.0), Val::Px(20.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    position: Rect {
                        right: Val::Px(15.0),
                        bottom: Val::Px(i as f32 * 20.0 + 5.0),
                        ..Rect::default()
                    },
                    ..Style::default()
                },
                color: Color::NONE.into(),
                ..ButtonBundle::default()
            },
            TextBundle {
                text: Text::with_section(
                    language.to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.0,
                        color: if language == language {
                            languages_button_colors.selected
                        } else {
                            languages_button_colors.normal
                        },
                    },
                    TextAlignment::default(),
                ),
                ..TextBundle::default()
            },
            Language::default() == language,
            language,
        ));
    }

    commands.spawn_bundle(background_image);
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Style::default()
            },
            color: Color::NONE.into(),
            ..NodeBundle::default()
        })
        // .with_children(|parent| { parent.spawn_bundle(background_image); })
        .with_children(|parent| {
            parent.spawn_bundle(title).insert(DialogId::MenuTitle);
        })
        .with_children(|parent| {
            for (button, text, selected, language) in languages_buttons.into_iter() {
                parent
                    .spawn_bundle(button)
                    .with_children(|parent| {
                        parent.spawn_bundle(text);
                    })
                    .insert(Selected(selected))
                    .insert(language.clone());
            }
        })
        .with_children(|parent| {
            parent.spawn_bundle(play_text).insert(DialogId::MenuPlay);
        })
        .insert(Menu);
}

fn destroy_menu(mut commands: Commands, mut query: Query<Entity, With<Menu>>) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
}

fn game_start(
    mut keyboard_inputs: EventReader<KeyboardInput>,
    mut game_state: ResMut<State<GameState>>,
) {
    if game_state.current() == &GameState::Menu {
        for _ in keyboard_inputs.iter() {
            game_state.set(GameState::Playing).unwrap();
            break;
        }
    }
}

fn language_button_interactions(
    button_colors: Res<LanguagesButtonColors>,
    mut language: ResMut<Language>,
    mut buttons_query: ParamSet<(
        Query<
            (&Interaction, &mut Selected, &Children, &Language),
            (Changed<Interaction>, With<Button>),
        >,
        Query<(&mut Selected, &Children)>,
    )>,
    mut text_query: Query<&mut Text>,
    mut language_event_writer: EventWriter<LanguageChangedEvent>,
) {
    let mut reset_selected = false;

    for (interaction, ..) in buttons_query.p0().iter_mut() {
        if *interaction == Interaction::Clicked {
            reset_selected = true;
        }
    }

    if reset_selected {
        for (mut selected, children) in buttons_query.p1().iter_mut() {
            selected.0 = false;
            let mut text = text_query.get_mut(children[0]).unwrap();
            text.sections[0].style.color = button_colors.normal;
        }
    }

    for (interaction, mut selected, children, button_language) in buttons_query.p0().iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                selected.0 = true;
                text.sections[0].style.color = button_colors.selected;
                *language = *button_language;
                language_event_writer.send(LanguageChangedEvent);
            }
            Interaction::Hovered => {
                if selected.0 {
                    text.sections[0].style.color = button_colors.hovered_selected;
                } else {
                    text.sections[0].style.color = button_colors.hovered;
                }
            }
            Interaction::None => {
                if selected.0 {
                    text.sections[0].style.color = button_colors.selected;
                } else {
                    text.sections[0].style.color = button_colors.normal;
                }
            }
        }
    }
}

fn language_changed(
    mut language_event: EventReader<LanguageChangedEvent>,
    language: Res<Language>,
    dialogs: Res<Dialogs>,
    mut text_query: Query<(&mut Text, &DialogId)>,
) {
    for _ in language_event.iter() {
        for (mut text, dialog_id) in text_query.iter_mut() {
            if *dialog_id == DialogId::MenuTitle {
                text.sections[0].value =
                    format!("{}\n", dialogs.get(DialogId::MenuTitle01, *language));
                text.sections[1].value = dialogs.get(DialogId::MenuTitle02, *language);
            } else {
                text.sections[0].value = dialogs.get(*dialog_id, *language);
            }
        }
    }
}

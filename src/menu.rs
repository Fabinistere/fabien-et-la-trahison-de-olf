use std::time::Duration;

use crate::{
    animations::sprite_sheet_animation::{AnimationDuration, SpriteSheetAnimation},
    in_menu, DialogId, Dialogs, GameState, Language,
};
use bevy::{input::keyboard::KeyboardInput, prelude::*};
use strum::IntoEnumIterator;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LanguageChangedEvent>()
            .init_resource::<LanguagesButtonColors>()
            .add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(OnExit(GameState::Menu), destroy_menu)
            .add_systems(
                Update,
                (
                    // game_start,
                    language_button_interactions,
                    language_changed,
                )
                    .run_if(in_menu),
            );
    }
}

#[derive(Component)]
struct Menu;

#[derive(Component)]
struct Title;

#[derive(Component)]
pub struct UISpriteSheetAnimation {
    pub timer: Timer,
    pub duration: AnimationDuration,
}

#[derive(Event)]
struct LanguageChangedEvent;

#[derive(Component)]
struct Selected(bool);

#[derive(Resource)]
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
            selected: Color::rgb(1., 0.9, 0.),
            hovered_selected: Color::rgb(0.9, 0.8, 0.),
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                                   Systems                                  */
/* -------------------------------------------------------------------------- */

fn destroy_menu(mut commands: Commands, mut query: Query<Entity, With<Menu>>) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
}

fn game_start(
    mut keyboard_inputs: EventReader<KeyboardInput>,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if game_state.get() == &GameState::Menu && keyboard_inputs.iter().next().is_some() {
        next_game_state.set(GameState::Playing);
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
        if *interaction == Interaction::Pressed {
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
            Interaction::Pressed => {
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

/* -------------------------------------------------------------------------- */
/*                                    Setup                                   */
/* -------------------------------------------------------------------------- */

fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    dialogs: Res<Dialogs>,
    languages_button_colors: Res<LanguagesButtonColors>,
    current_language: Res<Language>,
) {
    let font = asset_server.load("fonts/dpcomic.ttf");
    let clouds_spritesheet = asset_server.load("textures/title_screen/clouds_sheet.png");
    let clouds_texture_atlas =
        TextureAtlas::from_grid(clouds_spritesheet, Vec2::new(426., 280.), 10, 1, None, None);
    let clouds_texture_atlas_handle = texture_atlases.add(clouds_texture_atlas.clone());

    // get this texture from the title resource and the default language
    let french_title = asset_server.load("textures/title_screen/title_fr_white.png");
    let moon = asset_server.load("textures/title_screen/moon.png");

    let foreground = asset_server.load("textures/title_screen/static_landscape_big_picture_with_lights.png");

    let title = TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: format!(
                        "{}\n",
                        dialogs.get(DialogId::MenuTitle01, *current_language)
                    ),
                    style: TextStyle {
                        font: font.clone(),
                        font_size: 100.,
                        color: Color::WHITE,
                    },
                },
                TextSection {
                    value: dialogs.get(DialogId::MenuTitle02, *current_language),
                    style: TextStyle {
                        font: font.clone(),
                        font_size: 60.,
                        color: Color::RED,
                    },
                },
            ],
            alignment: TextAlignment::Center,
            ..default()
        },
        ..default()
    };

    let play_text = TextBundle {
        style: Style {
            margin: UiRect {
                top: Val::Auto,
                bottom: Val::Percent(5.),
                ..default()
            },
            ..default()
        },
        text: Text::from_section(
            dialogs.get(DialogId::MenuPlay, *current_language),
            TextStyle {
                font: font.clone(),
                font_size: 30.,
                color: Color::YELLOW,
            },
        ),
        ..default()
    };

    let mut languages_buttons: Vec<(ButtonBundle, TextBundle, bool, Language)> = Vec::new();
    for (i, language) in Language::iter().enumerate() {
        languages_buttons.push((
            ButtonBundle {
                style: Style {
                    width: Val::Px(100.),
                    height: Val::Px(20.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    right: Val::Px(15.),
                    bottom: Val::Px(i as f32 * 20. + 5.),
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            },
            TextBundle {
                text: Text::from_section(
                    language.to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.,
                        color: if *current_language == language {
                            languages_button_colors.selected
                        } else {
                            languages_button_colors.normal
                        },
                    },
                ),
                ..default()
            },
            Language::default() == language,
            language,
        ));
    }

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    ..default()
                },
                ..default()
            },
            Name::new("Menu"),
            Menu,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    AtlasImageBundle {
                        style: Style {
                            width: Val::Percent(100.),
                            // height: Val::Percent(100.),
                            flex_shrink: 0.,
                            ..default()
                        },
                        texture_atlas: clouds_texture_atlas_handle,
                        texture_atlas_image: UiTextureAtlasImage::default(),
                        ..default()
                    },
                    SpriteSheetAnimation {
                        start_index: 0,
                        end_index: clouds_texture_atlas.len() - 1,
                        duration: AnimationDuration::Infinite,
                        timer: Timer::new(Duration::from_millis(200), TimerMode::Repeating),
                    },
                    Name::new("Art - Title Screen"),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    flex_shrink: 0.,
                                    width: Val::Percent(100.),
                                    ..default()
                                },
                                ..default()
                            },
                            Name::new("Title Node"),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                ImageBundle {
                                    image: french_title.into(),
                                    style: Style {
                                        flex_shrink: 0.,
                                        ..default()
                                    },
                                    ..default()
                                },
                                Name::new("French Title"),
                                Title,
                            ));
                        });

                    parent
                        .spawn((
                            ImageBundle {
                                image: foreground.into(),
                                style: Style {
                                    width: Val::Percent(100.),
                                    flex_shrink: 0.,
                                    right: Val::Percent(100.),
                                    ..default()
                                },
                                ..default()
                            },
                            Name::new("Foreground - Mounts and Manor"),
                        ))
                        .with_children(|parent| {
                            // parent.spawn((
                            //     ImageBundle {
                            //         image: manor_lights.into(),
                            //         style: Style {
                            //             flex_shrink: 0.,
                            //             ..default()
                            //         },
                            //         ..default()
                            //     },
                            //     Name::new("Manor Lights"),
                            // ));
                        });

                    parent.spawn((
                        ImageBundle {
                            image: moon.into(),
                            style: Style {
                                flex_shrink: 0.,
                                ..default()
                            },
                            // transform: Transform::from_translation(translation),
                            ..default()
                        },
                        Name::new("Moon"),
                    ));
                });

            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::ColumnReverse,
                            flex_shrink: 0.,
                            width: Val::Percent(100.),
                            // height: Val::Percent(100.),
                            right: Val::Percent(100.),
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("UI - TitleScreen"),
                ))
                .with_children(|parent| {
                    parent.spawn((title, DialogId::MenuTitle));

                    for (button, text, selected, language) in languages_buttons.into_iter() {
                        parent
                            .spawn((button, Selected(selected), language))
                            .with_children(|parent| {
                                parent.spawn(text);
                            });
                    }

                    parent.spawn((play_text, DialogId::MenuPlay));
                });
        });
}

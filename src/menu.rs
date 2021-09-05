use bevy::{ prelude::*, input::keyboard::KeyboardInput };
use crate::{ GameState, Dialogs, DialogId, Language };

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterials>()
            .add_system_set(
                SystemSet::on_enter(GameState::Menu)
                    .with_system(setup_menu.system())
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Menu)
                    .with_system(destroy_menu.system())
            )
            .add_system(button_interactions.system())
            .add_system(game_start.system());
    }
}

struct Menu;

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.35, 0.35).into()),
        }
    }
}

fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    dialogs: Res<Dialogs>,
    language: Res<Language>,
    button_materials: Res<ButtonMaterials>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    let title = TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: format!("{}\n", dialogs.get(DialogId::MenuTitle01, *language)),
                    style: TextStyle {
                        font: font.clone(),
                        font_size: 100.0,
                        color: Color::WHITE,
                    }
                },
                TextSection {
                    value: dialogs.get(DialogId::MenuTitle02, *language),
                    style: TextStyle {
                        font: font.clone(),
                        font_size: 60.0,
                        color: Color::RED,
                    }
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

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Style::default()
            },
            material: materials.add(Color::NONE.into()),
            ..NodeBundle::default()
        })
        .with_children(|parent| { parent.spawn_bundle(title); })
        .with_children(|parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: Rect::all(Val::Auto),
                        // margin: Rect {
                        //     top: Val::Percent(10.0),
                        //     ..Rect::default()
                        // },
                        ..Style::default()
                    },
                    material: button_materials.normal.clone(),
                    ..ButtonBundle::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                              ":)",
                              TextStyle {
                                  font: font.clone(),
                                  font_size: 40.0,
                                  color: Color::rgb(0.9, 0.9, 0.9),
                              },
                              TextAlignment::default()
                          ),
                          ..TextBundle::default()
                    });
                });
        })
        .with_children(|parent| { parent.spawn_bundle(play_text); })
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

fn button_interactions(
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut material, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Pressed".to_owned();
                *material = button_materials.pressed.clone();
            },
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_owned();
                *material = button_materials.hovered.clone();
            },
            Interaction::None => {
                text.sections[0].value = ":)".to_owned();
                *material = button_materials.normal.clone();
            },
        }
    }
}


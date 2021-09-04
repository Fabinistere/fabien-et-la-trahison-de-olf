use bevy::prelude::*;
use crate::GameState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterials>()
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup_menu.system()))
            .add_system(button_system.system());
    }
}

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
    button_materials: Res<ButtonMaterials>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent | {
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "FABIEN\n".to_owned(),
                            style: TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 100.0,
                                color: Color::WHITE,
                            }
                        },
                        TextSection {
                            value: "et la trahison de Olf".to_owned(),
                            style: TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 60.0,
                                color: Color::RED,
                            }
                        },
                    ],
                    alignment: TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            });
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: Rect {
                            top: Val::Percent(10.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    material: button_materials.normal.clone(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                              ":)",
                              TextStyle {
                                  font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                  font_size: 40.0,
                                  color: Color::rgb(0.9, 0.9, 0.9),
                              },
                              Default::default(),
                          ),
                          ..Default::default()
                    });
                });
        });
}

fn button_system(
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


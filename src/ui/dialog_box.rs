use crate::{constants::ui::dialogs::*, material::CustomMaterial};

use bevy::{
    prelude::*,
    render::RenderWorld,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    ui::{ExtractedUiNode, ExtractedUiNodes},
};
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
#[derive(Component)]
pub struct Scroll;
#[derive(Component, Deref, DerefMut)]
pub struct ScrollTimer(Timer);

pub struct CreateDialogBoxEvent {
    dialog: String,
}

pub struct DialogBoxResources {
    text_font: Handle<Font>,
    background: Handle<Image>,
    chandelier: Handle<Image>,
    stained_glass_closed: Handle<Image>,
    stained_glass_opened: Handle<Image>,
    stained_glass_bars: Handle<Image>,
    stained_glass_panels: Handle<Image>,
    scroll_animation: [Handle<Image>; SCROLL_ANIMATION_FRAMES_NUMBER],
}

pub fn load_textures(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // let scroll_texture = asset_server.load("textures/hud/scroll_animation.png");
    // let scroll_atlas = TextureAtlas::from_grid(scroll_texture, SCROLL_SIZE.into(), 1, 45);

    let mut scroll_animation_frames: [Handle<Image>; SCROLL_ANIMATION_FRAMES_NUMBER];
    for i in 0..SCROLL_ANIMATION_FRAMES_NUMBER {
        scroll_animation_frames[i] =
            asset_server.load(&format!("textures/hud/scroll_animation/frame_{}.png", i));
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
    mut query: Query<(Entity, &mut Animator<Style>), With<DialogBox>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::O) {
        if let Ok((entity, mut animator)) = query.get_single_mut() {
            animator.rewind();
        } else {
            create_dialog_box_event.send(CreateDialogBoxEvent {
                dialog: "Bonjour Florian\nComment vas-tu ?\nJ'ai faim.".to_owned(),
            });
        }
    }
}

pub fn create_dialog_box(
    mut create_dialog_box_events: EventReader<CreateDialogBoxEvent>,
    mut commands: Commands,
    mut custom_material_assets: ResMut<Assets<CustomMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    dialog_box_resources: Res<DialogBoxResources>,
) {
    for CreateDialogBoxEvent { dialog } in create_dialog_box_events.iter() {
        let start_right_offset = -1000.0;

        let dialog_box_tween = Tween::new(
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

        let panels_tween = Tween::new(
            EaseMethod::Linear,
            TweeningType::Once,
            Duration::from_millis(1000),
            UiPositionLens {
                start: Rect {
                    top: Val::Px(0.0),
                    ..Rect::default()
                },
                end: Rect {
                    top: Val::Px(-160.0),
                    ..Rect::default()
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
                    ..Style::default()
                },
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
                        ..Style::default()
                    },
                    ..TextBundle::default()
                });

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

                parent
                    .spawn()
                    .insert_bundle(SpriteSheetBundle {
                        texture_atlas: dialog_box_resources.scroll_animation.clone(),
                        ..SpriteSheetBundle::default()
                    })
                    .insert(child_sprite_style.clone())
                    .insert(CalculatedSize {
                        size: Size::new(300.0, 300.0),
                    })
                    .insert(bevy::ui::FocusPolicy::Block)
                    .insert(bevy::ui::Interaction::None)
                    .insert(bevy::ui::widget::ImageMode::KeepAspect)
                    .insert(bevy::ui::UiColor(Color::rgb(1.0, 1.0, 0.0)))
                    .insert(bevy::ui::FocusPolicy::Block)
                    .insert(Transform::from_xyz(0.0, 0.0, 20.0))
                    .insert(GlobalTransform::default())
                    .insert(Visibility { is_visible: true })
                    .insert(Node::default())
                    .insert(Scroll)
                    .insert(ScrollTimer(Timer::from_seconds(
                        SCROLL_ANIMATION_DELTA_S,
                        true,
                    )));

                parent
                    .spawn()
                    .insert_bundle(MaterialMesh2dBundle {
                        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                        material: custom_material_assets.add(CustomMaterial {
                            brightness: 0.5,
                            progress: 0.5,
                            texture: dialog_box_resources.chandelier.clone(),
                        }),
                        transform: Transform {
                            translation: Vec3::new(0.0, 0.0, 15.0),
                            scale: Vec3::new(1000.0, 1000.0, 0.0),
                            ..Transform::default()
                        },
                        ..MaterialMesh2dBundle::default()
                    })
                    .insert(child_sprite_style.clone())
                    .insert(Transform::from_xyz(0.0, 0.0, 20.0))
                    .insert(GlobalTransform::default())
                    .insert(CalculatedSize {
                        size: Size::new(30.0, 30.0),
                    })
                    .insert(Node::default());
            })
            .insert(DialogBox::new(dialog.clone(), DIALOG_BOX_UPDATE_DELTA))
            .insert(Animator::new(dialog_box_tween));
        commands
            .spawn()
            .insert_bundle(MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                material: custom_material_assets.add(CustomMaterial {
                    brightness: 0.5,
                    progress: 0.5,
                    texture: dialog_box_resources.chandelier.clone(),
                }),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 15.0),
                    scale: Vec3::new(1000.0, 1000.0, 0.0),
                    ..Transform::default()
                },
                ..MaterialMesh2dBundle::default()
            })
            .insert(Style::default())
            .insert(Transform::from_xyz(0.0, 0.0, 20.0))
            .insert(GlobalTransform::default())
            .insert(CalculatedSize {
                size: Size::new(300.0, 300.0),
            })
            .insert(bevy::ui::FocusPolicy::Block)
            .insert(bevy::ui::Interaction::None)
            .insert(Node::default());

        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: dialog_box_resources.scroll_animation.clone(),
                ..SpriteSheetBundle::default()
            })
            .insert(Style::default())
            .insert(CalculatedSize {
                size: Size::new(300.0, 300.0),
            })
            .insert(Transform::from_xyz(0.0, 0.0, 20.0))
            .insert(GlobalTransform::default())
            .insert(bevy::ui::FocusPolicy::Block)
            .insert(bevy::ui::Interaction::None)
            .insert(bevy::ui::widget::ImageMode::KeepAspect)
            .insert(bevy::ui::UiColor(Color::rgb(1.0, 1.0, 0.0)))
            .insert(bevy::ui::FocusPolicy::Block)
            .insert(Visibility { is_visible: true })
            .insert(Node::default())
            .insert(Scroll)
            .insert(ScrollTimer(Timer::from_seconds(
                SCROLL_ANIMATION_DELTA_S,
                true,
            )));
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
    mut commands: Commands,
    mut scroll_query: Query<
        (
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
            &mut ScrollTimer,
            Entity,
        ),
        With<Scroll>,
    >,
) {
    for (mut sprite, texture_atlas_handle, mut timer, entity) in scroll_query.iter_mut() {
        timer.tick(time.delta());
        let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();

        if timer.finished() {
            sprite.index += 1;

            if sprite.index >= texture_atlas.textures.len() - 1 {
                commands.entity(entity).remove::<ScrollTimer>();
            }
        }
    }
}

pub fn extract_atlas_uinodes(
    mut render_world: ResMut<RenderWorld>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    images: Res<Assets<Image>>,
    uinode_query: Query<(
        &Node,
        &GlobalTransform,
        Option<&UiColor>,
        &Handle<TextureAtlas>,
        &Visibility,
        Option<&CalculatedClip>,
    )>,
) {
    let mut extracted_uinodes = render_world.get_resource_mut::<ExtractedUiNodes>().unwrap();
    for (uinode, transform, color, ui_atlas, visibility, clip) in uinode_query.iter() {
        // Skips if the node is not visible or if its size is set to zero (e.g. when a parent is set to `Display::None`)
        if !visibility.is_visible || uinode.size == Vec2::ZERO {
            continue;
        }
        let atlas = texture_atlases
            .get(ui_atlas.clone_weak())
            .unwrap_or_else(|| {
                panic!(
                    "Failed to retrieve `TextureAtlas` from handle {:?}",
                    ui_atlas
                )
            });
        // Skip loading images
        if !images.contains(atlas.texture.clone_weak()) {
            continue;
        }
        let image = atlas.texture.clone_weak();
        let atlas_size = Some(atlas.size);
        let color = color.map_or(Color::default(), |c| c.0);
        let rect =
            atlas.textures.get(0).copied().unwrap_or_else(|| {
                panic!("TextureAtlas {:?} as no texture at index {}", ui_atlas, 0)
            });
        extracted_uinodes.uinodes.push(ExtractedUiNode {
            transform: transform.compute_matrix(),
            color,
            rect,
            image,
            atlas_size,
            clip: clip.map(|clip| clip.clip),
        });
    }
}

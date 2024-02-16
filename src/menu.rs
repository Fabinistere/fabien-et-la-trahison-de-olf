use std::time::Duration;

use crate::{
    animations::sprite_sheet_animation::{
        AnimationDuration, SpriteSheetAnimation, SpriteSheetIndex,
    },
    in_menu, DialogId, Dialogs, GameState, Language,
};
use bevy::{input::keyboard::KeyboardInput, prelude::*, window::WindowResized};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
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
                (game_start, language_button_interactions, language_changed).run_if(in_menu),
            )
            .add_systems(PostUpdate, adjust_art_height.run_if(in_menu));
    }
}
#[derive(Component)]
struct Menu;

#[derive(Component)]
struct ArtMenu;

#[derive(Component)]
pub struct Title;

#[derive(Component)]
pub enum TitleState {
    /// Behind the moutains
    Hidden,
    /// At top position
    FlexTop,
    /// At bot position
    FlexBot,
}

#[derive(Component)]
pub struct Smoke;

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

#[derive(Deref, DerefMut, Reflect, Component)]
pub struct ManorLightsTimer {
    pub timer: Timer,
}

#[derive(Copy, Clone, Default, Reflect, Debug, Component)]
pub enum ManorLightsPattern {
    #[default]
    FullLights,
    TowerReset,
    SmallShutdown,
    TopShutdown,
    BotShutdown,
    LeftShutdown,
}

/// Won't draw `ManorLightsPattern::FullLights`
impl Distribution<ManorLightsPattern> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ManorLightsPattern {
        match rng.gen_range(1..=5) {
            1 => ManorLightsPattern::TowerReset,
            2 => ManorLightsPattern::SmallShutdown,
            3 => ManorLightsPattern::TopShutdown,
            4 => ManorLightsPattern::BotShutdown,
            _ => ManorLightsPattern::LeftShutdown,
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
    asset_server: Res<AssetServer>,

    mut text_query: Query<(&mut Text, &DialogId)>,
    mut ui_image_query: Query<(&mut UiImage, &DialogId), With<Title>>,
) {
    for LanguageChangedEvent in language_event.iter() {
        for (mut text, dialog_id) in &mut text_query {
            text.sections[0].value = dialogs.get(*dialog_id, *language);
        }
        for (mut image, dialog_id) in &mut ui_image_query {
            if *dialog_id == DialogId::MenuTitle {
                *image = match *language {
                    Language::Francais => asset_server
                        .load("textures/title_screen/Francais.png")
                        .into(),
                    Language::English => asset_server
                        .load("textures/title_screen/English.png")
                        .into(),
                    Language::FabienAncien => asset_server
                        .load("textures/title_screen/Fabien Ancien.png")
                        .into(),
                }
            }
        }
    }
}

/// Keeps the art in a 16/9 resolution.
///
/// TODO: Move the art to always be at the bottom of the screen.
/// Note that you can let the position still to let the player reveal a bit of the bottom of the mountains (about 5.5%)
/// So, `bottom` never above 5.5 (or 0 if we keep the `top` in the setup style).
fn adjust_art_height(
    mut resize_reader: EventReader<WindowResized>,
    mut query: Query<&mut Style, With<ArtMenu>>,
) {
    for WindowResized {
        window: _,
        width,
        height,
    } in resize_reader.iter()
    {
        let mut style = query.single_mut();
        info!(
            target: "misc",
            "window's width: {} * {} / window's height {} = {}",
            width,
            (9. / 16.),
            height,
            (*width * (9. / 16.)) / *height
        );

        style.height = Val::Percent(100. * (*width * (9. / 16.)) / *height);
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

    let smoke_spritesheet = asset_server.load("textures/title_screen/smoke_sheet.png");
    let smoke_texture_atlas =
        TextureAtlas::from_grid(smoke_spritesheet, Vec2::new(426., 280.), 17, 1, None, None);
    let smoke_texture_atlas_handle = texture_atlases.add(smoke_texture_atlas.clone());

    let french_title = asset_server.load("textures/title_screen/Francais.png");
    let moon = asset_server.load("textures/title_screen/moon.png");

    let foreground = asset_server.load("textures/title_screen/static_landscape_big_picture.png");
    let manor_lights_spritesheet =
        asset_server.load("textures/title_screen/manor_lights_sheet.png");
    let manor_lights_texture_atlas = TextureAtlas::from_grid(
        manor_lights_spritesheet,
        Vec2::new(426., 280.),
        21,
        1,
        None,
        None,
    );
    let manor_lights_texture_atlas_handle = texture_atlases.add(manor_lights_texture_atlas.clone());

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    // TODO: Animate Transi Start
                    // bottom: Val::Percent(-40.),
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
                            align_self: AlignSelf::FlexEnd,
                            ..default()
                        },
                        texture_atlas: clouds_texture_atlas_handle,
                        texture_atlas_image: UiTextureAtlasImage::default(),
                        ..default()
                    },
                    SpriteSheetAnimation {
                        index: SpriteSheetIndex::new(0, clouds_texture_atlas.len() - 1),
                        duration: AnimationDuration::Infinite,
                        timer: Timer::new(Duration::from_millis(150), TimerMode::Repeating),
                    },
                    Name::new("Art - Title Screen"),
                    ArtMenu,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        AtlasImageBundle {
                            style: Style {
                                width: Val::Percent(100.),
                                top: Val::Percent(16.5),
                                flex_shrink: 0.,
                                align_self: AlignSelf::FlexEnd,
                                ..default()
                            },
                            texture_atlas: smoke_texture_atlas_handle,
                            texture_atlas_image: UiTextureAtlasImage::default(),
                            ..default()
                        },
                        SpriteSheetAnimation {
                            index: SpriteSheetIndex::new(0, smoke_texture_atlas.len() - 1),
                            duration: AnimationDuration::Infinite,
                            timer: Timer::new(Duration::from_millis(100), TimerMode::Repeating),
                        },
                        Name::new("Smoke"),
                        Smoke,
                    ));

                    // TODO: Test Anim Moon
                    parent.spawn((
                        ImageBundle {
                            image: moon.into(),
                            style: Style {
                                flex_shrink: 0.,
                                width: Val::Percent(100.),
                                right: Val::Percent(53.55),
                                bottom: Val::Percent(50.5),
                                align_self: AlignSelf::FlexEnd,
                                ..default()
                            },
                            ..default()
                        },
                        Name::new("Moon"),
                    ));

                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    flex_shrink: 0.,
                                    width: Val::Percent(100.),
                                    right: Val::Percent(200.),
                                    bottom: Val::Px(750.),
                                    align_self: AlignSelf::FlexEnd,
                                    ..default()
                                },
                                transform: Transform::from_scale((4., 4., 4.).into()),
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
                                TitleState::FlexTop,
                                DialogId::MenuTitle,
                            ));
                        });
                    // REFACTOR: foreground mounts and background (title in between fade in + bottom raising at start)
                    parent
                        .spawn((
                            ImageBundle {
                                image: foreground.into(),
                                style: Style {
                                    flex_shrink: 0.,
                                    width: Val::Percent(100.),
                                    // min_height: Val::Px(1200.),
                                    // max_height: Val::Px(1200.),
                                    right: Val::Percent(300.),
                                    top: Val::Percent(16.5),
                                    align_self: AlignSelf::FlexEnd,
                                    ..default()
                                },
                                ..default()
                            },
                            Name::new("Foreground - Mounts and Manor"),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                AtlasImageBundle {
                                    style: Style {
                                        width: Val::Percent(100.),
                                        flex_shrink: 0.,
                                        align_self: AlignSelf::FlexEnd,
                                        ..default()
                                    },
                                    texture_atlas: manor_lights_texture_atlas_handle,
                                    texture_atlas_image: UiTextureAtlasImage::default(),
                                    ..default()
                                },
                                ManorLightsTimer {
                                    timer: Timer::new(
                                        Duration::from_millis(200),
                                        TimerMode::Repeating,
                                    ),
                                },
                                ManorLightsPattern::default(),
                                Name::new("Manor Lights"),
                            ));
                        });
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
                    for (i, language) in Language::iter().enumerate() {
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        width: Val::Px(100.),
                                        height: Val::Px(20.),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        position_type: PositionType::Absolute,
                                        right: Val::Px(15.),
                                        bottom: Val::Px(i as f32 * 40. + 5.),
                                        ..default()
                                    },
                                    background_color: Color::NONE.into(),
                                    ..default()
                                },
                                Selected(Language::default() == language),
                                language,
                                Name::new(format!("{}", language)),
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle {
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
                                });
                            });
                    }

                    parent.spawn((
                        TextBundle {
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
                        },
                        DialogId::MenuPlay,
                        Name::new("Play Text"),
                    ));
                });
        });
}

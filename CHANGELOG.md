# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## The great migration - Bevy 0.18

### Bevy `0.15`

[Migration Guide Bevy 0.14 -> 0.15](https://bevy.org/learn/migration-guides/0-14-to-0-15/)

#### regressions

- [x] language buttons aren't vertically centered
- [x] no animation in the title screen
- [ ] the z of characters are off; hide behind cover now offset the character's z (example: fake plant wall or north hall's walls)
- [ ] `MultipleEntities(ui::dialog_scrolls::MonologPanel)` when pressing the interactive key too fast after the dialog wall opened

#### changes

- Dependencies
  - bevy_rapier_2d `0.28` - [bevy changelog](https://github.com/dimforge/bevy_rapier/blob/v0.28.0/CHANGELOG.md#v0280-09-december-2024) and [changelog](https://github.com/dimforge/rapier/blob/master/CHANGELOG.md#v0280-08-august-2025)
    - `ResMut<mut RapierContext>` -> `WriteDefaultRapierContext`
    - `Res<RapierContext>` -> `ReadDefaultRapierContext`
    - Access to `RapierConfiguration` and `SimulationToRenderTime` should query for it on the responsible entity owning the `RenderContext`.

    ```rs
    // 0.14
    fn game_setup(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
        rapier_config.gravity = Vect::ZERO;
    }

    // 0.15
    fn game_setup(mut commands: Commands, mut rapier_config: Query<&mut RapierConfiguration>) {
        let mut rapier_config = rapier_config.single_mut();
        rapier_config.gravity = Vect::ZERO;
    }
    ```

  - bevy-inspector-egui `0.30` - [changelog](https://github.com/jakobhellermann/bevy-inspector-egui/compare/v0.25.0...v0.30.0)
  - bevy_tweening `0.12` - [changelog](https://github.com/djeedai/bevy_tweening/blob/main/CHANGELOG.md#0120---2024-12-07)
    - Replaced `interpolation::EaseFunction` with `bevy_math::EaseFunction`, and removed the `interpolation` crate dependency.
- ECS
- Assets
  - [Split `TextureAtlasSources` out of `TextureAtlasLayout` and make `TextureAtlasLayout` serializable](https://bevy.org/learn/migration-guides/0-14-to-0-15/#split-textureatlassources-out-of-textureatlaslayout-and-make-textureatlaslayout-serializable)

    ```rs
    // instead of performing a reverse-lookup from the layout, like so:
    let atlas_layout = TextureAtlasLayout::from_grid(UVec2::new(426, 280), 10, 1, None, None);
    let atlas_layout_handle = texture_atlases.add(atlas_layout.clone());
    let index = atlas_layout.get_texture_index(&my_handle);
    let handle = TextureAtlas {
        layout: atlas_layout_handle,
        index,
    };

    // You can perform the lookup from the sources instead:
    let atlas_layout = TextureAtlasLayout::from_grid(UVec2::new(426, 280), 10, 1, None, None);
    let atlas_layout = texture_atlases.add(atlas_layout);
    let index = atlas_sources.get_texture_index(&my_handle);
    let handle = TextureAtlas {
        layout: atlas_layout,
        index,
    };
    ```

    - use of `Handle<Image>` and `TextureAtlas` as components on sprite entities will NO LONGER WORK. Use the fields on `Sprite` instead.

   ```rs
   // 0.14
    pub fn system(
        mut query: Query<(Entity, &mut TextureAtlas)>,
    ) {
        for (character, mut atlas) in &mut query {
            let (start_anim, _, _) = &indices.get(character_state).unwrap();
            atlas.index = *start_anim;
        }
    }

   // 0.15
    pub fn system(
        mut query: Query<(Entity, &mut Sprite)>,
    ) {
        for (character, mut sprite) in &mut query {
            let (start_anim, _, _) = &indices.get(character_state).unwrap();

            // from [this example](https://bevy.org/examples/2d-rendering/sprite-animation/)
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = *start_anim;
            }
            // or
            // sprite.texture_atlas.as_mut().unwrap().index = *start_anim;
        }
    }
    ```

  - `SpriteBundle` -> `Sprite`

    ```rs
    // 0.14 
    SpriteBundle {
        texture: hall_up_door,
        transform: Transform::from_xyz(0., 0., UP_DOOR_Z),
        visibility: Visibility::Hidden,
        ..default()
    },

    // 0.15
    Sprite::from_image(hall_up_door),
    Transform::from_xyz(0., 0., UP_DOOR_Z),
    Visibility::Hidden,
    ```

    regex find and replace:

    ```regex
    find:
    Sprite \{[\n\s]+texture: ([^,]*),[\n\s]+transform: (Transform \{[^\}]*[\n\s]+\},)[\n\s]+visibility: (Visibility::\w+,)[\n\s]+..default\(\)[\n\s]+\},


    replace:
    Sprite::from_image($1),
    $2
    $3

    # with no visibility
    Sprite \{[\n\s]+texture: ([^,]*),[\n\s]+transform: (Transform[^\n]*,[\n\s]+)..default\(\)[\n\s]+\},

    Sprite::from_image($1),
    $2
    ```

    ```rs
    // 0.14
    SpriteBundle {
        texture: characters_spritesheet.texture.clone(),

        transform: Transform {
            translation: spawn_position.into(),
            scale: Vec3::splat(NPC_SCALE),
            ..default()
        },
        ..default()
    },
    TextureAtlas {
        layout: characters_spritesheet.atlas_handle.clone(),
        // idle start index
        index: global_animations_indices[spritesheet_line][1].0,
    },
    
    // 0.15
    Sprite {
        texture_atlas: Some(TextureAtlas {
            layout: characters_spritesheet.atlas_handle.clone(),
            // idle start index
            index: global_animations_indices[spritesheet_line][1].0,
        }),
        image: characters_spritesheet.texture.clone(),
        ..default()
    },
    Transform {
        translation: spawn_position.into(),
        scale: Vec3::splat(NPC_SCALE),
        ..default()
    },
    ```

    regex find and replace:

    ```regex
    find:
    Sprite \{[\n\s]+texture: ([^,]*),[\n\s]+transform: (Transform[^,]+,)[\n\s]+..default\(\)[\n\s]+\},[\n\s]+(TextureAtlas \{[^\}]*\},)

    replace:
    Sprite {
        image: $1,
        texture_atlas: Some($3),
        ..default()
    },
    $2
    ```

    - `TextureAtlasLayout`

    ```rs
    // 0.14
    SpriteSheetAnimation {
        start_index: 0,
        end_index: smoke_layout.len() - 1,
        duration: AnimationDuration::Infinite,
        timer: Timer::new(Duration::from_millis(100), TimerMode::Repeating),
    },

    // 0.15
    SpriteSheetAnimation {
        start_index: 0,
        end_index: AA
        duration: AnimationDuration::Infinite,
        timer: Timer::new(Duration::from_millis(100), TimerMode::Repeating),
    },
    ```

- Audio
  - `AudioBundle` -> `AudioPlayer`

    ```rs
    // 0.14
    commands.spawn((
        AudioBundle {
            source: asset_server.load("sounds/FTO_Dracula_theme.ogg"),
            settings: PlaybackSettings::LOOP.with_volume(bevy::audio::Volume::new(0.10)),
        },
        CastleTheme,
    ));

    // 0.15
    commands.spawn((
        AudioPlayer::<AudioSource>(asset_server.load("sounds/FTO_Dracula_theme.ogg")),
        // PlaybackSettings::LOOP.with_volume(bevy::audio::Volume::new(0.10))
        CastleTheme,
    ));
    ```

- Core
  - [Don't re-export `bevy_image` from `bevy_render`](https://bevy.org/learn/migration-guides/0-14-to-0-15/#don-t-re-export-bevy-image-from-bevy-render)
  
    ```rs
    // 0.14
    use bevy::render::texture::Image;
    // 0.15
    use bevy::image::Image;
    ```

- Rendering
  - [Move `Msaa` to component](https://bevy.org/learn/migration-guides/0-14-to-0-15/#move-msaa-to-component)

    ```rs
    // 0.14
    app.insert_resource(Msaa::Off);

    // 0.15
    commands.spawn((Camera2dBundle::default(), Msaa::Off, PlayerCamera));
    ```

- Text
  - [split up `TextStyle`](https://bevy.org/learn/migration-guides/0-14-to-0-15/#split-up-textstyle): `TextStyle` has been renamed to `TextFont` and its color field has been moved to a separate component named `TextColor` which newtypes Color.

    ```rs
    // 0.14
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
                color: Color::Srgba(YELLOW),
            },
        )
        .with_justify(Justify::Left),
        ..default()
    },
    
    // 0.15
    Text::new(dialogs.get(DialogId::MenuPlay, *current_language)),
    TextFont {
        font: font.clone(),
        font_size: 30.,
        ..default()
    },
    TextColor(Color::Srgba(YELLOW)),
    TextLayout::new_with_justify(JustifyText::Left),
    Node {
        margin: UiRect {
            top: Val::Auto,
            bottom: Val::Percent(5.),
            ..default()
        },
        ..default()
    }
    ```

- UI
  - Camera

    ```rs
    // 0.14
    fn game_setup(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
        rapier_config.gravity = Vect::ZERO;

        let mut camera = Camera2dBundle::default();
        camera.projection.scale = 0.1;
        // Higher order camera (UI is displayed onto this one)
        camera.camera.order = 2;
        commands.spawn((camera, PlayerCamera));
    }

    // 0.15
    fn game_setup(mut commands: Commands, mut rapier_config: Query<&mut RapierConfiguration>) {
        let mut rapier_config = rapier_config.single_mut();
        rapier_config.gravity = Vect::ZERO;

        commands.spawn((
            Camera2d,
            // Higher order camera (UI is displayed onto this one)
            Camera {
                order: 2,
                ..default()
            },
            Projection::from(OrthographicProjection {
                scale: 0.1,
                ..OrthographicProjection::default_2d()
            }),
            Msaa::Off,
            PlayerCamera,
        ));
    }
    ```

  - `UiImage` -> `ImageNode`
  
    ```rs
    // 0.14
    ImageBundle {
        style: Style {
            width: Val::Percent(100.),
            // height: Val::Percent(100.),
            flex_shrink: 0.,
            align_self: AlignSelf::FlexEnd,
            ..default()
        },
        image: UiImage {
            texture: clouds_spritesheet,
            ..default()
        },
        ..default()
    },
    
    // 0.15
    ImageNode {
        image: clouds_spritesheet,
        ..default()
    },
    Node {
        width: Val::Percent(100.),
        // height: Val::Percent(100.),
        flex_shrink: 0.,
        align_self: AlignSelf::FlexEnd,
        ..default()
    },
    ```

    ```rs
    // 0.14
    mut scroll_query: Query<
        (&mut UiImage, &mut Scroll, &mut ScrollTimer, Entity),
        (With<MonologPanel>, Without<PlayerChoicePanel>),
    >,

    image = x;

    // 0.15
    mut scroll_query: Query<
        (&mut ImageNode, &mut Scroll, &mut ScrollTimer, Entity),
        (With<MonologPanel>, Without<PlayerChoicePanel>),
    >,

    node.image = x;
    ```

    - `BackgroundColor` no longer tints the color of images in `ImageBundle` or `ButtonBundle`. Set `UiImage::color` to tint images instead.
  
    ```rs
    // 0.14
    commands.spawn((
        Button {
            background_color: NORMAL_BUTTON.into(),
            visibility: Visibility::Hidden,
            ..default()
        }
    ));

    // 0.15
    commands.spawn((
        Button,
        ImageNode {
            color: NORMAL_BUTTON.into(),
            ..default()
        },
        Visibility::Hidden,
    ));
    ```

  - [Migrate UI bundles to required components](https://bevy.org/learn/migration-guides/0-14-to-0-15/#migrate-ui-bundles-to-required-components)
  It will be easiest to migrate if you replace `Node` with `ComputedNode` first, then `Style` with `Node`, and finally `NodeBundle` with `Node`.

    ```rs
    // 0.14
    commands
        .spawn(NodeBundle {
            style: Style {
                 width: Val::Percent(100.),
                 align_items: AlignItems::Center,
                 justify_content: JustifyContent::Center,
                 ..default()
             },
            ..default()
        });
    
    // 0.15
    commands
        .spawn(Node {
            width: Val::Percent(100.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        });
    ```

    for systems:

    ```rs
    // 0.14
    mut title_query: Query<(Entity, &mut Style)>,
    
    // 0.15
    mut title_query: Query<(Entity, &mut Node)>,
    ```

### Bevy `0.14`

[Migration Guide Bevy 0.13 -> 0.14](https://bevyengine.org/learn/migration-guides/0-13-to-0-14/)

- Dependencies
  - bevy_rapier_2d `0.27` - [bevy_rapier changelog](https://github.com/dimforge/bevy_rapier/blob/v0.36.0/CHANGELOG.md#v0270-07-july-2024) and [rapier changelog `0.18` to `0.21`](https://github.com/dimforge/rapier/blob/master/CHANGELOG.md#v0210-23-june-2024)
    - `Rapier_Configuration` doesn't derive `Default`
  - bevy-inspector-egui `0.25` - [changelog](https://github.com/jakobhellermann/bevy-inspector-egui/compare/v0.24.0...v0.25.0)
  - bevy_tweening `0.11`
  - `use bevy::state::app::AppExtStates as _;` to derive States in our dependencies setup
- ECS
  - Colors
    - [colors import](https://bevyengine.org/learn/migration-guides/0-13-to-0-14/#css-constants)

    ```rs
    // 0.13
    let color = Color::BLUE;

    // 0.14
    use bevy::color::palettes::css::BLUE;
    let color = BLUE;
    ```

    - [`rgb` to `srgb`](https://bevyengine.org/learn/migration-guides/0-13-to-0-14/#color-methods)
    - `set_a` to `set_alpha`
  - [Deprecate `SpriteSheetBundle` and `AtlasImageBundle`](https://bevyengine.org/learn/migration-guides/0-13-to-0-14/#deprecate-spritesheetbundle-and-atlasimagebundle)
  - [Use `UVec2` when working with texture dimensions](https://bevyengine.org/learn/migration-guides/0-13-to-0-14/#use-uvec2-when-working-with-texture-dimensions)
  - [Schedules `Startup` runs after `OnEnter`](https://bevyengine.org/learn/migration-guides/0-13-to-0-14/#onenter-state-schedules-now-run-before-startup-schedules)
    We don't have anything to change as we have only Startup system `spawn_camera`; the camera will only be use in `Update`.
    Note that you can create `SubState` with a source.
  - [ECS observers were introduced: mechanisms for immediately responding to events in the world.](https://bevyengine.org/learn/migration-guides/0-13-to-0-14/#generalised-ecs-reactivity-with-observers)
    - [Component Lifecycle Hooks](https://bevy.org/news/bevy-0-14/#ecs-hooks-and-observers): to ???
- States
  - [Derive States requires to add the feature `bevy_state`](https://bevy.org/learn/migration-guides/0-13-to-0-14/#move-state-initialization-methods-to-bevy-state): `bevy = { version = "0.14", features = ["bevy_state"] }` or [`default_features`](https://bevy.org/learn/migration-guides/0-13-to-0-14/#separate-states-from-core-ecs)
    - **If we use `bevy_ecs` directly, we need to add `bevy_state` as dependency**
  - [Make `apply_state_transition` private](https://bevyengine.org/learn/migration-guides/0-13-to-0-14/#make-apply-state-transition-private)

#### WGPU error

```text
ERROR wgpu_hal::gles: wgpu-hal heuristics assumed that the view dimension will be equal to `D2` rather than `D2Array`.
`D2` textures with `depth_or_array_ blayers == 1` are assumed to have view dimension `D2`
`D2` textures with `depth_or_array_layers > 1` are assumed to have view dimension `D2Array`
`D2` textures with `depth_or_array_layers == 6` are assumed to have view dimension `Cube`
`D2` textures with `depth_or_array_layers > 6 && depth_or_array_layers % 6 == 0` are assumed to have view dimension `CubeArray`
```

### Bevy `0.13`

[Migration Guide Bevy 0.12 -> 0.13](https://bevy.org/learn/migration-guides/0-12-to-0-13/)

- Dependencies
  - bevy_rapier_2d `0.25` - [changelog](https://github.com/dimforge/bevy_rapier/blob/master/CHANGELOG.md#v0250-19-feb-2024)
    - Collisions between the character controller and sensors are now disabled by default.
  - bevy-inspector-egui `0.24` - [changelog](https://github.com/jakobhellermann/bevy-inspector-egui/compare/v0.22.0...v0.24.0)
- ECS

  - [Ensure calls to `EventWriter::send` either handle the returned value, or suppress the result with `;`.](https://bevyengine.org/learn/migration-guides/0-12-to-0-13/#update-event-send-methods-to-return-eventid)
  - [Replace `Option<With<T>>` with `Has<T>`](https://bevyengine.org/learn/migration-guides/0-12-to-0-13/#split-worldquery-into-querydata-and-queryfilter)
  - [Rename `Input` to `ButtonInput`](https://bevyengine.org/learn/migration-guides/0-12-to-0-13/#rename-input-to-buttoninput)
  - [Camera-driven UI](https://bevy.org/learn/migration-guides/0-12-to-0-13/#camera-driven-ui)

  ```rust
  // 0.12
  commands.spawn(Camera3dBundle { ... });
  commands.spawn(NodeBundle { ... });

  // 0.13
  let camera = commands.spawn(Camera3dBundle { ... }).id();
  commands.spawn((TargetCamera(camera), NodeBundle { ... }));
  ```

  - [Texture Atlas rework](https://bevyengine.org/learn/migration-guides/0-12-to-0-13/#texture-atlas-rework)

    - `SpriteSheetBundle` now uses a `Sprite` instead of a `TextureAtlasSprite` component

    ```rust
    // before
    fn system_that_handle_animation(
        texture_atlases: Res<Assets<TextureAtlas>>,
        mut characters_query: Query<
            (
                Entity,
                &AnimationIndices,
                &mut AnimationTimer,
                &mut TextureAtlas,
                &Handle<TextureAtlas>,
                &mut CharacterState,
                &Name,
            ),
    ) {
        for (
            _character,
            indices,
            mut timer,
            mut sprite,
            texture_atlas_handle,
            mut character_state,
            name,
        ) in &mut characters_query {
                let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }

    // after
    fn system_that_handle_animation(
        texture_atlases: Res<Assets<TextureAtlasLayout>>, // TextureAtlasLayout
        mut characters_query: Query<
            (
                Entity,
                &AnimationIndices,
                &mut AnimationTimer,
                &mut TextureAtlas,
                // &Handle<TextureAtlas>, // removed
                &mut CharacterState,
                &Name,
            ),
    ) {
        for (
            _character,
            indices,
            mut timer,
            mut atlas, // renamed
            // texture_atlas_handle, // removed
            mut character_state,
            name,
        ) in &mut characters_query {
            let layout = texture_atlases.get(atlas.layout.clone()).unwrap();
            atlas.index = (atlas.index + 1) % layout.textures.len();
        }
    }
    ```

    - `TextureAtlas::from_grid` to `TextureAtlasLayout::from_grid` (remove the first argument, the spritesheet)

    ```rust
    // before
    let manor_lights_spritesheet =
        asset_server.load("textures/title_screen/manor_lights_sheet.png");
    let layout =
        TextureAtlas::from_grid(manor_lights_spritesheet, Vec2::new(426., 280.), 21, 1, None, None);
    let layout_handle = texture_atlases.add(layout.clone());

    commands.spawn(
        AtlasImageBundle {
            texture_atlas_image: UiTextureAtlasImage {
                index: 0,
                flip_x: false,
                flip_y: false,
            },
            texture_atlas: layout_handle,
            texture_atlas_image: UiTextureAtlasImage::default(), // removed
            ..default()
        }
    )

    // after
    let manor_lights_spritesheet =
        asset_server.load("textures/title_screen/manor_lights_sheet.png");
    let layout =
        TextureAtlasLayout::from_grid(Vec2::new(426., 280.), 21, 1, None, None);
    let layout_handle = texture_atlases.add(layout.clone());

     commands.spawn(
        AtlasImageBundle {
            texture_atlas: TextureAtlas {
                layout: layout_handle,
                index: 0
            },
            image: UiImage {
                texture: manor_lights_spritesheet,
                flip_x: false,
                flip_y: false,
            },
            ..default()
        }
    )
    ```

    - `UiTextureAtlasImage` was removed. The `AtlasImageBundle` is now identical to `ImageBundle` with an additional `TextureAtlas`
      `mut atlases: ResMut<Assets<TextureAtlas>>` to `mut atlases: ResMut<Assets<TextureAtlasLayout>>`
      - `mut ui_anim_query: Query<&mut UiTextureAtlasImage>` to `mut ui_anim_query: Query<&mut ???>`

  - [Renamed `App::add_state` to `init_state`.](https://bevyengine.org/learn/migration-guides/0-12-to-0-13/#add-insert-state-to-app)
  - [`KeyCode` rename](https://bevyengine.org/learn/migration-guides/0-12-to-0-13/#update-winit-dependency-to-0-29)
    - `KeyCode::W` -> `KeyCode::KeyW`
    - `KeyCode::Up` -> `KeyCode::ArrowUp`
    - `KeyCode::Key1` -> `KeyCode::Digit1`
  - Remove the ability to ignore global volume. The option to ignore the global volume using `Volume::Absolute` has been removed and `Volume` now stores the volume level directly, removing the need for the `VolumeLevel` struct. `Volume::new_absolute` and `Volume::new_relative` were removed.
    Use `Volume::new(0.5)`. `bevy::audio::Volume::Relative(VolumeLevel::new(0.10))` to `bevy::audio::Volume::new(0.10)`
  - Rename `TextAlignment` to `JustifyText`.
    - `Text::with_alignment` has been renamed to `Text::with_justify`
  - `KeyboardInput.scan_code` renamed to `KeyboardInput.logical_key`

### Bevy `0.12`

[Migration Guide Bevy 0.11 -> 0.12](https://bevy.org/learn/migration-guides/0-11-to-0-12/)

- Dependencies
  - bevy_rapier_2d `0.23` - [changelog](https://github.com/dimforge/bevy_rapier/blob/master/CHANGELOG.md#0230)
  - [Removed anyhow](https://bevy.org/learn/migration-guides/0-11-to-0-12/#removed-anyhow)
- ECS
  - no longer need to manually configure the `ChangeWatcher` in the `AssetPlugin` as it is now configured automatically when the feature is enabled.
  - [`EventReader::iter` to `EventReader::read`](https://bevyengine.org/learn/migration-guides/0-11-to-0-12/#refactor-eventreader-iter-to-read)
  - [`RemovedComponents::iter` to `RemovedComponents::read`](https://bevyengine.org/learn/migration-guides/0-11-to-0-12/#rename-removedcomponents-iter-iter-with-id-to-read-read-with-id)
  - [If you were using Bevy's `bevy_dylib` feature, use Bevy's `dynamic_linking` feature instead.](https://bevyengine.org/learn/migration-guides/0-11-to-0-12/#remove-the-bevy-dylib-feature)
- Have a separate implicit viewport node per root node + make viewport node Display::Grid #
  UI
  - The implicit viewport node (which contains each user-specified root node) is now Display::Grid with align_items and justify_items both set to Start. You may need to add height: Val::Percent(100.) to your root nodes if you were previously relying on being implicitly set.

### Rand `0.10`

- [Migration Guide Rand 0.8 to 0.9](https://rust-random.github.io/book/update-0.9.html)
  - `thread_rng().gen_range` to `rng().random_range`
  - `thread_rng().gen` to `rng().random`
  - The old trait `SliceRandom` has been split into three traits: `IndexedRandom`, `IndexedMutRandom` and `SliceRandom`.
- [Migration Guide Rand 0.9 to 0.10](https://rust-random.github.io/book/update-0.10.html)

## NPCs Update - [v0.4.0](https://github.com/Fabinistere/fabien-et-la-trahison-de-olf/releases/tag/v0.4.0) - 2023-09-24

[![v0.4.0](https://img.shields.io/badge/v0.4.0-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/Fabinistere/fabien-et-la-trahison-de-olf/releases/tag/v0.4.0)](https://github.com/Fabinistere/fabien-et-la-trahison-de-olf/releases/tag/v0.4.0)

### Preview

<https://github.com/Fabinistere/fabien-et-la-trahison-de-olf/assets/73140258/151188b8-ee5b-40d0-ac3a-e92cad0880fe>

<https://github.com/Fabinistere/fabien-et-la-trahison-de-olf/assets/73140258/d5eb97c4-7fd5-4f6d-ad8c-9d50d57a14d1>

<https://github.com/Fabinistere/fabien-et-la-trahison-de-olf/assets/73140258/9af22dd1-0d50-4121-86dd-66f67e0946c1>

### Landmarks

#### Added

- Landmarks Sensor
  - Hall
  - Temple
  - Secret Room
- Landmarks forcing

#### Changed

- globalize `PlayerLocation` to `Location`
  - The component `Location` is given to
    - Each npc
    - Each landmark
  - `Location` is still a `State` to keep the quick and easy access to the player's location
- globalize Spritesheet Indices

### Dialog

- YML Dialog and reorganize ui components

### NPCs' Behavior

#### Refactored

- Follow Behavior
  - add and use a `FollowRangeSensor` to detect the follow_target's hitbox
- JustWalk to LandmarkSeeking

#### Added

- Detection Behavior
  - `TargetSeeker`
  - `DetectionRangeSensor` used to analyze all entering characters' hitbox and compare with their `TargetType`
    - if it correspond: Deactivate this sensor and start the Chase Behavior with the component `Chaser`.
- Chase Behavior
  - `Chaser`
  - `PursuitRangeSensor` used to analyze all exiting characters' hitbox and compare with their `Chaser`'s `target`
    - if it correspond: Deactivate this sensor and remove the Chase Behavior.
  - The `CharacterCloseSensor` used to detect all entering `Chaser`s hitbox and start a Combat if the `Chaser`'s `target` is the parent of the `CharacterCloseSensor`.

## Map, Title Screen and Animation Update - [v0.3.9](https://github.com/Fabinistere/fabien-et-la-trahison-de-olf/releases/tag/v0.3.9) - 2023-08-31

[![v0.3.9](https://img.shields.io/badge/v0.3.9-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/Fabinistere/fabien-et-la-trahison-de-olf/releases/tag/v0.3.9)](https://github.com/Fabinistere/fabien-et-la-trahison-de-olf/releases/tag/v0.3.9)
[![**Full Commits History**](https://img.shields.io/badge/GitHubLog-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/fabinistere/fabien-et-la-trahison-de-olf/commits/v0.3.9)](https://github.com/fabinistere/fabien-et-la-trahison-de-olf/commits/v0.3.9)
![Demo](https://img.shields.io/badge/Demo-gray?style=flat&logo=darkreader&logoColor=181717&link=https://fabinistere.github.io/fabien-et-la-trahison-de-olf/)

### Preview

[![Title Banner](https://github.com/Fabinistere/fabien-et-la-trahison-de-olf/assets/73140258/4cf67c3a-f587-4604-9990-db44143e6fcc)](https://fabinistere.github.io/fabien-et-la-trahison-de-olf/)

<https://github.com/Fabinistere/fabien-et-la-trahison-de-olf/assets/73140258/07cf5f8c-ac60-4d46-be3b-9365b00c6abd>

The Title screen (you can zoom) and the music doesn't quite work in the [web demo](https://fabinistere.github.io/fabien-et-la-trahison-de-olf/).

### Added

- Castle Theme Music

### Removed

- `Location` States to focus on only one location state: `PlayerLocation`

### Changed

- Player Movement
  - random idle
  - top/bot walk anim
  - spritesheet handle
- Menu
  - Art Anim (randomness)
    - smoke
    - lights
    - clouds
  - Scaling
  - Language
- Map
  - Z refactor
  - Map updated to v4.0.0
    - Map design
      - new Hall
        - new lights
      - new secret room
        - flower panel
      - new Temple
        - new floor
        - new throne
        - new stairs
        - new column
        - new props
        - new statues
        - new plants
        - new lights
        - replace curtains by banners
    - Collider generated by sprite
      - TODO: create more accurate sprite
    - Locations Sensor to detect a location change
    - add Balcony Cover
  - Sprite are now 1pixel scale
    The camera has been adapted to this size

### Fixed

- [All previous layer glitches](https://github.com/Fabinistere/fabien-et-la-trahison-de-olf/issues/2)

## Bevy Migration - [v0.3.8](https://github.com/Fabinistere/fabien-et-la-trahison-de-olf/releases/tag/v0.3.8) - 2023-08-18

[![v0.3.8](https://img.shields.io/badge/v0.2.0alpha-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/Fabinistere/fabien-et-la-trahison-de-olf/releases/tag/v0.3.8)](https://github.com/Fabinistere/fabien-et-la-trahison-de-olf/releases/tag/v0.3.8)
[![**Full Commits History**](https://img.shields.io/badge/GitHubLog-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/fabinistere/fabien-et-la-trahison-de-olf/commits/v0.3.8)](https://github.com/fabinistere/fabien-et-la-trahison-de-olf/commits/v0.3.8)

- [Migration Guide Bevy 0.10 -> 0.11](https://bevyengine.org/learn/migration-guides/0-10-to-0-11/)
- _not needed_ [Changelog Bevy Rapier 0.21 -> 0.22](https://github.com/dimforge/bevy_rapier/blob/master/CHANGELOG.md#0220-10-july-2023)

### Added

- [![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/fabinistere/fabien-et-la-trahison-de-olf#license)

### [Bevy 0.11](https://bevyengine.org/learn/migration-guides/0-10-to-0-11/) Migration

- ECS
  - `in_set(OnUpdate(*))` -> `run_if(in_state(*))`
  - Add the `#[derive(Event)]` macro for events.
  - Allow tuples and single plugins in `add_plugins`, deprecate `add_plugin`
  - [Schedule-First: the new and improved `add_systems`](https://bevyengine.org/learn/migration-guides/0-10-to-0-11/#schedule-first-the-new-and-improved-add-systems)
- UI
  - Flatten UI Style properties that use Size + remove Size
    - The `size`, `min_size`, `max_size`, and `gap` properties have been replaced by the `width`, `height`, `min_width`, `min_height`, `max_width`, `max_height`, `row_gap`, and `column_gap` properties. Use the new properties instead.
  - [Remove `Val::Undefined`](https://bevyengine.org/learn/migration-guides/0-10-to-0-11/#remove-val-undefined)
    - `Val::Undefined` has been removed. Bevy UI’s behavior with default values should remain the same.
      The default values of `UiRect`’s fields have been changed to `Val::Px(0.)`.
      `Style`’s position field has been removed. Its `left`, `right`, `top` and `bottom` fields have been added to `Style` directly.
      For the `size`, `margin`, `border`, and `padding` fields of `Style`, `Val::Undefined` should be replaced with `Val::Px(0.)`.
      For the `min_size`, `max_size`, `left`, `right`, `top` and `bottom` fields of `Style`, `Val::Undefined` should be replaced with `Val::Auto`
  - [Rename keys like `LAlt` to `AltLeft`](https://bevyengine.org/learn/migration-guides/0-10-to-0-11/#rename-keys-like-lalt-to-altleft)
  - [Delay asset hot reloading](https://bevyengine.org/learn/migration-guides/0-10-to-0-11/#delay-asset-hot-reloading)
  - [`Interaction::Clicked` replaced by `Interaction::Pressed`](https://bevyengine.org/learn/migration-guides/0-10-to-0-11/#rename-interaction-clicked-interaction-pressed)
- Dependencies
  - bevy_rapier_2d `0.22`
  - bevy_tweening `0.8`

### [Bevy 0.10](https://bevyengine.org/learn/migration-guides/0-9-to-0-10/) Migration

- Dependencies
  - bevy_rapier2d [0.21](https://github.com/dimforge/bevy_rapier/blob/master/CHANGELOG.md#0210--07-march-2023)
    - feature `debug-render` change to `debug-render-2d`
  - Remove bevy-web-resizer dependency: [Note: this functionality is now built into Bevy and this crate will no longer be maintained.](https://github.com/frewsxcv/bevy-web-resizer#readme)
- ECS
  - [Migrate engine to Schedule v3 (stageless)](https://bevyengine.org/learn/migration-guides/0-9-to-0-10/#migrate-engine-to-schedule-v3-stageless)
  - [System sets (Bevy 0.9)](https://bevyengine.org/learn/migration-guides/0-9-to-0-10/#system-sets-bevy-0-9)
  - [States](https://bevyengine.org/learn/migration-guides/0-9-to-0-10/#states)
- UI
  - [Windows as Entities](https://bevyengine.org/learn/migration-guides/0-9-to-0-10/#windows-as-entities)
  - [Remove VerticalAlign from TextAlignment](https://bevyengine.org/learn/migration-guides/0-9-to-0-10/#remove-verticalalign-from-textalignment)
  - [Remove the `GlobalTransform::translation_mut` method](https://bevyengine.org/learn/migration-guides/0-9-to-0-10/#remove-the-globaltransform-translation-mut-method)

### [Bevy 0.9](https://bevyengine.org/learn/migration-guides/0-8-to-0-9/) Migration

- ECS
  - [Make `Resource` trait opt-in, requiring `#[derive(Resource)]` V2](https://bevyengine.org/learn/migration-guides/0-8-to-0-9/#make-resource-trait-opt-in-requiring-derive-resource-v2)
  - [Spawn now takes a Bundle](https://bevyengine.org/learn/migration-guides/0-8-to-0-9/#spawn-now-takes-a-bundle)
  - [Accept Bundles for insert and remove. Deprecate `insert`/`remove_bundle`](https://bevyengine.org/learn/migration-guides/0-8-to-0-9/#accept-bundles-for-insert-and-remove-deprecate-insert-remove-bundle)
  - [Replace the `bool` argument of `Timer` with `TimerMode`](https://bevyengine.org/learn/migration-guides/0-8-to-0-9/#replace-the-bool-argument-of-timer-with-timermode)
  - [Add global time scaling](https://bevyengine.org/learn/migration-guides/0-8-to-0-9/#add-global-time-scaling)
- UI
  - TODO: [Change UI coordinate system to have origin at top left corner](https://bevyengine.org/learn/migration-guides/0-8-to-0-9/#change-ui-coordinate-system-to-have-origin-at-top-left-corner)
  - [Rename `UiColor` to `BackgroundColor`](https://bevyengine.org/learn/migration-guides/0-8-to-0-9/#rename-uicolor-to-backgroundcolor)
  - [Make the default background color of `NodeBundle` transparent](https://bevyengine.org/learn/migration-guides/0-8-to-0-9/#make-the-default-background-color-of-nodebundle-transparent)
    - remove useless field (completed by `..NodeBundle::default()`)
  - [Merge TextureAtlas::from_grid_with_padding into TextureAtlas::from_grid through option arguments](https://bevyengine.org/learn/migration-guides/0-8-to-0-9/#merge-textureatlas-from-grid-with-padding-into-textureatlas-from-grid-through-option-arguments)
- Dependency
  - bevy_tweening 0.6
    - [Removed the `tweening_type` parameter from the signature of `Tween<T>::new()`; use `with_repeat_count()` and `with_repeat_strategy()` instead.](https://github.com/djeedai/bevy_tweening/blob/main/CHANGELOG.md#changed-2)

## Curtains Update - [v0.2.0-alpha](https://github.com/Fabinistere/fabien-et-la-trahison-de-olf/releases/tag/v0.2.0-alpha) - 2022-05-29

[![v0.2.0-alpha](https://img.shields.io/badge/v0.2.0alpha-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/Fabinistere/fabien-et-la-trahison-de-olf/releases/tag/v0.2.0-alpha)](https://github.com/Fabinistere/fabien-et-la-trahison-de-olf/releases/tag/v0.2.0-alpha)
[![**Full Commits History**](https://img.shields.io/badge/GitHubLog-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/fabinistere/fabien-et-la-trahison-de-olf/commits/v0.2.0-alpha)](https://github.com/fabinistere/fabien-et-la-trahison-de-olf/commits/v0.2.0-alpha)

### Preview

![Physics](https://github.com/Fabinistere/fabien-et-la-trahison-de-olf/assets/73140258/89c2279a-9a56-4708-8812-220a8ea0645e)

### Feature

- Play with curtains

### Added

- Spawn in the Temple

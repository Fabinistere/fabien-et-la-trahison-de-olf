use bevy::{prelude::*, utils::Duration};
use bevy_rapier2d::prelude::*;

use crate::{
    animations::{
        functions::{ease_in_sine, ease_out_sine},
        sprite_sheet_animation::{AnimationDuration, SpriteSheetAnimation},
        Fade, FadeType,
    },
    collisions::{TesselatedCollider, TesselatedColliderConfig},
    constants::{locations::secret_room::*, BACKGROUND_COLOR_INGAME},
    locations::temple::{Location, LocationSensor, OverlappingEntity, WallCollider},
};

/* -------------------------------------------------------------------------- */
/*                                 Components                                 */
/* -------------------------------------------------------------------------- */

/// Used to detect collision with player to manage the secret room cover.
#[derive(Component)]
pub struct SecretRoomSensor;

#[derive(Component)]
pub struct SecretRoom;

#[derive(Component)]
pub struct SecretRoomCover;

#[derive(Component)]
pub struct FlowerPanel;

#[derive(Component)]
pub struct FlowerPot;

#[derive(Component)]
pub struct SecondLayerFakeWall;

/* -------------------------------------------------------------------------- */
/*                                   Events                                   */
/* -------------------------------------------------------------------------- */

#[derive(Event)]
pub struct SecretRoomTriggerEvent {
    pub started: bool,
}

#[derive(Event)]
pub struct RemoveSecretRoomCoverEvent;

#[derive(Event)]
pub struct AddSecretRoomCoverEvent;

/* -------------------------------------------------------------------------- */
/*                                   Systems                                  */
/* -------------------------------------------------------------------------- */

pub fn remove_secret_room_cover(
    mut remove_secret_room_cover_event: EventReader<RemoveSecretRoomCoverEvent>,
    mut commands: Commands,
    mut secret_room_cover_query: Query<(Entity, Option<&mut Fade>), With<SecretRoomCover>>,
) {
    for RemoveSecretRoomCoverEvent in remove_secret_room_cover_event.iter() {
        if let Ok((cover_entity, fade_opt)) = secret_room_cover_query.get_single_mut() {
            if let Some(mut fade) = fade_opt {
                fade.invert();
            } else {
                commands.entity(cover_entity).insert(Fade::new(
                    FadeType::FadeIn,
                    Duration::from_secs(1),
                    ease_in_sine,
                ));
            }
        }
    }
}

/// FIXME: We can close the banner from the temple into run in the secret and we will in the fog of war
pub fn add_secret_room_cover(
    mut add_secret_room_cover_event: EventReader<AddSecretRoomCoverEvent>,
    mut commands: Commands,
    mut secret_room_cover_query: Query<(Entity, Option<&mut Fade>), With<SecretRoomCover>>,
) {
    for AddSecretRoomCoverEvent in add_secret_room_cover_event.iter() {
        if let Ok((cover_entity, fade_opt)) = secret_room_cover_query.get_single_mut() {
            if let Some(mut fade) = fade_opt {
                fade.invert();
            } else {
                commands.entity(cover_entity).insert(Fade::new(
                    FadeType::FadeOut,
                    Duration::from_secs(1),
                    ease_out_sine,
                ));
            }
        }
    }
}

/// OPTIMIZE: OnEnter of secretRoom => visibility on, OnExit => off
pub fn second_layer_fake_wall_visibility(
    location: Res<State<Location>>,
    mut second_layer_fake_wall_query: Query<&mut Visibility, With<SecondLayerFakeWall>>,
) {
    if location.is_changed() {
        let mut visibility = second_layer_fake_wall_query.single_mut();
        *visibility = match location.get() {
            Location::SecretRoom => Visibility::Inherited,
            _ => Visibility::Hidden,
        };
    }
}

/* -------------------------------------------------------------------------- */
/*                                    Setup                                   */
/* -------------------------------------------------------------------------- */

pub fn setup_secret_room(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    /* -------------------------------------------------------------------------- */
    /*                                Assets Loader                               */
    /* -------------------------------------------------------------------------- */

    let secret_room = asset_server.load("textures/v4.0.0/Secret_Room/Secret_Room.png");

    let fake_stone = asset_server.load("textures/v4.0.0/Secret_Room/fake_stones_cuted.png");

    let first_flower_panel_spritesheet =
        asset_server.load("textures/v4.0.0/Secret_Room/1e_frame.png");
    let first_flower_panel_texture_atlas = TextureAtlas::from_grid(
        first_flower_panel_spritesheet,
        Vec2::new(24., 39.),
        16,
        1,
        None,
        None,
    );
    let second_flower_panel_spritesheet =
        asset_server.load("textures/v4.0.0/Secret_Room/2e_frame.png");
    let second_flower_panel_texture_atlas = TextureAtlas::from_grid(
        second_flower_panel_spritesheet,
        Vec2::new(24., 39.),
        16,
        1,
        None,
        None,
    );
    let third_flower_panel_spritesheet =
        asset_server.load("textures/v4.0.0/Secret_Room/3e_frame.png");
    let third_flower_panel_texture_atlas = TextureAtlas::from_grid(
        third_flower_panel_spritesheet,
        Vec2::new(24., 39.),
        16,
        1,
        None,
        None,
    );
    let fourth_flower_panel_spritesheet =
        asset_server.load("textures/v4.0.0/Secret_Room/4e_frame.png");
    let fourth_flower_panel_texture_atlas = TextureAtlas::from_grid(
        fourth_flower_panel_spritesheet,
        Vec2::new(24., 39.),
        16,
        1,
        None,
        None,
    );
    let repair_flower_panel_spritesheet =
        asset_server.load("textures/v4.0.0/Secret_Room/Repair_Frame.png");
    let repair_flower_panel_texture_atlas = TextureAtlas::from_grid(
        repair_flower_panel_spritesheet,
        Vec2::new(24., 39.),
        16,
        1,
        None,
        None,
    );

    let flower_panel_texture_atlas = vec![
        first_flower_panel_texture_atlas,
        second_flower_panel_texture_atlas,
        third_flower_panel_texture_atlas,
        fourth_flower_panel_texture_atlas,
        repair_flower_panel_texture_atlas,
    ];
    let flower_panel_collider_left =
        asset_server.load("textures/v4.0.0/Secret_Room/flower_panel_collider_left.png");
    let flower_panel_collider_right =
        asset_server.load("textures/v4.0.0/Secret_Room/flower_panel_collider_right.png");

    let wall_pot_spritesheet = asset_server.load("textures/v4.0.0/Secret_Room/wall_pot.png");
    let wall_pot_texture_atlas =
        TextureAtlas::from_grid(wall_pot_spritesheet, Vec2::new(21., 11.), 16, 1, None, None);

    let second_layer_fake_wall =
        asset_server.load("textures/v4.0.0/Secret_Room/2nd_layer_fake_wall.png");

    let stairs_down_ramp = asset_server.load("textures/v4.0.0/Secret_Room/stairs_down_ramp.png");

    /* -------------------------------------------------------------------------- */
    /*                               Wall Colliders                               */
    /* -------------------------------------------------------------------------- */

    let collider_bot_right =
        asset_server.load("textures/v4.0.0/Secret_Room/Wall_Colliders/bot_right.png");
    let collider_center_bot_left =
        asset_server.load("textures/v4.0.0/Secret_Room/Wall_Colliders/center_bot_left.png");
    let collider_center_bot_right =
        asset_server.load("textures/v4.0.0/Secret_Room/Wall_Colliders/center_bot_right.png");
    let collider_center_door_bot =
        asset_server.load("textures/v4.0.0/Secret_Room/Wall_Colliders/center_door_bot.png");
    let collider_center_door_top =
        asset_server.load("textures/v4.0.0/Secret_Room/Wall_Colliders/center_door_top.png");
    let collider_center_left =
        asset_server.load("textures/v4.0.0/Secret_Room/Wall_Colliders/center_left.png");
    let collider_left_bot =
        asset_server.load("textures/v4.0.0/Secret_Room/Wall_Colliders/left_bot.png");
    let collider_left = asset_server.load("textures/v4.0.0/Secret_Room/Wall_Colliders/left.png");
    let collider_right_bot_right =
        asset_server.load("textures/v4.0.0/Secret_Room/Wall_Colliders/right_bot_right.png");
    let collider_right_bot_top =
        asset_server.load("textures/v4.0.0/Secret_Room/Wall_Colliders/right_bot_top.png");
    let collider_right_top_left =
        asset_server.load("textures/v4.0.0/Secret_Room/Wall_Colliders/right_top_left.png");
    let collider_right_top_right =
        asset_server.load("textures/v4.0.0/Secret_Room/Wall_Colliders/right_top_right.png");
    let collider_right = asset_server.load("textures/v4.0.0/Secret_Room/Wall_Colliders/right.png");
    // maybe too complex
    let collider_stairs_bot =
        asset_server.load("textures/v4.0.0/Secret_Room/Wall_Colliders/stairs_bot.png");
    let collider_stairs_top =
        asset_server.load("textures/v4.0.0/Secret_Room/Wall_Colliders/stairs_top.png");
    let collider_top_center =
        asset_server.load("textures/v4.0.0/Secret_Room/Wall_Colliders/top_center.png");
    let collider_top_left =
        asset_server.load("textures/v4.0.0/Secret_Room/Wall_Colliders/top_left.png");

    let wall_colliders: Vec<Handle<Image>> = vec![
        collider_bot_right,
        collider_center_bot_left,
        collider_center_bot_right,
        collider_center_door_bot,
        collider_center_door_top,
        collider_center_left,
        collider_left_bot,
        collider_left,
        collider_right_bot_right,
        collider_right_bot_top,
        collider_right_top_left,
        collider_right_top_right,
        collider_right,
        collider_stairs_bot,
        collider_stairs_top,
        collider_top_center,
        collider_top_left,
    ];

    /* -------------------------------------------------------------------------- */
    /*                               Spawn Commands                               */
    /* -------------------------------------------------------------------------- */

    commands
        .spawn((
            SpriteBundle {
                texture: secret_room,
                transform: Transform::from_xyz(0., 0., SECRET_ROOM_Z),
                ..default()
            },
            SecretRoom,
            RigidBody::Fixed,
            Name::new("Secret Room"),
        ))
        .with_children(|parent| {
            // TEMP: Indicators
            let indicators = asset_server.load("textures/v4.0.0/Secret_Room/deco_indicators.png");
            parent.spawn((
                SpriteBundle {
                    texture: indicators,
                    transform: Transform::from_xyz(0., 0., 0.1),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                Name::new("Indicators"),
            ));

            // --- Secret Room Sensor ---
            parent.spawn((
                Collider::cuboid(6., 3.),
                Transform::from_translation(SECRET_LOCATION_SENSOR_POSITION.into()),
                ActiveEvents::COLLISION_EVENTS,
                Sensor,
                LocationSensor {
                    location: Location::SecretRoom,
                },
                Name::new("Secret Sensor from Temple"),
            ));

            parent
                .spawn((
                    TransformBundle::default(),
                    RigidBody::Fixed,
                    Name::new("Wall Colliders"),
                ))
                .with_children(|parent| {
                    for collider in &wall_colliders {
                        parent.spawn((
                            TesselatedCollider {
                                texture: collider.clone(),
                                tesselator_config: TesselatedColliderConfig {
                                    vertice_separation: 0.,
                                    ..default()
                                },
                            },
                            Transform::default(),
                            WallCollider(Location::SecretRoom),
                        ));
                    }
                });

            // parent.spawn((
            //     Collider::cuboid(SECRET_ROOM_TRIGGER_CUBOID.0, SECRET_ROOM_TRIGGER_CUBOID.1),
            //     Transform::from_translation(SECRET_ROOM_TRIGGER_POSITION.into()),
            //     ActiveEvents::COLLISION_EVENTS,
            //     SecretRoomSensor,
            //     Sensor,
            //     Name::new("Secret Room Trigger Sensor"),
            // ));

            // BUG: Flower Panel and pot Glitch, like if one of their frame was too short - or lightly out of alignment
            // (when the camera move horizontaly for the panel)
            for count in 0..5 {
                parent
                    .spawn((
                        SpriteSheetBundle {
                            texture_atlas: texture_atlases
                                .add(flower_panel_texture_atlas[count].clone()),
                            transform: Transform::from_translation(
                                FLOWER_PANEL_POSITIONS[count].into(),
                            ),
                            ..default()
                        },
                        SpriteSheetAnimation {
                            start_index: 0,
                            end_index: flower_panel_texture_atlas[count].clone().len() - 1,
                            duration: AnimationDuration::Infinite,
                            timer: Timer::new(Duration::from_millis(100), TimerMode::Repeating),
                        },
                        OverlappingEntity::new(FLOWER_PANEL_SWITCH_Z_OFFSET),
                        FlowerPanel,
                        Name::new(format!("Flower Panel°{}", count + 1)),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            TesselatedCollider {
                                // The first, Second and Repair (4th) are left sided, the others right sided
                                texture: if count == 4 || count < 2 {
                                    flower_panel_collider_left.clone()
                                } else {
                                    flower_panel_collider_right.clone()
                                },
                                tesselator_config: TesselatedColliderConfig {
                                    vertice_separation: 0.,
                                    ..default()
                                },
                            },
                            Transform::default(),
                        ));
                    });
            }

            parent.spawn((
                SpriteBundle {
                    texture: fake_stone,
                    transform: Transform::from_translation(FAKE_STONE_POSITION.into()),
                    ..default()
                },
                OverlappingEntity::new(FAKE_STONE_SWITCH_Z_OFFSET),
                Name::new("Fake Stone"),
            ));

            parent.spawn((
                SpriteSheetBundle {
                    texture_atlas: texture_atlases.add(wall_pot_texture_atlas.clone()),
                    transform: Transform::from_translation(WALL_POT_POSITION.into()),
                    ..default()
                },
                SpriteSheetAnimation {
                    start_index: 0,
                    end_index: wall_pot_texture_atlas.len() - 1,
                    duration: AnimationDuration::Infinite,
                    timer: Timer::new(Duration::from_millis(100), TimerMode::Repeating),
                },
                FlowerPot,
                Name::new("Flower Wall Pot"),
            ));

            parent.spawn((
                SpriteBundle {
                    texture: stairs_down_ramp,
                    transform: Transform::from_translation(STAIRS_RAMP_POSITION.into()),
                    ..default()
                },
                OverlappingEntity::new(STAIRS_RAMP_SWITCH_Z_OFFSET),
                Name::new("Stairs Down Ramp"),
            ));

            // Cause the y switch of the temple is too high
            // (up to the stairs)
            // Being in the secret Room behind the Temple Wall
            // Make your character above (cause under the y switch of the Temple)
            // Also, to avoid having this second layer above us caus we are in the stairs,
            // we only display it when we are in the SecretRoom.
            parent.spawn((
                SpriteBundle {
                    texture: second_layer_fake_wall,
                    ..default()
                },
                OverlappingEntity::new(SECOND_FAKE_WALL_SWITCH_Z_OFFSET),
                SecondLayerFakeWall,
                Name::new("2nd layer of the Fake Wall"),
            ));
        });

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(SECRET_ROOM_COVER_POSITION.into()),
            sprite: Sprite {
                custom_size: Some(SECRET_ROOM_COVER_SIZE.into()),
                color: BACKGROUND_COLOR_INGAME, // Color::WHITE,
                ..default()
            },
            ..default()
        },
        SecretRoomCover,
        Name::new("Secret Room Cover"),
    ));
}

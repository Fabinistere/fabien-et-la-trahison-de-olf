use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

use crate::{
    animations::sprite_sheet_animation::{AnimationDuration, SpriteSheetAnimation},
    collisions::{TesselatedCollider, TesselatedColliderConfig},
    constants::locations::{main_room::*, CHANDELIER_FLAME_POSITIONS},
    interactions::{Interactible, InteractionSensor},
    locations::temple::{
        secret_room::{AddSecretRoomCoverEvent, RemoveSecretRoomCoverEvent},
        Chandelier, DoorColliderClosed, DoorState, Flame, LocationSensor, OverlappingEntity,
        PlayerLocation, WallCollider,
    },
};

/* -------------------------------------------------------------------------- */
/*                                 Components                                 */
/* -------------------------------------------------------------------------- */

#[derive(Component)]
pub struct Temple;

#[derive(Component)]
pub struct Pillar;

#[derive(Component)]
pub struct Throne;

#[derive(Component)]
pub struct SecretBanner;

/* -------------------------------------------------------------------------- */
/*                                   Events                                   */
/* -------------------------------------------------------------------------- */

#[derive(Event)]
pub struct SecretBannerEvent(pub DoorState);

/* -------------------------------------------------------------------------- */
/*                                   Systems                                  */
/* -------------------------------------------------------------------------- */

/// REFACTOR: SecretRoomCover
pub fn secret_banner_interaction(
    mut secret_banner_event: EventReader<SecretBannerEvent>,
    player_location: Res<State<PlayerLocation>>,

    mut remove_secret_room_cover_event: EventWriter<RemoveSecretRoomCoverEvent>,
    mut add_secret_room_cover_event: EventWriter<AddSecretRoomCoverEvent>,
) {
    for SecretBannerEvent(door_state) in secret_banner_event.iter() {
        if player_location.get() == &PlayerLocation::Temple {
            match *door_state {
                DoorState::Closed => {
                    remove_secret_room_cover_event.send(RemoveSecretRoomCoverEvent)
                }
                DoorState::Opened => add_secret_room_cover_event.send(AddSecretRoomCoverEvent),
                _ => {}
            }
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                                    Setup                                   */
/* -------------------------------------------------------------------------- */

// Spawns all entity related to the main room
pub fn setup_main_room(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    /* -------------------------------------------------------------------------- */
    /*                                Assets Loader                               */
    /* -------------------------------------------------------------------------- */

    let main_room = asset_server.load("textures/v4.0.0/Temple/Temple.png");

    let throne = asset_server.load("textures/v4.0.0/Temple/Throne.png");
    let throne_hitbox_left_triangle =
        asset_server.load("textures/v4.0.0/Temple/Throne_hitbox_left_triangle.png");
    let throne_hitbox_left_bar =
        asset_server.load("textures/v4.0.0/Temple/Throne_hitbox_left_bar.png");
    let throne_hitbox_center = asset_server.load("textures/v4.0.0/Temple/Throne_hitbox_center.png");
    let throne_hitbox_right_triangle =
        asset_server.load("textures/v4.0.0/Temple/Throne_hitbox_right_triangle.png");
    let throne_hitbox_right_bar =
        asset_server.load("textures/v4.0.0/Temple/Throne_hitbox_right_bar.png");

    let column = asset_server.load("textures/v4.0.0/Temple/column.png");
    let column_hitbox = asset_server.load("textures/v4.0.0/Temple/column_hitbox.png");

    let chandelier = asset_server.load("textures/v4.0.0/chandelier.png");
    let small_flame_spritesheet = asset_server.load("textures/v4.0.0/burning_loop_5.png");
    let small_flame_texture_atlas =
        TextureAtlas::from_grid(small_flame_spritesheet, Vec2::new(8., 8.), 4, 1, None, None);

    let plants = vec![
        asset_server.load("textures/v4.0.0/Temple/TL_plants.png"),
        asset_server.load("textures/v4.0.0/Temple/BL_plants.png"),
        asset_server.load("textures/v4.0.0/Temple/TR_plants.png"),
        asset_server.load("textures/v4.0.0/Temple/BR_plants.png"),
    ];
    let plants_collider = asset_server.load("textures/v4.0.0/Temple/plants_collider.png");

    let brazier_back = asset_server.load("textures/v4.0.0/Temple/brazier_back.png");
    let brazier_front = asset_server.load("textures/v4.0.0/Temple/brazier_front.png");
    let brazier_collider = asset_server.load("textures/v4.0.0/Temple/brazier_collider.png");
    let medium_flame_spritesheet =
        asset_server.load("textures/v4.0.0/Temple/fire_orange_medium.png");
    let medium_flame_texture_atlas = TextureAtlas::from_grid(
        medium_flame_spritesheet,
        Vec2::new(11., 18.),
        8,
        1,
        None,
        None,
    );

    let banner_spritesheet =
        asset_server.load("textures/v4.0.0/Temple/left_banner_spritesheet.png");
    let banner_texture_atlas =
        TextureAtlas::from_grid(banner_spritesheet, Vec2::new(21., 34.), 33, 1, None, None);
    let banner_collider = asset_server.load("textures/v4.0.0/Temple/banner_collider.png");

    let cat_statue = asset_server.load("textures/v4.0.0/Temple/cat_statue.png");
    let fabien_statue = asset_server.load("textures/v4.0.0/Temple/fabien_statue.png");
    let statue_collider = asset_server.load("textures/v4.0.0/Temple/statue_collider.png");

    /* -------------------------------------------------------------------------- */
    /*                               Wall Colliders                               */
    /* -------------------------------------------------------------------------- */

    // maybe too complex
    let collider_stairs_left =
        asset_server.load("textures/v4.0.0/Temple/Wall_Colliders/stairs_left.png");
    let collider_stairs_right =
        asset_server.load("textures/v4.0.0/Temple/Wall_Colliders/stairs_right.png");
    let collider_bot_left = asset_server.load("textures/v4.0.0/Temple/Wall_Colliders/bot_left.png");
    let collider_bot_right =
        asset_server.load("textures/v4.0.0/Temple/Wall_Colliders/bot_right.png");
    let collider_left = asset_server.load("textures/v4.0.0/Temple/Wall_Colliders/left.png");
    let collider_right = asset_server.load("textures/v4.0.0/Temple/Wall_Colliders/right.png");
    let collider_top_left = asset_server.load("textures/v4.0.0/Temple/Wall_Colliders/top_left.png");
    let collider_top_right =
        asset_server.load("textures/v4.0.0/Temple/Wall_Colliders/top_right.png");

    let wall_colliders: Vec<Handle<Image>> = vec![
        collider_stairs_left,
        collider_stairs_right,
        collider_bot_left,
        collider_bot_right,
        collider_left,
        collider_right,
        collider_top_left,
        collider_top_right,
    ];

    /* -------------------------------------------------------------------------- */
    /*                               Spawn Commands                               */
    /* -------------------------------------------------------------------------- */

    commands
        .spawn((
            SpriteBundle {
                texture: main_room,
                transform: Transform::from_xyz(0., 0., MAIN_ROOM_Z),
                ..default()
            },
            Temple,
            RigidBody::Fixed,
            Name::new("Temple"),
        ))
        .with_children(|parent| {
            // --- Temple Sensors ---
            parent.spawn((
                Collider::cuboid(10., 3.),
                Transform::from_translation(TEMPLE_HALL_LOCATION_SENSOR_POSITION.into()),
                ActiveEvents::COLLISION_EVENTS,
                Sensor,
                LocationSensor {
                    location: PlayerLocation::Temple,
                },
                Name::new("Temple Sensor from Hall"),
            ));

            parent.spawn((
                Collider::cuboid(6., 3.),
                Transform::from_translation(TEMPLE_SECRET_LOCATION_SENSOR_POSITION.into()),
                ActiveEvents::COLLISION_EVENTS,
                Sensor,
                LocationSensor {
                    location: PlayerLocation::Temple,
                },
                Name::new("Temple Sensor from Secret Room"),
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
                            WallCollider(PlayerLocation::Temple),
                        ));
                    }
                });

            parent
                .spawn((
                    SpriteSheetBundle {
                        texture_atlas: texture_atlases.add(banner_texture_atlas),
                        transform: Transform::from_translation(BANNER_POSITION.into()),
                        ..default()
                    },
                    SecretBanner,
                    DoorState::Closed,
                    Interactible::new(
                        BANNER_INTERACT_BUTTON_POSITION.into(),
                        BANNER_INTERACTION_ID,
                    ),
                    RigidBody::Fixed,
                    Name::new("Secret Banner"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Collider::ball(15.),
                        Transform::from_translation(BANNER_SENSOR_OFFSET.into()),
                        Sensor,
                        InteractionSensor,
                    ));

                    parent.spawn((
                        TesselatedCollider {
                            texture: banner_collider,
                            tesselator_config: TesselatedColliderConfig {
                                vertice_separation: 0.,
                                ..default()
                            },
                        },
                        Transform::from_translation(BANNER_COLLIDER_OFFSET.into()),
                        DoorColliderClosed,
                    ));
                });

            /*
            parent
                .spawn((
                    Collider::segment(
                        Vect::new(-320., MAIN_ROOM_ENTER_Y),
                        Vect::new(-140., MAIN_ROOM_ENTER_Y),
                    ),
                    Transform::default(),
                    ActiveEvents::COLLISION_EVENTS,
                    EnterMainRoomSensor,
                    Sensor(true),
                ));
            */
            parent
                .spawn((
                    SpriteBundle {
                        texture: throne,
                        transform: Transform::from_translation(THRONE_POSITION.into()),
                        ..default()
                    },
                    RigidBody::Fixed,
                    Throne,
                    OverlappingEntity::new(THRONE_SWITCH_Z_OFFSET),
                    Name::new("Throne"),
                ))
                .with_children(|parent| {
                    // use `bevy_rapier_collider_gen` instead
                    parent.spawn((
                        TesselatedCollider {
                            texture: throne_hitbox_left_triangle.clone(),
                            tesselator_config: TesselatedColliderConfig {
                                vertice_separation: 0.,
                                extrusion: 0.1,
                                vertice_radius: 0.4,
                            },
                        },
                        Transform::IDENTITY,
                        Name::new("Throne Hitbox Left Triangle"),
                    ));
                    parent.spawn((
                        TesselatedCollider {
                            texture: throne_hitbox_left_bar.clone(),
                            tesselator_config: TesselatedColliderConfig {
                                vertice_separation: 0.,
                                extrusion: 0.1,
                                vertice_radius: 0.4,
                            },
                        },
                        Transform::IDENTITY,
                        Name::new("Throne Hitbox Left Bar"),
                    ));
                    parent.spawn((
                        TesselatedCollider {
                            texture: throne_hitbox_center,
                            tesselator_config: TesselatedColliderConfig {
                                vertice_separation: 0.,
                                extrusion: 0.1,
                                vertice_radius: 0.4,
                            },
                        },
                        Transform::IDENTITY,
                        Name::new("Throne Hitbox Center"),
                    ));
                    parent.spawn((
                        TesselatedCollider {
                            texture: throne_hitbox_right_triangle,
                            tesselator_config: TesselatedColliderConfig {
                                vertice_separation: 0.,
                                extrusion: 0.1,
                                vertice_radius: 0.4,
                            },
                        },
                        Transform::IDENTITY,
                        Name::new("Throne Hitbox Right Triangle"),
                    ));
                    parent.spawn((
                        TesselatedCollider {
                            texture: throne_hitbox_right_bar,
                            tesselator_config: TesselatedColliderConfig {
                                vertice_separation: 0.,
                                extrusion: 0.1,
                                vertice_radius: 0.4,
                            },
                        },
                        Transform::IDENTITY,
                        Name::new("Throne Hitbox Right Bar"),
                    ));
                });

            // TODO: CHECK https://bevy-cheatbook.github.io/features/parent-child.html
            // 6 PILLARS
            for (count, pillar_position) in PILLAR_POSITIONS.iter().enumerate() {
                parent
                    .spawn((
                        SpriteBundle {
                            texture: column.clone(),
                            transform: Transform {
                                translation: (*pillar_position).into(),
                                ..default()
                            },
                            ..default()
                        },
                        RigidBody::Fixed,
                        Pillar,
                        OverlappingEntity::new(PILLAR_SWITCH_Z_OFFSET),
                        Name::new(format!("Column {}", count + 1)),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            TesselatedCollider {
                                texture: column_hitbox.clone(),
                                tesselator_config: TesselatedColliderConfig {
                                    vertice_separation: 0.,
                                    ..default()
                                },
                            },
                            Transform::from_xyz(0., PILLAR_HITBOX_Y_OFFSET, 0.),
                            Name::new(format!("Column {} Hitbox", count + 1)),
                        ));
                    });
            }

            // 4 Chandeliers
            for (count, temple_chandelier_position) in
                TEMPLE_CHANDELIER_POSITIONS.iter().enumerate()
            {
                parent
                    .spawn((
                        SpriteBundle {
                            texture: chandelier.clone(),
                            transform: Transform::from_translation(
                                (*temple_chandelier_position).into(),
                            ),
                            ..default()
                        },
                        Chandelier,
                        Name::new(format!("Chandelier {}", count + 1)),
                    ))
                    .with_children(|parent| {
                        // spawn 3 flame
                        for (flame_number, chandelier_flame_position) in
                            CHANDELIER_FLAME_POSITIONS.iter().enumerate()
                        {
                            parent.spawn((
                                SpriteSheetBundle {
                                    texture_atlas: texture_atlases
                                        .add(small_flame_texture_atlas.clone()),
                                    transform: Transform::from_translation(
                                        (*chandelier_flame_position).into(),
                                    ),
                                    ..default()
                                },
                                SpriteSheetAnimation {
                                    start_index: 0,
                                    end_index: small_flame_texture_atlas.clone().len() - 1,
                                    duration: AnimationDuration::Infinite,
                                    timer: Timer::new(
                                        Duration::from_millis(100),
                                        TimerMode::Repeating,
                                    ),
                                },
                                Flame,
                                Name::new(format!("Flame°{}", flame_number + 1)),
                            ));
                        }
                    });
            }

            // 4 plants pots
            for count in 0..4 {
                parent
                    .spawn((
                        SpriteBundle {
                            texture: plants[count].clone(),
                            transform: Transform::from_translation(PLANTS_POSITIONS[count].into()),
                            ..default()
                        },
                        RigidBody::Fixed,
                        // Plant,
                        OverlappingEntity::new(PLANTS_SWITCH_Z_OFFSET),
                        Name::new(format!("Plants {}", count + 1)),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            TesselatedCollider {
                                texture: plants_collider.clone(),
                                tesselator_config: TesselatedColliderConfig {
                                    vertice_separation: 0.,
                                    ..default()
                                },
                            },
                            Transform::IDENTITY,
                            Name::new(format!("Pot {} Hitbox", count + 1)),
                        ));
                    });
            }

            // 4 Braziers
            for (count, brazier_position) in BRAZIERS_POSITIONS.iter().enumerate() {
                parent
                    .spawn((
                        SpriteBundle {
                            texture: brazier_back.clone(),
                            transform: Transform::from_translation((*brazier_position).into()),
                            ..default()
                        },
                        RigidBody::Fixed,
                        // Brazier,
                        OverlappingEntity::new(BRAZIER_Z_OFFSET),
                        Name::new(format!("Braizier {}", count + 1)),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            SpriteSheetBundle {
                                texture_atlas: texture_atlases
                                    .add(medium_flame_texture_atlas.clone()),
                                transform: Transform::from_translation(BRAZIER_FLAME_OFFSET.into()),
                                ..default()
                            },
                            SpriteSheetAnimation {
                                start_index: 0,
                                end_index: medium_flame_texture_atlas.clone().len() - 1,
                                duration: AnimationDuration::Infinite,
                                timer: Timer::new(Duration::from_millis(100), TimerMode::Repeating),
                            },
                            Name::new(format!("Medium Flame°{}", count + 1)),
                        ));

                        parent.spawn((
                            SpriteBundle {
                                texture: brazier_front.clone(),
                                ..default()
                            },
                            Name::new("Braizier Front"),
                        ));

                        parent.spawn((
                            TesselatedCollider {
                                texture: brazier_collider.clone(),
                                tesselator_config: TesselatedColliderConfig {
                                    vertice_separation: 0.,
                                    ..default()
                                },
                            },
                            Transform::IDENTITY,
                            Name::new(format!("Brazier {} Hitbox", count + 1)),
                        ));
                    });
            }

            // The 2 Statues
            parent
                .spawn((
                    SpriteBundle {
                        texture: cat_statue.clone(),
                        transform: Transform::from_translation(CAT_STATUE_POSITION.into()),
                        ..default()
                    },
                    RigidBody::Fixed,
                    // Statue,
                    OverlappingEntity::default(),
                    Name::new("Cat Statue"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TesselatedCollider {
                            texture: statue_collider.clone(),
                            tesselator_config: TesselatedColliderConfig {
                                vertice_separation: 0.,
                                ..default()
                            },
                        },
                        Transform::IDENTITY,
                        Name::new("Cat Statue Collider"),
                    ));
                });

            parent
                .spawn((
                    SpriteBundle {
                        texture: fabien_statue.clone(),
                        transform: Transform::from_translation(FABIEN_STATUE_POSITION.into()),
                        ..default()
                    },
                    RigidBody::Fixed,
                    // Statue,
                    OverlappingEntity::default(),
                    Name::new("Fabien Statue"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TesselatedCollider {
                            texture: statue_collider.clone(),
                            tesselator_config: TesselatedColliderConfig {
                                vertice_separation: 0.,
                                ..default()
                            },
                        },
                        Transform::IDENTITY,
                        Name::new("Fabien Statue Collider"),
                    ));
                });
        });
}

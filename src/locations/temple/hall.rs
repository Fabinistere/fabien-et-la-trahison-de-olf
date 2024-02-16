use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

use crate::{
    animations::{
        functions::ease_in_sine,
        sprite_sheet_animation::{AnimationDuration, SpriteSheetAnimation, SpriteSheetIndex},
        Fade, FadeType,
    },
    characters::player::PlayerHitbox,
    collisions::{TesselatedCollider, TesselatedColliderConfig},
    constants::{
        interactions::INTERACT_BUTTON_SCALE,
        locations::{hall::*, CHANDELIER_FLAME_POSITIONS},
        BACKGROUND_COLOR_INGAME,
    },
    interactions::{InteractIcon, Interactible, InteractionResources, InteractionSensor},
    locations::temple::{
        Chandelier, DoorColliderClosed, DoorState, Flame, Location, LocationSensor,
        OverlappingEntity, WallCollider,
    },
};

use super::DoorColliderOpened;

/* -------------------------------------------------------------------------- */
/*                                 Components                                 */
/* -------------------------------------------------------------------------- */

#[derive(Component)]
pub struct TempleDoor;

#[derive(Component)]
pub struct Hall;

#[derive(Deref, DerefMut, Component)]
pub struct BalconyCover {
    on: bool,
}

#[derive(Component)]
pub struct BalconyUpDoor;

#[derive(Component)]
pub struct BalconySensor;

/* -------------------------------------------------------------------------- */
/*                                   Events                                   */
/* -------------------------------------------------------------------------- */

#[derive(Event)]
pub struct PropsInteractionEvent;

/* -------------------------------------------------------------------------- */
/*                                   Systems                                  */
/* -------------------------------------------------------------------------- */

/// TODO: Lunch a dialog or smth
pub fn props_interaction_event(mut props_interaction_events: EventReader<PropsInteractionEvent>) {
    for PropsInteractionEvent in props_interaction_events.iter() {
        info!("interact with props");
    }
}

pub fn remove_balcony_cover(
    mut collision_events: EventReader<CollisionEvent>,

    balcony_sensor_query: Query<Entity, With<BalconySensor>>,
    player_hitbox_query: Query<Entity, With<PlayerHitbox>>,

    mut commands: Commands,
    mut balcony_cover_query: Query<(Entity, Option<&mut Fade>, &mut BalconyCover)>,
    mut balcony_door_query: Query<&mut Visibility, With<BalconyUpDoor>>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                match (
                    player_hitbox_query.get(*e1),
                    player_hitbox_query.get(*e2),
                    balcony_sensor_query.get(*e1),
                    balcony_sensor_query.get(*e2),
                ) {
                    (Ok(player_hitbox), Err(_), Err(_), Ok(balcony_sensor))
                    | (Err(_), Ok(player_hitbox), Ok(balcony_sensor), Err(_)) => {
                        if (*e1 == balcony_sensor && *e2 == player_hitbox)
                            || (*e1 == player_hitbox && *e2 == balcony_sensor)
                        {
                            if let Ok((cover_entity, fade_opt, mut cover)) =
                                balcony_cover_query.get_single_mut()
                            {
                                if cover.on && fade_opt.is_none() {
                                    commands.entity(cover_entity).insert(Fade::new(
                                        FadeType::FadeIn,
                                        Duration::from_secs(1),
                                        ease_in_sine,
                                    ));

                                    cover.on = false;

                                    // REFACTOR: put the balcony_up_door up
                                    let mut door_up_visibility = balcony_door_query.single_mut();
                                    *door_up_visibility = Visibility::Inherited;
                                }
                            }
                        }
                    }
                    _ => continue,
                }
            }
            _ => continue,
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                                    Setup                                   */
/* -------------------------------------------------------------------------- */

/// FIXME: When in Temple you could slip a lttle bit under the hall asset
/// 2 solutions:
/// - Move sensors (carefull with interaction)
/// - cut the Hall floor and put it in the Temple Floor which will always be under
pub fn setup_hall(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    interaction_resources: Res<InteractionResources>,
) {
    /* -------------------------------------------------------------------------- */
    /*                                Assets Loader                               */
    /* -------------------------------------------------------------------------- */

    let hall = asset_server.load("textures/v4.0.0/Hall/Hall.png");
    let balcony = asset_server.load("textures/v4.0.0/Hall/Balcony.png");
    let hall_up_door = asset_server.load("textures/v4.0.0/Hall/up_doors.png");
    let balcony_up_door = asset_server.load("textures/v4.0.0/Hall/balcony_up_door.png");

    let props = asset_server.load("textures/v4.0.0/Hall/box_obstacles.png");
    let props_hitbox = asset_server.load("textures/v4.0.0/Hall/box_obstacles_collider.png");

    let statue = asset_server.load("textures/v4.0.0/Hall/TearNBlood_statue.png");
    let statue_hitbox = asset_server.load("textures/v4.0.0/Hall/TearNBlood_statue_collider.png");

    let chandelier = asset_server.load("textures/v4.0.0/chandelier.png");
    let small_flame_spritesheet = asset_server.load("textures/v4.0.0/burning_loop_5.png");
    let small_flame_texture_atlas =
        TextureAtlas::from_grid(small_flame_spritesheet, Vec2::new(8., 8.), 4, 1, None, None);
    let wall_light_support = asset_server.load("textures/v4.0.0/Hall/Light_support.png");

    let door_spritesheet = asset_server.load("textures/v4.0.0/Hall/door_spritesheet.png");
    let door_texture_atlas =
        TextureAtlas::from_grid(door_spritesheet, Vec2::new(20., 38.), 1, 8, None, None);
    let door_collider = asset_server.load("textures/v4.0.0/Hall/door_collider.png");
    let door_collider_opened_left =
        asset_server.load("textures/v4.0.0/Hall/door_collider_opened_left.png");
    let door_collider_opened_right =
        asset_server.load("textures/v4.0.0/Hall/door_collider_opened_right.png");

    /* -------------------------------------------------------------------------- */
    /*                               Wall Colliders                               */
    /* -------------------------------------------------------------------------- */

    // maybe too complex
    let collider_balcony_bot_right =
        asset_server.load("textures/v4.0.0/Hall/Wall_Colliders/Balcony/bot_right.png");
    let collider_balcony_entry_bot =
        asset_server.load("textures/v4.0.0/Hall/Wall_Colliders/Balcony/entry_bot.png");
    let collider_balcony_entry_top =
        asset_server.load("textures/v4.0.0/Hall/Wall_Colliders/Balcony/entry_top.png");
    let collider_balcony_left =
        asset_server.load("textures/v4.0.0/Hall/Wall_Colliders/Balcony/left.png");
    let collider_balcony_top =
        asset_server.load("textures/v4.0.0/Hall/Wall_Colliders/Balcony/top.png");
    let collider_hall_bottom = asset_server.load("textures/v4.0.0/Hall/Wall_Colliders/bottom.png");
    let collider_hall_left = asset_server.load("textures/v4.0.0/Hall/Wall_Colliders/left.png");
    let collider_hall_right = asset_server.load("textures/v4.0.0/Hall/Wall_Colliders/right.png");
    let collider_hall_top_left =
        asset_server.load("textures/v4.0.0/Hall/Wall_Colliders/top_left.png");
    let collider_hall_top_right =
        asset_server.load("textures/v4.0.0/Hall/Wall_Colliders/top_right.png");

    let wall_colliders: Vec<Handle<Image>> = vec![
        collider_balcony_bot_right,
        collider_balcony_entry_bot,
        collider_balcony_entry_top,
        collider_balcony_left,
        collider_balcony_top,
        collider_hall_bottom,
        collider_hall_left,
        collider_hall_right,
        collider_hall_top_left,
        collider_hall_top_right,
    ];

    /* -------------------------------------------------------------------------- */
    /*                               Spawn Commands                               */
    /* -------------------------------------------------------------------------- */

    commands
        .spawn((
            SpriteBundle {
                texture: hall,
                transform: Transform::from_xyz(0., 0., HALL_Z),
                ..default()
            },
            Hall,
            RigidBody::Fixed,
            Name::new("Hall"),
        ))
        .with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    texture: hall_up_door,
                    transform: Transform::from_xyz(0., 0., UP_DOOR_Z),
                    ..default()
                },
                Name::new("Hall Up Doors"),
            ));

            parent
                .spawn((
                    SpriteBundle {
                        texture: balcony,
                        transform: Transform::from_translation(BALCONY_POSITION.into()),
                        ..default()
                    },
                    Name::new("Balcony"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        SpriteBundle {
                            transform: Transform::from_translation(BALCONY_COVER_POSITION.into()),
                            sprite: Sprite {
                                custom_size: Some(BALCONY_COVER_SIZE.into()),
                                color: BACKGROUND_COLOR_INGAME, // Color::WHITE, //
                                ..default()
                            },
                            ..default()
                        },
                        BalconyCover { on: true },
                        Name::new("Balcony Cover"),
                    ));

                    // TODO: Change Balcony Up door when Balcony Sensor
                    parent.spawn((
                        SpriteBundle {
                            texture: balcony_up_door,
                            transform: Transform::from_translation(UP_DOOR_POSITION.into()),
                            visibility: Visibility::Hidden,
                            ..default()
                        },
                        BalconyUpDoor,
                        Name::new("Balcony Up Door"),
                    ));
                });

            // --- Hall Sensor ---
            parent.spawn((
                // FIXME: "Magical" number
                Collider::cuboid(10., 3.),
                Transform::from_translation(HALL_FROM_TEMPLE_LOCATION_SENSOR_POSITION.into()),
                ActiveEvents::COLLISION_EVENTS,
                Sensor,
                LocationSensor {
                    location: Location::Hall,
                },
                Name::new("Hall Sensor from Temple"),
            ));

            parent.spawn((
                Collider::cuboid(
                    BALCONY_LOCATION_SENSOR_SIZE.0,
                    BALCONY_LOCATION_SENSOR_SIZE.1,
                ),
                Transform::from_translation(BALCONY_LOCATION_SENSOR_POSITION.into()),
                ActiveEvents::COLLISION_EVENTS,
                Sensor,
                BalconySensor,
                Name::new("Balcony Sensor"),
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
                            WallCollider(Location::Hall),
                        ));
                    }
                });

            parent
                .spawn((
                    SpriteBundle {
                        texture: props,
                        transform: Transform::from_translation(BOX_POSITION.into()),
                        ..default()
                    },
                    OverlappingEntity::default(),
                    Interactible::new(BOX_INTERACT_BUTTON_POSITION.into(), BOX_INTERACTION_ID),
                    RigidBody::Fixed,
                    Name::new("Box"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Collider::ball(20.),
                        Transform::from_translation(BOX_SENSOR_OFFSET.into()),
                        Sensor,
                        InteractionSensor,
                    ));

                    parent.spawn((
                        SpriteBundle {
                            texture: interaction_resources.interact_button.clone(),
                            transform: Transform {
                                translation: BOX_INTERACT_BUTTON_POSITION.into(),
                                scale: Vec3::splat(INTERACT_BUTTON_SCALE),
                                ..default()
                            },
                            visibility: Visibility::Hidden,
                            ..default()
                        },
                        InteractIcon,
                    ));

                    parent.spawn((
                        TesselatedCollider {
                            texture: props_hitbox,
                            tesselator_config: TesselatedColliderConfig {
                                vertice_separation: 0.,
                                ..default()
                            },
                        },
                        Transform::IDENTITY,
                    ));
                });

            parent
                .spawn((
                    SpriteSheetBundle {
                        texture_atlas: texture_atlases.add(door_texture_atlas),
                        transform: Transform::from_translation(DOOR_POSITION.into()),
                        ..default()
                    },
                    TempleDoor,
                    DoorState::Closed,
                    OverlappingEntity::new(TEMPLE_DOOR_SWITCH_Z_OFFSET_CLOSED),
                    Interactible::new(DOOR_INTERACT_BUTTON_POSITION.into(), DOOR_INTERACTION_ID),
                    RigidBody::Fixed,
                    Name::new("Temple Door"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Collider::ball(20.),
                        Transform::from_translation(DOOR_SENSOR_OFFSET.into()),
                        Sensor,
                        InteractionSensor,
                    ));

                    parent.spawn((
                        SpriteBundle {
                            texture: interaction_resources.interact_button.clone(),
                            transform: Transform {
                                translation: BOX_INTERACT_BUTTON_POSITION.into(),
                                scale: Vec3::splat(INTERACT_BUTTON_SCALE),
                                ..default()
                            },
                            visibility: Visibility::Hidden,
                            ..default()
                        },
                        InteractIcon,
                    ));

                    parent.spawn((
                        TesselatedCollider {
                            texture: door_collider,
                            tesselator_config: TesselatedColliderConfig {
                                vertice_separation: 0.,
                                ..default()
                            },
                        },
                        Transform::IDENTITY,
                        DoorColliderClosed,
                    ));
                    parent.spawn((
                        TesselatedCollider {
                            texture: door_collider_opened_left,
                            tesselator_config: TesselatedColliderConfig {
                                vertice_separation: 0.,
                                ..default()
                            },
                        },
                        Transform::IDENTITY,
                        DoorColliderOpened,
                        Sensor,
                    ));
                    parent.spawn((
                        TesselatedCollider {
                            texture: door_collider_opened_right,
                            tesselator_config: TesselatedColliderConfig {
                                vertice_separation: 0.,
                                ..default()
                            },
                        },
                        Transform::IDENTITY,
                        DoorColliderOpened,
                        Sensor,
                    ));
                });

            parent
                .spawn((
                    SpriteBundle {
                        texture: statue,
                        transform: Transform::from_translation(STATUE_POSITION.into()),
                        ..default()
                    },
                    OverlappingEntity::default(),
                    Interactible::new(
                        STATUE_INTERACT_BUTTON_POSITION.into(),
                        STATUE_INTERACTION_ID,
                    ),
                    RigidBody::Fixed,
                    Name::new("TearsNBlood Statue"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Collider::ball(15.),
                        Transform::IDENTITY,
                        Sensor,
                        InteractionSensor,
                    ));

                    parent.spawn((
                        SpriteBundle {
                            texture: interaction_resources.interact_button.clone(),
                            transform: Transform {
                                translation: BOX_INTERACT_BUTTON_POSITION.into(),
                                scale: Vec3::splat(INTERACT_BUTTON_SCALE),
                                ..default()
                            },
                            visibility: Visibility::Hidden,
                            ..default()
                        },
                        InteractIcon,
                    ));

                    parent.spawn((
                        TesselatedCollider {
                            texture: statue_hitbox,
                            tesselator_config: TesselatedColliderConfig {
                                vertice_separation: 0.,
                                ..default()
                            },
                        },
                        Transform::IDENTITY,
                    ));
                });

            for (count, wall_light_position) in WALL_LIGHT_POSITIONS.iter().enumerate() {
                parent
                    .spawn((
                        SpriteSheetBundle {
                            texture_atlas: texture_atlases.add(small_flame_texture_atlas.clone()),
                            transform: Transform::from_translation((*wall_light_position).into()),
                            ..default()
                        },
                        SpriteSheetAnimation {
                            index: SpriteSheetIndex::new(
                                0,
                                small_flame_texture_atlas.clone().len() - 1,
                            ),
                            duration: AnimationDuration::Infinite,
                            timer: Timer::new(Duration::from_millis(100), TimerMode::Repeating),
                        },
                        Flame,
                        Name::new(format!("Wall Light°{}", count + 1)),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            SpriteBundle {
                                texture: wall_light_support.clone(),
                                transform: Transform::from_translation(LIGHT_SUPPORT_OFFSET.into()),
                                ..default()
                            },
                            Name::new("Light support"),
                        ));
                    });
            }

            for (count, hall_chancelier_position) in HALL_CHANDELIER_POSITIONS.iter().enumerate() {
                parent
                    .spawn((
                        SpriteBundle {
                            texture: chandelier.clone(),
                            transform: Transform {
                                translation: (*hall_chancelier_position).into(),
                                ..default()
                            },
                            ..default()
                        },
                        Chandelier,
                        Name::new(format!("Chandelier {}", count + 1)),
                    ))
                    .with_children(|parent| {
                        // spawn 3 flame
                        for (flame_number, chancelier_flame_position) in
                            CHANDELIER_FLAME_POSITIONS.iter().enumerate()
                        {
                            parent.spawn((
                                SpriteSheetBundle {
                                    texture_atlas: texture_atlases
                                        .add(small_flame_texture_atlas.clone()),
                                    transform: Transform::from_translation(
                                        (*chancelier_flame_position).into(),
                                    ),
                                    ..default()
                                },
                                SpriteSheetAnimation {
                                    index: SpriteSheetIndex::new(
                                        0,
                                        small_flame_texture_atlas.clone().len() - 1,
                                    ),
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
        });
}

use super::{spawn_collision_cuboid, Location};
use crate::{
    animations::{
        fade::*,
        sprite_sheet_animation::{AnimationDuration, SpriteSheetAnimation},
    },
    constants::{
        locations::temple::*,
        player::{PLAYER_HITBOX_HEIGHT, PLAYER_HITBOX_WIDTH, PLAYER_HITBOX_Y_OFFSET},
        BACKGROUND_COLOR,
    },
    player::Player,
};
use bevy::{prelude::*, utils::Duration};
use bevy_rapier2d::prelude::*;

pub struct TemplePlugin;

impl Plugin for TemplePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(PlayerLocation::Temple)
            .add_state(PlayerCurtainsPosition::Below)
            .add_system_set(
                SystemSet::on_enter(Location::Temple)
                    .with_system(setup_temple)
                    .with_system(spawn_hitboxes),
            )
            .add_system_set(
                SystemSet::on_enter(PlayerLocation::SecretRoom)
                    .with_system(remove_secret_room_cover),
            )
            .add_system_set(
                SystemSet::on_exit(PlayerLocation::SecretRoom).with_system(add_secret_room_cover),
            )
            .add_system(pillars_position)
            .add_system(curtains_animation)
            .add_system(secret_room_enter)
            .add_system(curtains_z_position)
            .add_system(throne_position)
            .add_system(olf_cat_animation);
    }
}

#[derive(Component)]
struct Temple;
#[derive(Component)]
struct Pillar;
#[derive(Component)]
struct SecretRoom;
#[derive(Component)]
struct SecretRoomCover;
#[derive(Component)]
struct Curtain;
#[derive(Component)]
struct CurtainsZPositionTimer;
#[derive(Component)]
struct Throne;
#[derive(Component, Deref, DerefMut)]
struct ZPosition(f32);
#[derive(Component, Deref, DerefMut)]
struct CurtainsTimer(Timer);
#[derive(Component, Deref, DerefMut)]
struct OlfCatTimer(Timer);
#[derive(Component)]
struct SecretRoomSensor;

// States
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum PlayerLocation {
    Temple,
    SecretRoom,
}
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum PlayerCurtainsPosition {
    Above,
    Below,
}

fn pillars_position(
    player_query: Query<&GlobalTransform, With<Player>>,
    mut pillars_query: Query<&mut Transform, With<Pillar>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for mut pillar_transform in pillars_query.iter_mut() {
            if player_transform.translation.y + 60.0 > pillar_transform.translation.y {
                pillar_transform.translation.z = PILLARS_Z_FRONT;
            } else {
                pillar_transform.translation.z = PILLARS_Z_BACK;
            }
        }
    }
}

fn throne_position(
    player_query: Query<&GlobalTransform, With<Player>>,
    mut throne_query: Query<&mut Transform, With<Throne>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for mut throne_transform in throne_query.iter_mut() {
            if player_transform.translation.y > throne_transform.translation.y {
                throne_transform.translation.z = THRONE_Z_FRONT;
            } else {
                throne_transform.translation.z = THRONE_Z_BACK;
            }
        }
    }
}

fn secret_room_enter(
    // player_sensor_query: Query<Entity, With<PlayerSensorCollider>>,
    player_query: Query<&Transform, With<Player>>,
    sensor_query: Query<Entity, With<SecretRoomSensor>>,
    mut collision_events: EventReader<CollisionEvent>,
    mut player_location: ResMut<State<PlayerLocation>>,
) {
    for collision_event in collision_events.iter() {
        // if let CollisionEvent::Started(e1, e2, _) = collision_event {
        /*
        if let Ok(p_sensor_e) = player_sensor_query.get_single() {
            // let sensor_e = sensor_query.single();
            info!("EHHEHEHHEHE");
            info!("{collision_event:?} {p_sensor_e:?} {sensor_e:?}");

            if (*e1 == sensor_e && *e2 == p_sensor_e) || (*e1 == p_sensor_e && *e2 == sensor_e)
            {
                if player_location.current() == &PlayerLocation::Temple {
                    player_location.set(PlayerLocation::SecretRoom).unwrap();
                } else {
                    player_location.set(PlayerLocation::Temple).unwrap();
                }
            }
        }
        */

        if let Ok(transform) = player_query.get_single() {
            let sensor_e = sensor_query.single();

            match collision_event {
                // When the player goes through the sensor collider, change its location
                // to the secret room or the temple
                CollisionEvent::Started(e1, e2, _) if *e1 == sensor_e || *e2 == sensor_e => {
                    if player_location.current() == &PlayerLocation::Temple {
                        player_location.set(PlayerLocation::SecretRoom).unwrap();
                    } else {
                        player_location.set(PlayerLocation::Temple).unwrap();
                    }
                }
                CollisionEvent::Stopped(e1, e2, _) if *e1 == sensor_e || *e2 == sensor_e => {
                    // If the player changes direction while the sensor is still in its collider,
                    // check the top of its hitbox is in the temple or the secret room
                    if transform.translation.y + PLAYER_HITBOX_Y_OFFSET + PLAYER_HITBOX_WIDTH / 2.0
                        > SECRET_ROOM_TRIGGER_Y
                        && player_location.current() == &PlayerLocation::Temple
                    {
                        player_location.set(PlayerLocation::SecretRoom).unwrap();
                    } else if transform.translation.y
                        + PLAYER_HITBOX_Y_OFFSET
                        + PLAYER_HITBOX_WIDTH / 2.0
                        < SECRET_ROOM_TRIGGER_Y
                        && player_location.current() == &PlayerLocation::SecretRoom
                    {
                        player_location.set(PlayerLocation::Temple).unwrap();
                    }
                }
                _ => {}
            }
        }
    }
}

fn remove_secret_room_cover(
    mut commands: Commands,
    mut temple_query: Query<&mut Transform, With<Temple>>,
    mut secret_room_cover_query: Query<(Entity, Option<&mut Fade>), With<SecretRoomCover>>,
) {
    if let Ok((cover_entity, fade_opt)) = secret_room_cover_query.get_single_mut() {
        if let Some(mut fade) = fade_opt {
            fade.invert();
        } else {
            commands.entity(cover_entity).insert(Fade {
                current_alpha: 1.0,
                fade_type: FadeType::FadeIn,
                total_duration: Duration::from_secs(1),
                animation_fn: ease_in_sine,
                ..Fade::default()
            });
        }
    }

    if let Ok(mut temple_transform) = temple_query.get_single_mut() {
        temple_transform.translation.z = TEMPLE_Z_WHEN_IN_SECRET_ROOM;
    }
}

fn add_secret_room_cover(
    mut commands: Commands,
    mut temple_query: Query<&mut Transform, With<Temple>>,
    mut secret_room_cover_query: Query<(Entity, Option<&mut Fade>), With<SecretRoomCover>>,
) {
    if let Ok((cover_entity, fade_opt)) = secret_room_cover_query.get_single_mut() {
        if let Some(mut fade) = fade_opt {
            fade.invert();
        } else {
            commands.entity(cover_entity).insert(Fade {
                current_alpha: 0.0,
                fade_type: FadeType::FadeOut,
                total_duration: Duration::from_secs(1),
                animation_fn: ease_out_sine,
                ..Fade::default()
            });
        }
    }

    if let Ok(mut temple_transform) = temple_query.get_single_mut() {
        temple_transform.translation.z = TEMPLE_Z;
    }
}

fn curtains_animation(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut curtains_query: Query<(Entity, &Transform, &mut TextureAtlasSprite), With<Curtain>>,
    mut curtains_state: ResMut<State<PlayerCurtainsPosition>>,
    player_query: Query<&GlobalTransform, With<Player>>,
) {
    for collision_event in collision_events.iter() {
        // info!("testqskdjqlskdjs222IUEOAU82");
        if let CollisionEvent::Started(e1, e2, _) = collision_event {
            // info!("testqskdjqlskdjs");
            for (curtain_entity, curtain_transform, mut sprite) in curtains_query.iter_mut() {
                if *e1 == curtain_entity || *e2 == curtain_entity {
                    info!("{e1:?} {e2:?} {curtain_entity:?}");
                    let player_transform = player_query.single();

                    let (start, end) =
                        if player_transform.translation.x > curtain_transform.translation.x {
                            (0, 4)
                        } else {
                            (5, 9)
                        };

                    let player_hitbox_top_y = player_transform.translation.y
                        + PLAYER_HITBOX_Y_OFFSET
                        - PLAYER_HITBOX_HEIGHT;
                    let curtains_sensor_y =
                        curtain_transform.translation.y + CURTAINS_SENSOR_Y_OFFSET;

                    info!("{player_hitbox_top_y} {curtains_sensor_y}");
                    info!(
                        "{} {}",
                        player_transform.translation.y, curtain_transform.translation.y
                    );

                    if player_transform.translation.y >= curtains_sensor_y
                        && curtains_state.current() != &PlayerCurtainsPosition::Above
                    {
                        curtains_state.set(PlayerCurtainsPosition::Above).unwrap();
                        sprite.index = start;

                        commands
                            .spawn()
                            .insert(CurtainsTimer(Timer::from_seconds(
                                CURTAINS_CHANGE_Z_TIME,
                                false,
                            )))
                            .insert(ZPosition(CURTAINS_Z_FRONT));

                        commands
                            .entity(curtain_entity)
                            .insert(SpriteSheetAnimation {
                                start_index: start,
                                end_index: end,
                                timer: Timer::from_seconds(CURTAINS_ANIMATION_DELTA, true),
                                duration: AnimationDuration::Once,
                            });
                    } else if curtains_state.current() != &PlayerCurtainsPosition::Below {
                        curtains_state.set(PlayerCurtainsPosition::Below).unwrap();
                        sprite.index = start;

                        commands
                            .spawn()
                            .insert(CurtainsTimer(Timer::from_seconds(
                                CURTAINS_CHANGE_Z_TIME,
                                false,
                            )))
                            .insert(ZPosition(CURTAINS_Z_BACK));

                        commands
                            .entity(curtain_entity)
                            .insert(SpriteSheetAnimation {
                                start_index: start,
                                end_index: end,
                                timer: Timer::from_seconds(CURTAINS_ANIMATION_DELTA, true),
                                duration: AnimationDuration::Once,
                            });
                    }
                }
            }
        }
    }
}

/*
fn curtains_animation_(
    mut commands: Commands,
    mut curtains_state: ResMut<State<PlayerCurtainsPosition>>,
    mut curtain_query: Query<(Entity, &Transform, &mut TextureAtlasSprite), With<Curtain>>,
    player_query: Query<&GlobalTransform, With<Player>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (curtain_entity, curtain_transform, mut sprite) in curtain_query.iter_mut() {
            let range = PLAYER_WIDTH * PLAYER_SCALE;
            let in_range_left =
                curtain_transform.translation.x < player_transform.translation.x + range;
            let in_range_right =
                curtain_transform.translation.x > player_transform.translation.x - range;
            let in_range_down =
                curtain_transform.translation.y < player_transform.translation.y + 200.0;

            let (start, end) = if player_transform.translation.x > curtain_transform.translation.x {
                (0, 4)
            } else {
                (5, 9)
            };

            if player_transform.translation.y >= CURTAINS_TRIGGER_Y
                && in_range_left
                && in_range_right
                && curtains_state.current() == &PlayerCurtainsPosition::Below
            {
                curtains_state.set(PlayerCurtainsPosition::Above).unwrap();

                sprite.index = start;

                commands
                    .spawn()
                    .insert(CurtainsTimer(Timer::from_seconds(
                        CURTAINS_CHANGE_Z_TIME,
                        false,
                    )))
                    .insert(ZPosition(CURTAINS_Z_FRONT));

                commands
                    .entity(curtain_entity)
                    .insert(SpriteSheetAnimation {
                        start_index: start,
                        end_index: end,
                        timer: Timer::from_seconds(CURTAINS_ANIMATION_DELTA, true),
                        duration: AnimationDuration::Once,
                    });
            } else if player_transform.translation.y < CURTAINS_TRIGGER_Y
                && in_range_left
                && in_range_right
                && curtains_state.current() == &PlayerCurtainsPosition::Above
            {
                curtains_state.set(PlayerCurtainsPosition::Below).unwrap();

                if in_range_down {
                    sprite.index = start;

                    commands
                        .spawn()
                        .insert(CurtainsTimer(Timer::from_seconds(
                            CURTAINS_CHANGE_Z_TIME,
                            false,
                        )))
                        .insert(ZPosition(CURTAINS_Z_BACK));

                    commands
                        .entity(curtain_entity)
                        .insert(SpriteSheetAnimation {
                            start_index: start,
                            end_index: end,
                            timer: Timer::from_seconds(CURTAINS_ANIMATION_DELTA, true),
                            duration: AnimationDuration::Once,
                        });
                }
            }
        }
    }
}
*/

// Changes the Z position of the curtains after the player passes through them
fn curtains_z_position(
    mut commands: Commands,
    time: Res<Time>,
    mut timer_query: Query<(Entity, &mut CurtainsTimer, &ZPosition), With<CurtainsZPositionTimer>>,
    mut curtains_query: Query<&mut Transform, With<Curtain>>,
) {
    for (entity, mut timer, z_pos) in timer_query.iter_mut() {
        timer.tick(time.delta());

        if timer.finished() {
            commands.entity(entity).despawn();

            for mut curtains_transform in curtains_query.iter_mut() {
                curtains_transform.translation.z = **z_pos;
            }
        }
    }
}

// Animation of smol black cat
fn olf_cat_animation(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut OlfCatTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index as usize + 1) % texture_atlas.textures.len();
        }
    }
}

// Spawns all entity related to the temple
fn setup_temple(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let background = asset_server.load("textures/temple/background.png");
    let main_room = asset_server.load("textures/temple/main_room.png");
    let secret_room = asset_server.load("textures/temple/secret_room.png");
    let pillar = asset_server.load("textures/temple/pillar.png");
    let throne = asset_server.load("textures/temple/throne.png");
    let curtains_spritesheet = asset_server.load("textures/temple/curtains_sprite_sheet.png");
    let ground = asset_server.load("textures/temple/ground.png");
    let olf_cat_spritesheet = asset_server.load("textures/temple/olf_cat_spritesheet.png");
    let left_curtains_texture_atlas =
        TextureAtlas::from_grid(curtains_spritesheet.clone(), Vec2::new(200.0, 360.0), 1, 10);
    let right_curtains_texture_atlas =
        TextureAtlas::from_grid(curtains_spritesheet, Vec2::new(200.0, 360.0), 1, 10);
    let olf_cat_texture_atlas =
        TextureAtlas::from_grid(olf_cat_spritesheet, Vec2::new(100.0, 110.0), 2, 1);

    // Sensors colliders
    // Secret door sensor
    commands
        .spawn()
        .insert(Collider::segment(
            Vect::new(-235.0, SECRET_ROOM_TRIGGER_Y),
            Vect::new(-165.0, SECRET_ROOM_TRIGGER_Y),
        ))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(SecretRoomSensor)
        .insert(Sensor(true));

    // All the temple sprites
    commands.spawn_bundle(SpriteBundle {
        texture: background,
        transform: Transform::from_xyz(0.0, 0.0, BACKGROUND_Z),
        ..SpriteBundle::default()
    });

    commands
        .spawn_bundle(SpriteBundle {
            texture: main_room,
            transform: Transform::from_xyz(0.0, 0.0, TEMPLE_Z),
            ..SpriteBundle::default()
        })
        .insert(Temple);

    commands.spawn_bundle(SpriteBundle {
        texture: ground,
        transform: Transform::from_xyz(0.0, 0.0, GROUND_Z),
        ..SpriteBundle::default()
    });

    commands
        .spawn_bundle(SpriteBundle {
            texture: secret_room,
            transform: Transform::from_xyz(0.0, 0.0, SECRET_ROOM_Z),
            ..SpriteBundle::default()
        })
        .insert(SecretRoom);

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(0.0, 925.0, SECRET_ROOM_COVER_Z),
            sprite: Sprite {
                custom_size: Some(Vec2::new(2420.0, 670.0)),
                color: BACKGROUND_COLOR,
                ..Sprite::default()
            },
            ..SpriteBundle::default()
        })
        .insert(SecretRoomCover);

    commands
        .spawn_bundle(SpriteBundle {
            texture: throne,
            transform: Transform::from_xyz(0.0, 450.0, THRONE_Z_BACK),
            ..SpriteBundle::default()
        })
        .insert(Throne);

    // Left curtain, with a sensor collider to detect when the player passes through it
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlases.add(left_curtains_texture_atlas),
            transform: Transform::from_xyz(-200.0, 630.0, CURTAINS_Z_BACK),
            ..SpriteSheetBundle::default()
        })
        .insert(Collider::segment(
            Vect::new(-30.0, CURTAINS_SENSOR_Y_OFFSET),
            Vect::new(30.0, CURTAINS_SENSOR_Y_OFFSET),
        ))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Sensor(true))
        .insert(Curtain);

    // Right curtain
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlases.add(right_curtains_texture_atlas),
            transform: Transform::from_xyz(200.0, 630.0, CURTAINS_Z_BACK),
            ..SpriteSheetBundle::default()
        })
        .insert(Collider::segment(
            Vect::new(-30.0, CURTAINS_SENSOR_Y_OFFSET),
            Vect::new(30.0, CURTAINS_SENSOR_Y_OFFSET),
        ))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Sensor(true))
        .insert(Curtain);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlases.add(olf_cat_texture_atlas),
            transform: Transform {
                translation: Vec3::new(-200.0, 960.0, OLF_CAT_Z),
                scale: Vec3::new(OLF_CAT_SCALE, OLF_CAT_SCALE, 1.0),
                ..Transform::default()
            },
            ..SpriteSheetBundle::default()
        })
        .insert(OlfCatTimer(Timer::from_seconds(
            OLF_CAT_ANIMATION_DELTA,
            true,
        )));

    for pos in PILLAR_POSITIONS {
        commands
            .spawn_bundle(SpriteBundle {
                texture: pillar.clone(),
                transform: Transform::from_translation(pos.into()),
                ..SpriteBundle::default()
            })
            .with_children(|parent| {
                parent
                    .spawn()
                    .insert(Collider::cuboid(60.0, 20.0))
                    .insert(Transform::from_xyz(pos.0, pos.1 - 110.0, 0.0));
            })
            .insert(Pillar);
    }
}

fn spawn_hitboxes(mut commands: Commands) {
    // Left wall
    spawn_collision_cuboid(&mut commands, -1080.0, -40.0, 10.0, 1080.0);
    // Right wall
    spawn_collision_cuboid(&mut commands, 1080.0, -40.0, 10.0, 1080.0);
    // Left side of top wall
    spawn_collision_cuboid(&mut commands, -655.0, 530.0, 415.0, 30.0);
    // Right side of top wall
    spawn_collision_cuboid(&mut commands, 455.0, 530.0, 615.0, 30.0);
    // Bottom wall
    spawn_collision_cuboid(&mut commands, 0.0, -1130.0, 1070.0, 10.0);
    // Top wall of secret room
    spawn_collision_cuboid(&mut commands, 0.0, 1050.0, 1070.0, 10.0);
    // Middle wall of secret room
    spawn_collision_cuboid(&mut commands, 160.0, 850.0, 140.0, 190.0);
    // Throne seat
    spawn_collision_cuboid(&mut commands, 0.0, 410.0, 70.0, 40.0);
    // Throne front of seat
    spawn_collision_cuboid(&mut commands, 0.0, 360.0, 50.0, 10.0);
    // Throne front of front of seat
    spawn_collision_cuboid(&mut commands, 0.0, 340.0, 30.0, 10.0);
    // Throne bump left 1
    spawn_collision_cuboid(&mut commands, -330.0, 440.0, 1.0, 60.0);
    // Throne bump right 1
    spawn_collision_cuboid(&mut commands, 330.0, 440.0, 1.0, 60.0);
    // Throne bump left 2
    spawn_collision_cuboid(&mut commands, -310.0, 350.0, 1.0, 30.0);
    // Throne bump right 2
    spawn_collision_cuboid(&mut commands, 310.0, 350.0, 1.0, 30.0);
    // Throne bump left 3
    spawn_collision_cuboid(&mut commands, -290.0, 290.0, 1.0, 30.0);
    // Throne bump right 3
    spawn_collision_cuboid(&mut commands, 290.0, 290.0, 1.0, 30.0);
    // Throne bump left 4
    spawn_collision_cuboid(&mut commands, -230.0, 215.0, 1.0, 45.0);
    // Throne bump right 4
    spawn_collision_cuboid(&mut commands, 230.0, 215.0, 1.0, 45.0);
}
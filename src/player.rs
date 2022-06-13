use super::PlayerCamera;
use crate::{constants::player::*, material::CustomMaterial, GameState};
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::prelude::*;
use serde::Deserialize;
use std::collections::{HashMap, VecDeque};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerAnimationType::RightIdle)
            .insert_resource(KeyBindings {
                up: (KeyCode::Z, KeyCode::Up),
                down: (KeyCode::S, KeyCode::Down),
                right: (KeyCode::D, KeyCode::Right),
                left: (KeyCode::Q, KeyCode::Left),
            })
            .insert_resource(PlayerAnimationData(
                ron::de::from_bytes(include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/data/player_animations.ron"
                )))
                .unwrap(),
            ))
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_player))
            .add_system(animate_player)
            .add_system(set_player_movement)
            .add_system(player_movement)
            .add_system(camera_follow);
    }
}

#[derive(Component)]
pub struct Player;
#[derive(Component, Deref, DerefMut)]
struct Speed(f32);
#[derive(Component)]
struct Immobilized;
#[derive(Component)]
pub struct PlayerSensor;

#[derive(Deserialize, Debug)]
pub struct PlayerSpriteSheetAnimation {
    start_index: usize,
    end_index: usize,
    delta: f32,
}

#[derive(Deserialize, Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum PlayerAnimationType {
    RightIdle,
    LeftIdle,
    RightRun,
    LeftRun,
}

impl PlayerAnimationType {
    pub fn is_idle(&self) -> bool {
        matches!(
            &self,
            PlayerAnimationType::LeftIdle | PlayerAnimationType::RightIdle
        )
    }
}

#[derive(Deserialize, Component)]
struct PlayerAnimationData(HashMap<PlayerAnimationType, PlayerSpriteSheetAnimation>);

#[derive(Component)]
struct PlayerAnimation {
    timer: Timer,
    animation_type_queue: VecDeque<PlayerAnimationType>,
}

struct KeyBindings {
    up: (KeyCode, KeyCode),
    down: (KeyCode, KeyCode),
    left: (KeyCode, KeyCode),
    right: (KeyCode, KeyCode),
}

fn animate_player(
    time: Res<Time>,
    player_animations_data: Res<PlayerAnimationData>,
    mut query: Query<
        (&mut PlayerAnimation, &mut TextureAtlasSprite),
        (With<Player>, Without<Immobilized>),
    >,
) {
    for (mut player_animation, mut sprite) in query.iter_mut() {
        player_animation.timer.tick(time.delta());

        if let Some(animation_type) = player_animation.animation_type_queue.get(0) {
            if player_animation.timer.finished() {
                if sprite.index == player_animations_data.0[animation_type].end_index {
                    sprite.index = player_animations_data.0[animation_type].start_index;
                } else {
                    sprite.index += 1;
                }
            }
        }
    }
}

fn set_player_movement(
    key_bindings: Res<KeyBindings>,
    keyboard_input: Res<Input<KeyCode>>,
    player_animations_data: Res<PlayerAnimationData>,
    mut query: Query<
        (&mut PlayerAnimation, &mut TextureAtlasSprite),
        (With<Player>, Without<Immobilized>),
    >,
) {
    for (mut player_animation, mut sprite) in query.iter_mut() {
        let mut restart_animation = false;
        let start_anim_type = player_animation.animation_type_queue[0];

        if keyboard_input.just_released(key_bindings.right.0)
            || keyboard_input.just_released(key_bindings.right.1)
        {
            // player_animation.animation_type_queue.retain(|t| t.is_idle());
            player_animation
                .animation_type_queue
                .retain(|t| *t != PlayerAnimationType::RightRun);
            player_animation
                .animation_type_queue
                .push_back(PlayerAnimationType::RightIdle);
            restart_animation = true;
        } else if keyboard_input.just_released(key_bindings.left.0)
            || keyboard_input.just_released(key_bindings.left.1)
        {
            player_animation
                .animation_type_queue
                .retain(|t| *t != PlayerAnimationType::LeftRun);
            // player_animation.animation_type_queue.retain(|t| t.is_idle());
            player_animation
                .animation_type_queue
                .push_back(PlayerAnimationType::LeftIdle);
            restart_animation = true;
        } else if keyboard_input.just_pressed(key_bindings.up.0)
            || keyboard_input.just_released(key_bindings.up.1)
            || keyboard_input.just_released(key_bindings.down.1)
            || keyboard_input.just_released(key_bindings.down.0)
        {
            restart_animation = true;
        }

        if keyboard_input.just_pressed(key_bindings.right.0)
            || keyboard_input.just_pressed(key_bindings.right.1)
        {
            player_animation
                .animation_type_queue
                .retain(|t| !t.is_idle());
            player_animation
                .animation_type_queue
                .push_front(PlayerAnimationType::RightRun);
            restart_animation = true;
        } else if keyboard_input.just_pressed(key_bindings.left.0)
            || keyboard_input.just_pressed(key_bindings.left.1)
        {
            player_animation
                .animation_type_queue
                .retain(|t| !t.is_idle());
            player_animation
                .animation_type_queue
                .push_front(PlayerAnimationType::LeftRun);
            restart_animation = true;
        } else if keyboard_input.just_pressed(key_bindings.up.0)
            || keyboard_input.just_pressed(key_bindings.up.1)
            || keyboard_input.just_pressed(key_bindings.down.0)
            || keyboard_input.just_pressed(key_bindings.down.1)
        {
            restart_animation = true;
        }

        if restart_animation && start_anim_type != player_animation.animation_type_queue[0] {
            let animation_data =
                &player_animations_data.0[&player_animation.animation_type_queue[0]];
            sprite.index = animation_data.start_index + 1;
            player_animation.timer = Timer::from_seconds(animation_data.delta, true);
        }
    }
}

fn player_movement(
    key_bindings: Res<KeyBindings>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Speed, &mut Velocity), (With<Player>, Without<Immobilized>)>,
) {
    for (speed, mut rb_vel) in player_query.iter_mut() {
        let up =
            keyboard_input.pressed(key_bindings.up.0) || keyboard_input.pressed(key_bindings.up.1);
        let down = keyboard_input.pressed(key_bindings.down.0)
            || keyboard_input.pressed(key_bindings.down.1);
        let left = keyboard_input.pressed(key_bindings.left.0)
            || keyboard_input.pressed(key_bindings.left.1);
        let right = keyboard_input.pressed(key_bindings.right.0)
            || keyboard_input.pressed(key_bindings.right.1);

        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        let mut vel_x = x_axis as f32 * **speed;
        let mut vel_y = y_axis as f32 * **speed;

        if x_axis != 0 && y_axis != 0 {
            vel_x *= (std::f32::consts::PI / 4.0).cos();
            vel_y *= (std::f32::consts::PI / 4.0).cos();
        }

        rb_vel.linvel.x = vel_x;
        rb_vel.linvel.y = vel_y;
    }
}

fn camera_follow(
    mut query: ParamSet<(
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<PlayerCamera>>,
    )>,
) {
    if let Ok(t) = query.p0().get_single() {
        let player_transform = *t;

        if let Ok(mut camera_transform) = query.p1().get_single_mut() {
            camera_transform.translation = camera_transform.translation.lerp(
                Vec3::new(
                    player_transform.translation.x,
                    player_transform.translation.y,
                    camera_transform.translation.z,
                ),
                CAMERA_INTERPOLATION,
            );
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_animations_data: Res<PlayerAnimationData>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut my_material_assets: ResMut<Assets<CustomMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let texture_handle = asset_server.load("textures/fabien_info_spritesheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT), 4, 4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let random_texture = asset_server.load("textures/hud/stained_glass_opened.png");

    commands.spawn().insert_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        material: my_material_assets.add(CustomMaterial {
            brightness: 0.5,
            progress: 0.5,
            texture: random_texture,
        }),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 15.0),
            scale: Vec3::new(100.0, 100.0, 0.0),
            ..Transform::default()
        },
        ..MaterialMesh2dBundle::default()
    });

    commands
        .spawn()
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                // translation: Vec3::new(-200.0, -1500.0, PLAYER_Z),
                translation: Vec3::new(0.0, 0.0, PLAYER_Z),
                scale: Vec3::splat(PLAYER_SCALE),
                ..Transform::default()
            },
            ..SpriteSheetBundle::default()
        })
        .insert(PlayerAnimation {
            timer: Timer::from_seconds(player_animations_data.0[&STARTING_ANIMATION].delta, true),
            animation_type_queue: vec![STARTING_ANIMATION].into(),
        })
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Velocity {
            linvel: Vect::ZERO,
            angvel: 0.0,
        })
        .insert_bundle((Player, Speed(200.0)))
        .with_children(|parent| {
            parent
                .spawn()
                .insert(Collider::cuboid(PLAYER_HITBOX_WIDTH, PLAYER_HITBOX_HEIGHT))
                .insert(Transform::from_xyz(0.0, PLAYER_HITBOX_Y_OFFSET, 0.0));

            parent
                .spawn()
                .insert(Collider::segment(
                    Vect::new(-PLAYER_HITBOX_WIDTH, 0.0),
                    Vect::new(PLAYER_HITBOX_WIDTH, 0.0),
                ))
                .insert(Sensor(true))
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(ActiveCollisionTypes::STATIC_STATIC)
                .insert(PlayerSensor);
        });
}

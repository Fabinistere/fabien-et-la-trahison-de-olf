use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use super::PlayerCamera;
use crate::{
    GameState,
    constants::player::*,
};
use serde::Deserialize;
use std::collections::{ HashMap, VecDeque };

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(PlayerAnimationType::RightIdle)
            .insert_resource(KeyBindings {
                up: (KeyCode::Z, KeyCode::Up),
                down: (KeyCode::S, KeyCode::Down),
                right: (KeyCode::D, KeyCode::Right),
                left: (KeyCode::Q, KeyCode::Left),
            })
            .insert_resource(PlayerAnimationData(
                ron::de::from_bytes(include_bytes!(
                    concat!(env!("CARGO_MANIFEST_DIR"), "/data/player_animations.ron")
                )).unwrap()
            ))
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(spawn_player.system())
            )
            .add_system(animate_player.system())
            .add_system(set_player_movement.system())
            .add_system(player_movement.system());
    }
}

pub struct Player;
struct Speed(f32);
struct Immobilized;

#[derive(Deserialize, Debug)]
pub struct SpriteSheetAnimation {
    start_index: u32,
    end_index: u32,
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
        matches!(&self, PlayerAnimationType::LeftIdle | PlayerAnimationType::RightIdle)
    }
}

#[derive(Deserialize)]
struct PlayerAnimationData(HashMap<PlayerAnimationType, SpriteSheetAnimation>);

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
            player_animation.animation_type_queue.retain(|t| *t != PlayerAnimationType::RightRun);
            player_animation.animation_type_queue.push_back(PlayerAnimationType::RightIdle);
            restart_animation = true;
        } else if keyboard_input.just_released(key_bindings.left.0)
            || keyboard_input.just_released(key_bindings.left.1)
        {
            player_animation.animation_type_queue.retain(|t| *t != PlayerAnimationType::LeftRun);
            // player_animation.animation_type_queue.retain(|t| t.is_idle());
            player_animation.animation_type_queue.push_back(PlayerAnimationType::LeftIdle);
            restart_animation = true;
        } else if keyboard_input.just_pressed(key_bindings.up.0)
            || keyboard_input.just_released(key_bindings.up.1)
        {
            restart_animation = true;
        } else if keyboard_input.just_released(key_bindings.down.0)
            || keyboard_input.just_released(key_bindings.down.1)
        {
            restart_animation = true;
        }

        if keyboard_input.just_pressed(key_bindings.right.0)
            || keyboard_input.just_pressed(key_bindings.right.1)
        {
            player_animation.animation_type_queue.retain(|t| !t.is_idle());
            player_animation.animation_type_queue.push_front(PlayerAnimationType::RightRun);
            restart_animation = true;
        } else if keyboard_input.just_pressed(key_bindings.left.0)
            || keyboard_input.just_pressed(key_bindings.left.1)
        {
            player_animation.animation_type_queue.retain(|t| !t.is_idle());
            player_animation.animation_type_queue.push_front(PlayerAnimationType::LeftRun);
            restart_animation = true;
        } else if keyboard_input.just_pressed(key_bindings.up.0)
            || keyboard_input.just_pressed(key_bindings.up.1)
        {
            restart_animation = true;
        } else if keyboard_input.just_pressed(key_bindings.down.0)
            || keyboard_input.just_pressed(key_bindings.down.1)
        {
            restart_animation = true;
        }

        if restart_animation && start_anim_type != player_animation.animation_type_queue[0] {
            let animation_data = &player_animations_data.0[&player_animation.animation_type_queue[0]];
            sprite.index = animation_data.start_index + 1;
            player_animation.timer = Timer::from_seconds(animation_data.delta, true);
        }
    }
}

fn player_movement(
    key_bindings: Res<KeyBindings>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Speed, &mut RigidBodyVelocity, &GlobalTransform), (With<Player>, Without<Immobilized>)>,
    mut camera_query: Query<&mut Transform, With<PlayerCamera>>,
) {
    for (speed, mut rb_vel, player_transform) in player_query.iter_mut() {
        let up = keyboard_input.pressed(key_bindings.up.0) || keyboard_input.pressed(key_bindings.up.1);
        let down = keyboard_input.pressed(key_bindings.down.0) || keyboard_input.pressed(key_bindings.down.1);
        let left = keyboard_input.pressed(key_bindings.left.0) || keyboard_input.pressed(key_bindings.left.1);
        let right = keyboard_input.pressed(key_bindings.right.0) || keyboard_input.pressed(key_bindings.right.1);

        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        let mut vel_x = x_axis as f32 * speed.0;
        let mut vel_y = y_axis as f32 * speed.0;

        if x_axis != 0 && y_axis != 0 {
            vel_x *= (std::f32::consts::PI / 4.0).cos();
            vel_y *= (std::f32::consts::PI / 4.0).cos();
        }

        rb_vel.linvel.x = vel_x;
        rb_vel.linvel.y = vel_y;

        for mut camera_transform in camera_query.iter_mut() {
            camera_transform.translation.x = player_transform.translation.x;
            camera_transform.translation.y = player_transform.translation.y;
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_animations_data: Res<PlayerAnimationData>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("textures/fabien_info_spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT), 4, 4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn()
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_matrix(
                Mat4::from_scale_rotation_translation(
                    Vec3::splat(PLAYER_SCALE),
                    Quat::default(),
                    Vec3::new(0.0, 0.0, PLAYER_Z),
                )
            ),
            ..SpriteSheetBundle::default()
        })
        .insert(PlayerAnimation {
            timer: Timer::from_seconds(
                player_animations_data.0[&STARTING_ANIMATION].delta,
                true,
            ),
            animation_type_queue: vec![STARTING_ANIMATION].into(),
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Dynamic,
            mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
            position: Vec2::new(0.0, 0.0).into(),
            ..RigidBodyBundle::default()
        })
        .insert_bundle((
                RigidBodyPositionSync::Discrete,
                Player,
                Speed(500.0),
        ))
        .with_children(|parent| {
            parent.spawn().insert_bundle(ColliderBundle {
                shape: ColliderShape::cuboid(35.0, 20.0),
                position: Vec2::new(0.0, -30.0).into(),
                material: ColliderMaterial {
                    friction: 0.0,
                    restitution: 0.0,
                    ..ColliderMaterial::default()
                },
                ..ColliderBundle::default()
            });
        });
}

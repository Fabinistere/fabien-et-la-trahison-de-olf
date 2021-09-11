use bevy::prelude::*;
use crate::GameState;
use std::collections::{ HashMap, VecDeque };
use serde::Deserialize;

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

struct Player;
struct Speed(f32);

#[derive(Deserialize, Debug)]
pub struct SpriteSheetAnimation {
    start_index: u32,
    end_index: u32,
    delta: f32,
}

#[derive(Deserialize, Copy, Clone, PartialEq, Eq, Debug, Hash)]
enum PlayerAnimationType {
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
        With<Player>,
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
        With<Player>,
    >,
) {
    for (mut player_animation, mut sprite) in query.iter_mut() {
        let mut restart_animation = false;

        if keyboard_input.just_released(key_bindings.right.0)
            || keyboard_input.just_released(key_bindings.right.1)
        {
            player_animation.animation_type_queue.retain(|t| *t != PlayerAnimationType::RightRun);
            player_animation.animation_type_queue.push_back(PlayerAnimationType::RightIdle);
            restart_animation = true;
        } else if keyboard_input.just_released(key_bindings.left.0)
            || keyboard_input.just_released(key_bindings.left.1)
        {
            player_animation.animation_type_queue.retain(|t| *t != PlayerAnimationType::LeftRun);
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

        if restart_animation {
            if let Some(animation_type) = player_animation.animation_type_queue.get(0) {
                let animation_data = &player_animations_data.0[animation_type];
                sprite.index = animation_data.start_index;
                player_animation.timer = Timer::from_seconds(animation_data.delta, true);
            }
        }
    }
}

fn player_movement(
    time: Res<Time>,
    key_bindings: Res<KeyBindings>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Speed, &mut Transform), With<Player>>,
) {
    for (speed, mut transform) in player_query.iter_mut() {
        let up = keyboard_input.pressed(key_bindings.up.0) || keyboard_input.pressed(key_bindings.up.1);
        let down = keyboard_input.pressed(key_bindings.down.0) || keyboard_input.pressed(key_bindings.down.1);
        let left = keyboard_input.pressed(key_bindings.left.0) || keyboard_input.pressed(key_bindings.left.1);
        let right = keyboard_input.pressed(key_bindings.right.0) || keyboard_input.pressed(key_bindings.right.1);

        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        let mut delta_x = x_axis as f32 * speed.0 * time.delta().as_secs_f32();
        let mut delta_y = y_axis as f32 * speed.0 * time.delta().as_secs_f32();

        if x_axis != 0 && y_axis != 0 {
            let distance = (delta_x.powf(2.0) + delta_y.powf(2.0)).sqrt();
            delta_x -= delta_x / distance;
            delta_y -= delta_y / distance;
        }

        transform.translation += Vec3::new(delta_x, delta_y, 0.0);
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_animations_data: Res<PlayerAnimationData>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    const STARTING_ANIMATION: PlayerAnimationType = PlayerAnimationType::RightIdle;

    let texture_handle = asset_server.load("sprites/fabien_info_spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(12.0, 15.0), 4, 4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn()
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(10.0)),
            ..SpriteSheetBundle::default()
        })
        .insert(PlayerAnimation {
            timer: Timer::from_seconds(
                player_animations_data.0[&STARTING_ANIMATION].delta,
                true,
            ),
            animation_type_queue: vec![STARTING_ANIMATION].into(),
        })
        .insert(Player)
        .insert(Speed(200.0));
}

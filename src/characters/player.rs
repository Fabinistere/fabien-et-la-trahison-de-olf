use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::{BTreeMap, HashMap};
use yml_dialog::DialogNode;

use crate::{
    animations::{
        sprite_sheet_animation::{AnimationIndices, CharacterState},
        CharacterSpriteSheet,
    },
    characters::{
        movement::{MovementBundle, Speed},
        CharacterHitbox,
    },
    combat::{Leader, Reputation},
    constants::character::{player::*, *},
    controls::KeyBindings,
    hud_closed,
    ui::dialog_systems::DialogMap,
    GameState, PlayerCamera,
};

use super::movement::CharacterCloseSensor;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, (player_movement.run_if(hud_closed), camera_follow));
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
struct Immobilized;

#[derive(Component)]
pub struct PlayerInteractionSensor;

#[derive(Component)]
pub struct PlayerCloseSensor;

fn player_movement(
    key_bindings: Res<KeyBindings>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<
        (
            Entity,
            &Speed,
            &mut Velocity,
            &mut TextureAtlasSprite,
            &mut CharacterState,
        ),
        With<Player>,
    >,
) {
    if let Ok((_player, speed, mut rb_vel, mut texture_atlas_sprite, mut player_state)) =
        player_query.get_single_mut()
    {
        let up = keyboard_input.any_pressed(key_bindings.up());
        let down = keyboard_input.any_pressed(key_bindings.down());
        let left = keyboard_input.any_pressed(key_bindings.left());
        let right = keyboard_input.any_pressed(key_bindings.right());

        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        let mut vel_x = x_axis as f32 * **speed;
        let mut vel_y = y_axis as f32 * **speed;

        if x_axis != 0 && y_axis != 0 {
            vel_x *= (std::f32::consts::PI / 4.).cos();
            vel_y *= (std::f32::consts::PI / 4.).cos();
        }

        // rb_vel.linvel.x = x_axis as f32 * **speed * 200. * time.delta_seconds();
        rb_vel.linvel.x = vel_x;
        rb_vel.linvel.y = vel_y;

        /* -------------------------------------------------------------------------- */
        /*                                  Animation                                 */
        /* -------------------------------------------------------------------------- */

        // if there is any movement
        if (left || right || up || down) && *player_state != CharacterState::Run {
            *player_state = CharacterState::Run;
        } else if !(left || right || up || down)
            && *player_state == CharacterState::Run
            && *player_state != CharacterState::Idle
        {
            // IDEA: Polish #visual - When we reach max speed (one full run loop), whenever you stop there is a smoke anim (sudden braking)
            *player_state = CharacterState::Idle;
        }

        /* -------------------------------------------------------------------------- */
        /*                                  Direction                                 */
        /* -------------------------------------------------------------------------- */

        if !(left && right) {
            if right {
                texture_atlas_sprite.flip_x = false;
            } else if left {
                texture_atlas_sprite.flip_x = true;
            }
        }
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
    characters_spritesheet: Res<CharacterSpriteSheet>,
    mut dialogs: ResMut<DialogMap>,
) {
    /* -------------------------------------------------------------------------- */
    /*                              Animation Indices                             */
    /* -------------------------------------------------------------------------- */

    let mut animation_indices = AnimationIndices(HashMap::new());
    animation_indices.insert(CharacterState::Idle, PLAYER_IDLE_FRAMES);
    animation_indices.insert(CharacterState::Run, PLAYER_RUN_FRAMES);

    /* -------------------------------------------------------------------------- */
    /*                                  Textures                                  */
    /* -------------------------------------------------------------------------- */

    let player = commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: characters_spritesheet.texture_atlas.clone(),
                transform: Transform {
                    translation: PLAYER_SPAWN.into(),
                    scale: Vec3::splat(PLAYER_SCALE),
                    ..Transform::default()
                },
                ..default()
            },
            Name::new("Player"),
            Player,
            // -- Social --
            Reputation::new(100, 0),
            Leader,
            // -- Animation --
            MovementBundle {
                animation_indices,
                ..default()
            },
            // -- Hitbox --
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
        ))
        .with_children(|parent| {
            parent.spawn((
                Collider::cuboid(CHAR_HITBOX_WIDTH, CHAR_HITBOX_HEIGHT),
                Transform::from_xyz(0., CHAR_HITBOX_Y_OFFSET, 0.),
                CharacterHitbox,
                Name::new("Player Hitbox"),
            ));

            parent.spawn((
                Collider::segment(
                    Vect::new(-CHAR_HITBOX_WIDTH, CHAR_SENSOR_Y_OFFSET),
                    Vect::new(CHAR_HITBOX_WIDTH, CHAR_SENSOR_Y_OFFSET),
                ),
                Sensor,
                ActiveEvents::COLLISION_EVENTS,
                ActiveCollisionTypes::STATIC_STATIC,
                PlayerInteractionSensor,
                Name::new("Player Interaction Sensor"),
            ));

            parent.spawn((
                Collider::ball(10.),
                Sensor,
                ActiveEvents::COLLISION_EVENTS,
                ActiveCollisionTypes::STATIC_STATIC,
                PlayerCloseSensor,
                CharacterCloseSensor,
                Name::new("Player Close Sensor"),
            ));
        })
        .id();

    /* -------------------------------------------------------------------------- */
    /*                                   Dialog                                   */
    /* -------------------------------------------------------------------------- */

    let player_dialog_file = std::fs::File::open("data/self_player_dialog.yml").unwrap();
    let player_deserialized_map: BTreeMap<usize, DialogNode> =
        serde_yaml::from_reader(player_dialog_file).unwrap();
    dialogs.insert(
        player,
        (
            *player_deserialized_map.first_key_value().unwrap().0,
            player_deserialized_map,
        ),
    );
}

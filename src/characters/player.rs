use bevy::{input::common_conditions::input_just_pressed, prelude::*, transform::commands};
use bevy_rapier2d::prelude::*;
use std::collections::{BTreeMap, HashMap};
use yml_dialog::DialogNode;

use crate::{
    animations::{
        sprite_sheet_animation::{AnimationIndices, CharacterState, TempoAnimation},
        CharacterSpriteSheet,
    },
    characters::{
        movement::{MovementBundle, Speed},
        CharacterHitbox,
    },
    combat::{Leader, Reputation},
    constants::{
        character::{player::*, *},
        locations::main_room::THRONE_POSITION,
    },
    controls::KeyBindings,
    cutscene::player_is_in_control,
    hud_closed,
    locations::temple::Location,
    ui::dialog_systems::DialogMap,
    GameState,
};

use super::{movement::CharacterCloseSensor, Character};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(
                Update,
                (
                    player_movement
                        .run_if(hud_closed)
                        .run_if(player_is_in_control),
                    player_animation,
                    player_squat.run_if(input_just_pressed(KeyCode::ControlLeft)),
                ),
            );
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerHitbox;

#[derive(Component)]
struct Immobilized;

#[derive(Component)]
pub struct PlayerInteractionSensor;

#[derive(Component)]
pub struct PlayerCloseSensor;

fn player_animation(
    mut player_query: Query<
        (&Velocity, &mut TextureAtlasSprite, &mut CharacterState),
        (Changed<Velocity>, With<Player>),
    >,
) {
    if let Ok((rb_vel, mut texture_atlas_sprite, mut player_state)) = player_query.get_single_mut()
    {
        /* -------------------------------------------------------------------------- */
        /*                                  Animation                                 */
        /* -------------------------------------------------------------------------- */

        // if there is any movement
        if (rb_vel.linvel.x != 0. || rb_vel.linvel.y != 0.) && *player_state != CharacterState::Run
        {
            *player_state = CharacterState::Run;
        } else if rb_vel.linvel.x == 0.
            && rb_vel.linvel.y == 0.
            && *player_state == CharacterState::Run
            && *player_state != CharacterState::Idle
        {
            // IDEA: Polish #visual - When we reach max speed (one full run loop), whenever you stop there is a smoke anim (sudden braking)
            *player_state = CharacterState::Idle;
        }

        /* -------------------------------------------------------------------------- */
        /*                                  Direction                                 */
        /* -------------------------------------------------------------------------- */

        if rb_vel.linvel.x > 0. {
            texture_atlas_sprite.flip_x = false;
        } else if rb_vel.linvel.x < 0. {
            texture_atlas_sprite.flip_x = true;
        }
    }
}

/// NOTE: can't let the player squat down (we only trigger the idle anim) -> bypass the anim system
fn player_squat(
    mut commands: Commands,
    mut player_query: Query<(Entity, &AnimationIndices, &mut TextureAtlasSprite), With<Player>>,
) {
    if let Ok((player, indices, mut sprite)) = player_query.get_single_mut() {
        let (start_anim, _, _) = &indices.get(&CharacterState::Idle).unwrap();
        sprite.index = *start_anim;
        commands.entity(player).remove::<TempoAnimation>();
    }
}

fn player_movement(
    key_bindings: Res<KeyBindings>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(Entity, &Speed, &mut Velocity), With<Player>>,
) {
    if let Ok((_player, speed, mut rb_vel)) = player_query.get_single_mut() {
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
                    translation: THRONE_POSITION.into(), // PLAYER_SPAWN.into(),
                    scale: Vec3::splat(PLAYER_SCALE),
                    ..Transform::default()
                },
                ..default()
            },
            Name::new("Player"),
            Character,
            Player,
            // Location::default(),
            Location::Temple,
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
            // 10 = Cannot be moved by anything
            // Dominance::group(1),
            LockedAxes::ROTATION_LOCKED,
        ))
        .with_children(|parent| {
            parent.spawn((
                Collider::cuboid(CHAR_HITBOX_WIDTH, CHAR_HITBOX_HEIGHT),
                Transform::from_xyz(0., CHAR_HITBOX_Y_OFFSET, 0.),
                PlayerHitbox,
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

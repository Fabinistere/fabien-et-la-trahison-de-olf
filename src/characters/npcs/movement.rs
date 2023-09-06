//! Implements Npc for moving and steering entities.

use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use rand::Rng;
use std::time::Duration;

use crate::{
    animations::sprite_sheet_animation::CharacterState,
    // GameState,
    characters::{
        movement::Speed,
        npcs::{
            aggression::StopChaseEvent,
            idle::{IdleBehavior, RestTime},
            NPC,
        },
    },
    combat::{CombatEvent, InCombat, Leader, Reputation},
    constants::{character::npc::movement::*, TILE_SIZE},
};

/// Indicates that an entity should run towards a destination and which.
#[derive(Default, Component)]
pub struct JustWalkBehavior {
    pub destination: Vec3,
}

#[derive(Default, Component)]
pub struct FollowupBehavior;

#[derive(Default, Component)]
pub struct DetectionBehavior;

#[derive(Default, Component)]
pub struct PursuitBehavior;
// pub const PROXIMITY_RADIUS: f32 = 64.;

#[derive(Clone, Copy, Component)]
pub struct Target(pub Option<Entity>);

impl Default for Target {
    fn default() -> Self {
        Target { 0: None }
    }
}

// REFACTOR: The whole movement plugin (close sensor, smooth movement)

pub fn animation(
    mut npc_query: Query<(&Velocity, &mut CharacterState, &mut TextureAtlasSprite), With<NPC>>,
) {
    for (rb_vel, mut npc_state, mut texture_atlas_sprite) in &mut npc_query {
        /* -------------------------------------------------------------------------- */
        /*                                  Animation                                 */
        /* -------------------------------------------------------------------------- */

        // if there is any movement
        if rb_vel.linvel.x != 0. && rb_vel.linvel.y != 0. && *npc_state != CharacterState::Run {
            *npc_state = CharacterState::Run;
        } else if rb_vel.linvel.x == 0.
            && rb_vel.linvel.y == 0.
            && *npc_state == CharacterState::Run
            && *npc_state != CharacterState::Idle
        {
            // IDEA: Polish #visual - When we reach max speed (one full run loop), whenever you stop there is a smoke anim (sudden braking)
            *npc_state = CharacterState::Idle;
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

// TODO: feature - use ColliderType::Sensor to delimiter zone

/// For a certain destination contained in [RunToDestinationbehavior], make the npc run towards it
pub fn just_walk(
    mut commands: Commands,
    mut npc_query: Query<
        (
            Entity,
            &mut JustWalkBehavior,
            &Transform,
            &Speed,
            &mut Velocity,
            &Name,
        ),
        (
            With<JustWalkBehavior>,
            Without<IdleBehavior>,
            Without<PursuitBehavior>,
            Without<InCombat>,
        ),
    >,
) {
    for (npc, mut behavior, transform, speed, mut rb_vel, name) in npc_query.iter_mut() {
        let direction: Vec3 = behavior.destination;

        // XXX: Approximation Louche
        if !close(transform.translation, direction, TILE_SIZE / 2.) {
            // println!(
            //     "{} direction: ({},{}) \nposition: ({},{})",
            //     name, direction.x, direction.y,
            //     transform.translation.x, transform.translation.y
            // );

            let (vel_x, vel_y) = move_to_dest(direction, transform, speed);

            rb_vel.linvel.x = vel_x;
            rb_vel.linvel.y = vel_y;
        } else {
            info!(target: "Start Rest", "{:?}, {}", npc, name);

            // Stop the npc after reaching the destination
            // rb_vel.linvel.x = 0.;
            // rb_vel.linvel.y = 0.;

            // change their destiantion before resting
            // REFACTOR: handle the direction change with event like in do_flexing
            behavior.destination = give_a_direction();

            commands.entity(npc).insert(IdleBehavior);
            // println!("postChange: npc's state: {:#?}", npc.state);

            // REFACTOR: change this part by sending a event : FREEZE
            commands.entity(npc).insert(RestTime {
                // create the non-repeating rest timer
                timer: Timer::new(Duration::from_secs(REST_TIMER), TimerMode::Once),
            });
        }
    }
}

/// Entity gently follows their target.
/// depending the team
///
/// TODO: feature - Follow an ally by the component Target instead of Leader
pub fn follow(
    // mut commands: Commands,
    mut npc_query: Query<
        (Entity, &Transform, &Speed, &mut Velocity, &Reputation),
        (
            With<NPC>,
            With<FollowupBehavior>,
            Without<PursuitBehavior>,
            Without<InCombat>,
        ), // only npc can follow
    >,
    targets_query: Query<(&GlobalTransform, &Reputation, &Name), With<Leader>>,
    // pos_query: Query<&GlobalTransform>,
) {
    for (_npc, transform, speed, mut rb_vel, reputation) in npc_query.iter_mut() {
        for (target_transform, target_reputation, _name) in targets_query.iter() {
            // println!("target: {name}, Leader of team {:#?}", target_reputation);
            if reputation.in_the_same_team(target_reputation) {
                // carefull with more than one leader per team
                // it will be not nice

                // XXX: Approximation Louche
                // REFACTOR: Sensor Detection
                if !close(
                    transform.translation,
                    target_transform.translation(),
                    20. * TILE_SIZE,
                ) {
                    // println!("moving towards target: {}", name);

                    let (vel_x, vel_y) = move_to(target_transform, transform, speed);

                    rb_vel.linvel.x = vel_x;
                    rb_vel.linvel.y = vel_y;
                }
                // if reached the target
                else {
                    // TODO: feature - AVOID npc to merge with the target
                    rb_vel.linvel.x = 0.;
                    rb_vel.linvel.y = 0.;
                }
            }
        }
    }

    // target does not have position. Go to idle state
    // commands.entity(npc).remove::<FollowupBehavior>();
    // commands.entity(npc).remove::<RunToDestinationBehavior>();
    // commands.entity(npc).insert(IdleBehavior);

    // println!("pursue: {:?} entities, {:?} err, {:?} ok.", query.iter_mut().len(), err_count, ok_count);
}

/// Entity chases their target.
/// This target has entered in the detection range of the npc
pub fn pursue(
    // mut game_state: ResMut<State<GameState>>,
    mut npc_query: Query<
        (
            Entity,
            &Transform,
            &Speed,
            &mut Velocity,
            &Reputation,
            &Target,
            &Children,
            &Name,
        ),
        (With<NPC>, With<PursuitBehavior>, Without<InCombat>),
    >,
    pos_query: Query<&GlobalTransform>,
    mut ev_combat: EventWriter<CombatEvent>,
    mut ev_stop_chase: EventWriter<StopChaseEvent>,
) {
    for (npc, transform, speed, mut rb_vel, _reputation, target, _colliders, name) in
        npc_query.iter_mut()
    {
        if target.0.is_none() {
            info!(target: "target is none", "{}", name);
            continue;
        }

        let result = pos_query.get_component::<GlobalTransform>(target.0.expect("target is none"));
        match result {
            Err(_) => {
                // target does not have position. Disengage.
                ev_stop_chase.send(StopChaseEvent { npc_entity: npc });
                continue;
            }
            Ok(target_transform) => {
                // If the target is too far away
                // adjust npc's velocity to reach it
                if !close(
                    transform.translation,
                    target_transform.translation(),
                    10. * TILE_SIZE,
                ) {
                    // println!("moving towards target: {}", name);
                    let (vel_x, vel_y) = move_to(target_transform, transform, speed);

                    rb_vel.linvel.x = vel_x;
                    rb_vel.linvel.y = vel_y;
                } else {
                    info!("Target Caught in 4K by {:?} {}", npc, name);

                    // open HUD to combat talk after chase
                    ev_combat.send(CombatEvent { entity: npc });

                    ev_stop_chase.send(StopChaseEvent { npc_entity: npc });

                    // handle flee when pressing o or moving ?
                    // (timer on npc before rechase)
                    // 'global' timer (on player)
                    // atm just pressing o won't make you free cause still in the detectionSensor
                }
            }
        }
    }
}

/// Give velocity x and y value to move forward a certain target
fn move_to(target_transform: &GlobalTransform, transform: &Transform, speed: &Speed) -> (f32, f32) {
    let up = target_transform.translation().y > transform.translation.y;
    let down = target_transform.translation().y < transform.translation.y;
    let left = target_transform.translation().x < transform.translation.x;
    let right = target_transform.translation().x > transform.translation.x;

    let x_axis = -(left as i8) + right as i8;
    let y_axis = -(down as i8) + up as i8;

    // println!("x: {}, y: {}", x_axis, y_axis);

    let mut vel_x = x_axis as f32 * **speed;
    let mut vel_y = y_axis as f32 * **speed;

    if x_axis != 0 && y_axis != 0 {
        vel_x *= (std::f32::consts::PI / 4.).cos();
        vel_y *= (std::f32::consts::PI / 4.).cos();
    }

    return (vel_x, vel_y);
}

/// Give velocity x and y value to move forward a certain vec3
fn move_to_dest(target_vec3: Vec3, transform: &Transform, speed: &Speed) -> (f32, f32) {
    let up = target_vec3.y > transform.translation.y;
    let down = target_vec3.y < transform.translation.y;
    let left = target_vec3.x < transform.translation.x;
    let right = target_vec3.x > transform.translation.x;

    let x_axis = -(left as i8) + right as i8;
    let y_axis = -(down as i8) + up as i8;

    // println!("x: {}, y: {}", x_axis, y_axis);

    let mut vel_x = x_axis as f32 * **speed;
    let mut vel_y = y_axis as f32 * **speed;

    if x_axis != 0 && y_axis != 0 {
        vel_x *= (std::f32::consts::PI / 4.).cos();
        vel_y *= (std::f32::consts::PI / 4.).cos();
    }

    return (vel_x, vel_y);
}

/// # Parameters
///
/// position: of a entity
/// direction: the middle of the future zone,
///            is on the middle of the segment [a,c]
///
/// # Return
/// returns true if the entity is on the square around the direction point
///
/// # Note
///
/// XXX: Rework this Approximation Louche
fn close(position: Vec3, direction: Vec3, range: f32) -> bool {
    // direction.x == position.x &&
    // direction.y == position.y

    let a = Vec3::new(direction.x - range, direction.y + range, direction.z);

    let c = Vec3::new(direction.x + range, direction.y - range, direction.z);

    position.x >= a.x && position.x <= c.x && position.y <= a.y && position.y >= c.y
}

/**
 * param:
 *  force
 *  range: cuboid ? no ball
 * return:
 *  Vec3
 */
pub fn give_a_direction() -> Vec3 {
    let x =
        rand::thread_rng().gen_range(-100 * (TILE_SIZE as i32)..100 * (TILE_SIZE as i32)) as f32;
    let y =
        rand::thread_rng().gen_range(-100 * (TILE_SIZE as i32)..100 * (TILE_SIZE as i32)) as f32;
    // let z = rand::thread_rng().gen_range(1..101);

    /* shape ideas
     * (x, y) -> A
     * (x+1, y-1) -> C
     * (x+0.5, y-0.5) -> milieu
     */

    let direction: Vec3 = Vec3::new(x, y, 0.);

    direction
}

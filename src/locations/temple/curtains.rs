use super::ZPosition;
use crate::{
    animations::sprite_sheet_animation::{AnimationDuration, SpriteSheetAnimation},
    collisions::IsColliding,
    constants::{
        locations::temple::*,
        player::{PLAYER_HITBOX_WIDTH, PLAYER_HITBOX_Y_OFFSET},
    },
    player::{Player, PlayerSensor},
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct CurtainsTimer(Timer);
#[derive(Component)]
pub struct Curtain;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum PlayerCurtainsPosition {
    Above,
    Below,
}

pub fn curtains_animation(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut curtains_query: Query<(Entity, &Transform, &mut TextureAtlasSprite), With<Curtain>>,
    mut curtains_state: ResMut<State<PlayerCurtainsPosition>>,
    player_query: Query<(&GlobalTransform, &Children), With<Player>>,
    rapier_context: Res<RapierContext>,
) {
    let (player_transform, children) = player_query.single();
    for (e_c, ..) in curtains_query.iter() {
        info!("{:?}", rapier_context.intersection_pair(children[1], e_c));
        // for c in rapier_context.intersections_with(e_c) {
        //     info!("{children:?}");
        //     info!("{c:?}");
        // }
        // info!("{}", e_c.is_intersecting_with(children[0], &rapier_context));
    }
    for collision_event in collision_events.iter() {
        for (curtain_entity, curtain_transform, mut sprite) in curtains_query.iter_mut() {
            let (start, end) = if player_transform.translation.x > curtain_transform.translation.x {
                (0, 4)
            } else {
                (5, 9)
            };
            let curtains_sensor_y = curtain_transform.translation.y + CURTAINS_SENSOR_Y_OFFSET;

            match collision_event {
                CollisionEvent::Started(e1, e2, _)
                    if *e1 == curtain_entity || *e2 == curtain_entity =>
                {
                    if curtains_state.current() == &PlayerCurtainsPosition::Below {
                        curtains_state.set(PlayerCurtainsPosition::Above).unwrap();
                        sprite.index = start;

                        spawn_z_timer(&mut commands, CURTAINS_Z_BACK);
                        insert_curtain_animation(&mut commands, curtain_entity, start, end);
                    } else if curtains_state.current() == &PlayerCurtainsPosition::Above {
                        curtains_state.set(PlayerCurtainsPosition::Below).unwrap();
                        sprite.index = start;

                        spawn_z_timer(&mut commands, CURTAINS_Z_FRONT);
                        insert_curtain_animation(&mut commands, curtain_entity, start, end);
                    }
                }
                CollisionEvent::Stopped(e1, e2, _)
                    if *e1 == curtain_entity || *e2 == curtain_entity =>
                {
                    if player_transform.translation.y
                        + PLAYER_HITBOX_Y_OFFSET
                        + PLAYER_HITBOX_WIDTH / 2.0
                        > curtains_sensor_y
                        && curtains_state.current() == &PlayerCurtainsPosition::Above
                    {
                        curtains_state.set(PlayerCurtainsPosition::Below).unwrap();
                        sprite.index = start;

                        spawn_z_timer(&mut commands, CURTAINS_Z_FRONT);
                        insert_curtain_animation(&mut commands, curtain_entity, start, end);
                    } else if player_transform.translation.y
                        + PLAYER_HITBOX_Y_OFFSET
                        + PLAYER_HITBOX_WIDTH / 2.0
                        < curtains_sensor_y
                        && curtains_state.current() == &PlayerCurtainsPosition::Below
                    {
                        curtains_state.set(PlayerCurtainsPosition::Above).unwrap();
                        sprite.index = start;

                        spawn_z_timer(&mut commands, CURTAINS_Z_BACK);
                        insert_curtain_animation(&mut commands, curtain_entity, start, end);
                    }
                }
                _ => {}
            }
        }
    }
}

// Changes the Z position of the curtains after the player passes through them
pub fn curtains_z_position(
    mut commands: Commands,
    time: Res<Time>,
    mut timer_query: Query<(Entity, &mut CurtainsTimer, &ZPosition)>,
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

fn spawn_z_timer(commands: &mut Commands, z: f32) {
    commands
        .spawn()
        .insert(CurtainsTimer(Timer::from_seconds(
            CURTAINS_CHANGE_Z_TIME,
            false,
        )))
        .insert(ZPosition(z));
}

fn insert_curtain_animation(commands: &mut Commands, entity: Entity, start: usize, end: usize) {
    commands.entity(entity).insert(SpriteSheetAnimation {
        start_index: start,
        end_index: end,
        timer: Timer::from_seconds(CURTAINS_ANIMATION_DELTA, true),
        duration: AnimationDuration::Once,
    });
}

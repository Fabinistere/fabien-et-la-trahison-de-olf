use super::ZPosition;
use crate::{
    animations::sprite_sheet_animation::{AnimationDuration, SpriteSheetAnimation},
    constants::{
        locations::temple::curtains::*,
        player::{PLAYER_SCALE, PLAYER_WIDTH},
    },
    player::Player,
};
use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct CurtainsTimer(Timer);
#[derive(Component)]
pub struct Curtain;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum PlayerCurtainsPosition {
    Above,
    Below,
}

pub fn setup_curtains(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let curtains_spritesheet = asset_server.load("textures/temple/curtains_sprite_sheet.png");
    let curtains_texture_atlas = TextureAtlas::from_grid(
        curtains_spritesheet,
        Vec2::new(200.0, 360.0),
        1,
        10,
        None,
        None,
    );
    let curtains_texture_atlas_handle = texture_atlases.add(curtains_texture_atlas);

    // Left curtain, with a sensor collider to detect when the player passes through it
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: curtains_texture_atlas_handle.clone(),
            transform: Transform::from_translation(LEFT_CURTAIN_POSITION.into()),
            ..SpriteSheetBundle::default()
        },
        Curtain,
    ));

    // Right curtain
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: curtains_texture_atlas_handle,
            transform: Transform::from_translation(RIGHT_CURTAIN_POSITION.into()),
            ..SpriteSheetBundle::default()
        },
        Curtain,
    ));
}

pub fn curtains_animation(
    mut commands: Commands,
    mut curtains_query: Query<(Entity, &Transform, &mut TextureAtlasSprite), With<Curtain>>,
    mut curtains_state: ResMut<State<PlayerCurtainsPosition>>,
    player_query: Query<&GlobalTransform, With<Player>>,
) {
    let player_transform = player_query.single();

    for (curtain_entity, curtain_transform, mut sprite) in curtains_query.iter_mut() {
        let half_player_width = (PLAYER_WIDTH * PLAYER_SCALE) / 2.0;
        let in_range_left =
            curtain_transform.translation.x < player_transform.translation().x + half_player_width;
        let in_range_right =
            curtain_transform.translation.x > player_transform.translation().x - half_player_width;

        let (anim_start, anim_end) =
            if player_transform.translation().x > curtain_transform.translation.x {
                (0, 4)
            } else {
                (5, 9)
            };

        if player_transform.translation().y >= CURTAINS_TRIGGER_Y
            && curtains_state.current() == &PlayerCurtainsPosition::Below
        {
            curtains_state
                .overwrite_set(PlayerCurtainsPosition::Above)
                .unwrap();
            spawn_z_timer(&mut commands, CURTAINS_Z_FRONT);

            if in_range_left && in_range_right {
                sprite.index = anim_start;

                insert_curtain_animation(&mut commands, curtain_entity, anim_start, anim_end);
            }
        } else if player_transform.translation().y < CURTAINS_TRIGGER_Y
            && curtains_state.current() == &PlayerCurtainsPosition::Above
        {
            curtains_state
                .overwrite_set(PlayerCurtainsPosition::Below)
                .unwrap();
            spawn_z_timer(&mut commands, CURTAINS_Z_BACK);

            if in_range_left && in_range_right {
                sprite.index = anim_start;

                insert_curtain_animation(&mut commands, curtain_entity, anim_start, anim_end);
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
    commands.spawn((
        CurtainsTimer(Timer::from_seconds(CURTAINS_CHANGE_Z_TIME, TimerMode::Once)),
        ZPosition(z),
    ));
}

fn insert_curtain_animation(commands: &mut Commands, entity: Entity, start: usize, end: usize) {
    commands.entity(entity).insert(SpriteSheetAnimation {
        start_index: start,
        end_index: end,
        timer: Timer::from_seconds(CURTAINS_ANIMATION_DELTA, TimerMode::Repeating),
        duration: AnimationDuration::Once,
    });
}

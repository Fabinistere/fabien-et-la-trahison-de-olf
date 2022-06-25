use crate::{constants::locations::temple::first_corridor::*, interactions::Interactible};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Door;
#[derive(Component)]
pub struct DoorInteract {
    timer: Timer,
    opening: bool,
}

pub fn setup_first_corridor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let first_corridor = asset_server.load("textures/temple/first_corridor/first_corridor.png");
    let props = asset_server.load("textures/temple/first_corridor/props.png");
    let door_spritesheet = asset_server.load("textures/temple/first_corridor/door_spritesheet.png");
    let door_texture_atlas =
        TextureAtlas::from_grid(door_spritesheet, Vec2::new(120.0, 99.0), 1, 6);

    commands.spawn_bundle(SpriteBundle {
        texture: first_corridor,
        // transform: Transform::from_scale(Vec3::new(2.0, 2.0, 2.0)),
        transform: Transform::from_xyz(0.0, 0.0, FIRST_CORRIDOR_Z),
        ..SpriteBundle::default()
    });

    commands
        .spawn_bundle(SpriteBundle {
            texture: props,
            transform: Transform::from_xyz(0.0, 0.0, 2.5),
            ..SpriteBundle::default()
        })
        .with_children(|parent| {
            parent
                .spawn()
                .insert(Collider::cuboid(100.0, 105.0))
                .insert(Transform::from_xyz(-1210.0, -1430.0, 0.0));
        })
        .insert(Interactible {
            range: 100.0,
            icon_position: Vec3::new(0.0, 0.0, 0.0),
            interaction_id: 0,
        });

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlases.add(door_texture_atlas),
            transform: Transform::from_translation(DOOR_POSITION.into()),
            ..SpriteSheetBundle::default()
        })
        .insert(Door)
        .insert(DoorInteract {
            timer: Timer::from_seconds(1.0, true),
            opening: true,
        })
        .insert(Interactible {
            range: 50.0,
            icon_position: Vec3::new(0.0, 0.0, 0.0),
            interaction_id: 0,
        })
        .with_children(|parent| {
            parent
                .spawn()
                .insert(Collider::cuboid(30.0, 50.0))
                .insert(Transform::from_xyz(
                    DOOR_POSITION.0 - 30.0,
                    DOOR_POSITION.1,
                    0.0,
                ));

            parent
                .spawn()
                .insert(Collider::cuboid(30.0, 50.0))
                .insert(Transform::from_xyz(
                    DOOR_POSITION.0 + 30.0,
                    DOOR_POSITION.1,
                    0.0,
                ));
        });
}

pub fn open_door(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut commands: Commands,
    mut door_query: Query<(
        Entity,
        &mut DoorInteract,
        &Children,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
    mut colliders_query: Query<&mut Transform, With<Collider>>,
) {
    for (entity, mut door_interaction, children, mut sprite, texture_atlas_handle) in
        door_query.iter_mut()
    {
        door_interaction.timer.tick(time.delta());

        if door_interaction.timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();

            if door_interaction.opening {
                sprite.index += 1;
                colliders_query.get_mut(children[0]).unwrap().translation.x -= 10.0;
                colliders_query.get_mut(children[1]).unwrap().translation.x += 10.0;

                if sprite.index >= texture_atlas.len() - 1 {
                    commands.entity(entity).remove::<DoorInteract>();
                }
            } else {
                sprite.index -= 1;
                colliders_query.get_mut(children[0]).unwrap().translation.x += 10.0;
                colliders_query.get_mut(children[1]).unwrap().translation.x -= 10.0;

                if sprite.index == 0 {
                    commands.entity(entity).remove::<DoorInteract>();
                }
            }
        }
    }
}

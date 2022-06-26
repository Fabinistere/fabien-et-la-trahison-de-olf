use crate::{
    constants::{interactions::INTERACT_BUTTON_Z, locations::temple::first_corridor::*},
    interactions::Interactible,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Door {
    open: bool,
}

#[derive(Component)]
pub struct DoorInteract {
    timer: Timer,
    opening: bool,
}

pub struct DoorInteractEvent;

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
                .insert(Collider::cuboid(20.0, 105.0))
                .insert(Transform::from_xyz(20.0, 0.0, 0.0))
                .insert(Sensor(true));

            parent
                .spawn()
                .insert(Collider::cuboid(100.0, 105.0))
                .insert(Transform::from_translation(PROPS_POSITION.into()));
        })
        .insert(Interactible {
            icon_transform: Transform::from_xyz(0.0, 0.0, INTERACT_BUTTON_Z),
            interaction_id: PROPS_INTERACTION_ID,
        });

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlases.add(door_texture_atlas),
            transform: Transform::from_translation(DOOR_POSITION.into()),
            ..SpriteSheetBundle::default()
        })
        .insert(Door { open: false })
        .insert(DoorInteract {
            opening: true,
            timer: Timer::from_seconds(DOOR_OPEN_DELTA_S, true),
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

/*
pub fn door_interact(
    mut commands: Commands,
    mut door_interact_events: EventReader<DoorInteractEvent>,
    mut door_query: Query<(Entity, &Door, Option<&mut DoorInteract>)>,
) {
    for DoorInteractEvent in door_interact_events.iter() {
        let (entity, door, door_interact) = door_query.single_mut();

        if let Some(mut door_interact) = door_interact {
            door_interact.opening = !door_interact.opening;
        } else {
            commands.entity(entity).insert(DoorInteract {
                opening: !door.open,
                timer: Timer::from_seconds(DOOR_OPEN_DELTA_S, true),
            });
        }
    }
}
*/

pub fn open_close_door(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut commands: Commands,
    mut door_query: Query<(
        Entity,
        &mut Door,
        &mut DoorInteract,
        &Children,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
    mut colliders_query: Query<&mut Transform, With<Collider>>,
) {
    for (entity, mut door, mut door_interaction, children, mut sprite, texture_atlas_handle) in
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
                    door.open = true;
                }
            } else {
                sprite.index -= 1;
                colliders_query.get_mut(children[0]).unwrap().translation.x += 10.0;
                colliders_query.get_mut(children[1]).unwrap().translation.x -= 10.0;

                if sprite.index == 0 {
                    commands.entity(entity).remove::<DoorInteract>();
                    door.open = false;
                }
            }
        }
    }
}

use crate::{
    constants::{interactions::INTERACT_BUTTON_Z, locations::temple::second_corridor::*},
    interactions::Interactible,
    locations::spawn_collision_cuboid,
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

#[derive(Component)]
pub struct SecondCorridor;

#[derive(Event)]
pub struct DoorInteractEvent;

pub fn setup_second_corridor(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let second_corridor = asset_server.load("textures/temple/second_corridor/second_corridor.png");
    let door_spritesheet =
        asset_server.load("textures/temple/second_corridor/door_spritesheet.png");
    let door_texture_atlas =
        TextureAtlas::from_grid(door_spritesheet, Vec2::new(200.0, 300.0), 1, 8, None, None);

    // Left side of bottom wall
    spawn_collision_cuboid(&mut commands, -380.0, -1185.0, 930.0, 140.0);
    // Right side of bottom wall
    spawn_collision_cuboid(&mut commands, 760.0, -1185.0, 90.0, 140.0);

    commands.spawn((
        SpriteBundle {
            texture: second_corridor,
            transform: Transform::from_xyz(0.0, 0.0, SECOND_CORRIDOR_Z),
            ..SpriteBundle::default()
        },
        SecondCorridor,
    ));

    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlases.add(door_texture_atlas),
                transform: Transform::from_translation(DOOR_POSITION.into()),
                ..SpriteSheetBundle::default()
            },
            Door { open: false },
            Interactible::new(
                Vec3::new(-80.0, 10.0, INTERACT_BUTTON_Z),
                DOOR_INTERACTION_ID,
            ),
        ))
        .with_children(|parent| {
            parent.spawn((
                Collider::cuboid(120.0, 50.0),
                Transform::from_xyz(DOOR_POSITION.0, DOOR_POSITION.1 - 160.0, 0.0),
                Sensor,
            ));

            parent.spawn((
                Collider::cuboid(70.0, 20.0),
                Transform::from_xyz(DOOR_POSITION.0, DOOR_POSITION.1 - 130.0, 0.0),
            ));
        });
}

pub fn door_interact(
    mut commands: Commands,
    mut door_interact_events: EventReader<DoorInteractEvent>,
    mut door_query: Query<(Entity, &Door, Option<&mut DoorInteract>, &Children)>,
) {
    for DoorInteractEvent in door_interact_events.iter() {
        let (entity, door, door_interact, children) = door_query.single_mut();

        if let Some(mut door_interact) = door_interact {
            door_interact.opening = !door_interact.opening;
        } else {
            if door.open {
                commands.entity(children[1]).insert(Sensor);
            }

            commands.entity(entity).insert(DoorInteract {
                opening: !door.open,
                timer: Timer::from_seconds(DOOR_OPEN_DELTA_S, TimerMode::Repeating),
            });
        }
    }
}

pub fn open_close_door(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut commands: Commands,
    mut door_query: Query<(
        Entity,
        &mut Door,
        &mut DoorInteract,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
        &Children,
    )>,
) {
    for (entity, mut door, mut door_interaction, mut sprite, texture_atlas_handle, children) in
        door_query.iter_mut()
    {
        door_interaction.timer.tick(time.delta());

        if door_interaction.timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();

            if door_interaction.opening {
                sprite.index += 1;

                if sprite.index >= texture_atlas.len() - 1 {
                    commands.entity(entity).remove::<DoorInteract>();
                    commands.entity(children[1]).insert(Sensor);
                    door.open = true;
                }
            } else {
                sprite.index -= 1;

                if sprite.index == 0 {
                    commands.entity(entity).remove::<DoorInteract>();
                    door.open = false;
                }
            }
        }
    }
}

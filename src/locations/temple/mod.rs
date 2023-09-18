use bevy::prelude::*;
use bevy_rapier2d::prelude::{CollisionEvent, Sensor};

use crate::{
    characters::{player::Player, CharacterHitbox},
    constants::locations::{
        hall::{TEMPLE_DOOR_SWITCH_Z_OFFSET_CLOSED, TEMPLE_DOOR_SWITCH_Z_OFFSET_OPENED},
        *,
    },
    playing, GameState,
};

use self::hall::TempleDoor;

pub mod hall;
pub mod main_room;
pub mod secret_room;

#[derive(Component, Deref, DerefMut)]
pub struct ZPosition(f32);

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default, Reflect, Component, States)]
pub enum Location {
    #[default]
    Hall,
    Temple,
    SecretRoom,
}

pub struct TemplePlugin;

impl Plugin for TemplePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<Location>()
            .add_event::<main_room::SecretBannerEvent>()
            .add_event::<hall::PropsInteractionEvent>()
            .add_event::<secret_room::SecretRoomTriggerEvent>()
            .add_event::<secret_room::RemoveSecretRoomCoverEvent>()
            .add_event::<secret_room::AddSecretRoomCoverEvent>()
            .add_event::<DoorInteractEvent>()
            .add_systems(
                OnEnter(GameState::Playing),
                (
                    hall::setup_hall,
                    main_room::setup_main_room,
                    secret_room::setup_secret_room,
                ),
            )
            .add_systems(
                PostUpdate,
                (
                    chandeliers_opacity,
                    y_to_z_conversion,
                    location_event,
                    door_interact,
                    open_close_door,
                    secret_room::second_layer_fake_wall_visibility,
                    secret_room::remove_secret_room_cover,
                    secret_room::add_secret_room_cover,
                )
                    .run_if(playing),
            )
            .add_systems(
                PostUpdate,
                (hall::props_interaction_event, hall::remove_balcony_cover)
                    .distributive_run_if(in_hall),
            )
            .add_systems(
                PostUpdate,
                main_room::secret_banner_interaction.run_if(in_temple_or_secret_room),
            )
            .add_systems(
                OnEnter(Location::Hall),
                control_wall_collider.run_if(playing),
            )
            .add_systems(
                OnEnter(Location::Temple),
                control_wall_collider.run_if(playing),
            )
            .add_systems(
                OnEnter(Location::SecretRoom),
                control_wall_collider.run_if(playing),
            );
    }
}

/* -------------------------------------------------------------------------- */
/*                               Run If Systems                               */
/* -------------------------------------------------------------------------- */

fn in_hall(location: Res<State<Location>>, game_state: Res<State<GameState>>) -> bool {
    location.get() == &Location::Hall && game_state.get() == &GameState::Playing
}

fn _in_temple(location: Res<State<Location>>, game_state: Res<State<GameState>>) -> bool {
    location.get() == &Location::Temple && game_state.get() == &GameState::Playing
}

fn _in_secret_room(location: Res<State<Location>>, game_state: Res<State<GameState>>) -> bool {
    location.get() == &Location::SecretRoom && game_state.get() == &GameState::Playing
}

fn in_temple_or_secret_room(
    location: Res<State<Location>>,
    game_state: Res<State<GameState>>,
) -> bool {
    (location.get() == &Location::SecretRoom || location.get() == &Location::Temple)
        && game_state.get() == &GameState::Playing
}

/* -------------------------------------------------------------------------- */
/*                                 Global ECS                                 */
/* -------------------------------------------------------------------------- */

#[derive(Deref, DerefMut, Reflect, Default, Component)]
pub struct OverlappingEntity {
    pub z_offset: f32,
}

impl OverlappingEntity {
    pub fn new(z_offset: f32) -> Self {
        OverlappingEntity { z_offset }
    }
}

#[derive(Component)]
pub struct Chandelier;

#[derive(Component)]
pub struct Flame;

#[derive(Component)]
pub struct WallCollider(pub Location);

#[derive(Component)]
pub struct LocationSensor {
    pub location: Location,
}

/// TODO: make it work
pub fn chandeliers_opacity(
    mut chandeliers_query: Query<(&mut Sprite, &Transform), With<Chandelier>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let player_transform = player_query.single();
    // OPTIMIZE: we could put the location in the Chandelier struct
    // to first compare if we have to check the position
    for (mut sprite, chandelier_transform) in chandeliers_query.iter_mut() {
        sprite.color.set_a(
            if chandelier_transform.translation.x - CHANDELIER_SIZE.0 / 2.
                >= player_transform.translation.x
                && player_transform.translation.x
                    >= chandelier_transform.translation.x + CHANDELIER_SIZE.0 / 2.
                && chandelier_transform.translation.y - CHANDELIER_SIZE.1 / 2.
                    >= player_transform.translation.y
                && player_transform.translation.y
                    >= chandelier_transform.translation.y + CHANDELIER_SIZE.1 / 2.
            {
                CHANDELIER_TRANSPARENCY_COLOR
            } else {
                CHANDELIER_PLAIN_COLOR
            },
        );
    }
}

/// The more y you have the less z you will have.
/// The more you go up, the more you will be below things, in the farground.
///
/// # Map Elements
///
/// We encapsulate all props/objects in the parent room
/// It herits its parent's transform.
/// Or exclude Map elements from this system.
pub fn y_to_z_conversion(
    mut small_entity_query: Query<
        (&mut Transform, &OverlappingEntity, Option<&Parent>),
        Or<(Changed<Transform>, Changed<OverlappingEntity>)>,
    >,
    transform_query: Query<&Transform, Without<OverlappingEntity>>,
) {
    for (mut transform, overlapping, potential_parent) in &mut small_entity_query {
        let parent_z = match potential_parent {
            None => 0.,
            Some(parent) => transform_query.get(**parent).unwrap().translation.z,
        };
        transform.translation.z =
            (transform.translation.y - MAP_START_Y) * Y_UNIT - MAP_DISTANCE_IN_Z - parent_z
                + overlapping.z_offset;
    }
}

/// Manage where characters are
pub fn location_event(
    mut collision_events: EventReader<CollisionEvent>,

    location_sensor_query: Query<(Entity, &LocationSensor)>,
    character_hitbox_query: Query<(Entity, &Parent), With<CharacterHitbox>>,

    location: Res<State<Location>>,
    mut next_location: ResMut<NextState<Location>>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                match (
                    character_hitbox_query.get(*e1),
                    character_hitbox_query.get(*e2),
                    location_sensor_query.get(*e1),
                    location_sensor_query.get(*e2),
                ) {
                    (
                        Ok((character_hitbox, _character)),
                        Err(_),
                        Err(_),
                        Ok((location_sensor, location_point)),
                    )
                    | (
                        Err(_),
                        Ok((character_hitbox, _character)),
                        Ok((location_sensor, location_point)),
                        Err(_),
                    ) => {
                        if (*e1 == location_sensor && *e2 == character_hitbox)
                            || (*e1 == character_hitbox && *e2 == location_sensor)
                        {
                            if location.get() != &location_point.location {
                                next_location.set(location_point.location.clone());
                            }
                            break;
                        }
                    }
                    _ => continue,
                }
            }
            _ => continue,
        }
    }
}

/// NOTE: Instead of OnEnter(...), just check if the state has changed
fn control_wall_collider(
    mut commands: Commands,
    player_location: Res<State<Location>>,
    wall_colliders_query: Query<(Entity, &WallCollider)>,
) {
    let current_location = player_location.get();
    for (collider, WallCollider(collider_location)) in &wall_colliders_query {
        if current_location == collider_location {
            commands.entity(collider).remove::<Sensor>();
        } else {
            commands.entity(collider).insert(Sensor);
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                               Door Animation                               */
/* -------------------------------------------------------------------------- */

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Component)]
pub enum DoorState {
    Opened,
    Closed,
    // /// Stands for Opening or Closing state
    // Shifting,
    Opening,
    Closing,
}

#[derive(Component)]
pub struct DoorInteract {
    pub timer: Timer,
}

#[derive(Event)]
pub struct DoorInteractEvent {
    pub door_entity: Entity,
    pub open_delta_s: f32,
}

#[derive(Component)]
pub struct DoorColliderClosed;

#[derive(Component)]
pub struct DoorColliderOpened;

pub fn door_interact(
    mut commands: Commands,
    mut door_interact_events: EventReader<DoorInteractEvent>,
    mut doors_query: Query<(Entity, &mut DoorState, Option<&mut DoorInteract>)>,
) {
    for DoorInteractEvent {
        door_entity,
        open_delta_s,
    } in door_interact_events.iter()
    {
        let (entity, mut door_state, door_interact) = doors_query.get_mut(*door_entity).unwrap();

        match door_interact {
            Some(_) => {
                if *door_state == DoorState::Opening {
                    *door_state = DoorState::Closing;
                } else {
                    *door_state = DoorState::Opening;
                }
            }
            None => {
                if *door_state == DoorState::Opened {
                    *door_state = DoorState::Closing;
                } else {
                    *door_state = DoorState::Opening;
                }

                commands.entity(entity).insert(DoorInteract {
                    timer: Timer::from_seconds(*open_delta_s, TimerMode::Repeating),
                });
            }
        }
    }
}

pub fn open_close_door(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut commands: Commands,
    mut doors_query: Query<(
        Entity,
        &mut DoorState,
        &mut DoorInteract,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
        &Children,
    )>,
    door_collider_closed_query: Query<Entity, With<DoorColliderClosed>>,

    mut temple_door_query: Query<&mut OverlappingEntity, With<TempleDoor>>,
    door_collider_opened_query: Query<Entity, With<DoorColliderOpened>>,
) {
    for (
        entity,
        mut door_state,
        mut door_interaction,
        mut sprite,
        texture_atlas_handle,
        children,
    ) in doors_query.iter_mut()
    {
        door_interaction.timer.tick(time.delta());

        if door_interaction.timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();

            if *door_state == DoorState::Opening {
                sprite.index += 1;

                if sprite.index >= texture_atlas.len() - 1 {
                    commands.entity(entity).remove::<DoorInteract>();
                    match door_collider_closed_query.get(children[1]) {
                        Err(e) => warn!("{}", e),
                        Ok(collider) => {
                            commands.entity(collider).insert(Sensor);
                        }
                    }
                    *door_state = DoorState::Opened;

                    match temple_door_query.get_mut(entity) {
                        Err(_) => {}
                        Ok(mut ovelapping_setting) => {
                            match door_collider_opened_query.get_many([children[2], children[3]]) {
                                Err(e) => warn!("{}", e),
                                Ok([collider_1, collider_2]) => {
                                    commands.entity(collider_1).remove::<Sensor>();
                                    commands.entity(collider_2).remove::<Sensor>();
                                }
                            }
                            ovelapping_setting.z_offset = TEMPLE_DOOR_SWITCH_Z_OFFSET_OPENED;
                        }
                    }
                }
            } else if *door_state == DoorState::Closing {
                sprite.index -= 1;

                if sprite.index == 0 {
                    commands.entity(entity).remove::<DoorInteract>();
                    match door_collider_closed_query.get(children[1]) {
                        Err(e) => warn!("{}", e),
                        Ok(collider) => {
                            commands.entity(collider).remove::<Sensor>();
                        }
                    }
                    *door_state = DoorState::Closed;

                    match temple_door_query.get_mut(entity) {
                        Err(_) => {}
                        Ok(mut ovelapping_setting) => {
                            match door_collider_opened_query.get_many([children[2], children[3]]) {
                                Err(e) => warn!("{}", e),
                                Ok([collider_1, collider_2]) => {
                                    commands.entity(collider_1).insert(Sensor);
                                    commands.entity(collider_2).insert(Sensor);
                                }
                            }
                            ovelapping_setting.z_offset = TEMPLE_DOOR_SWITCH_Z_OFFSET_CLOSED;
                        }
                    }
                }
            }
        }
    }
}

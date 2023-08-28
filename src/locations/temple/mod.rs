use crate::{
    characters::{player::Player, CharacterHitbox},
    constants::{
        character::player::PLAYER_Z,
        locations::{
            hall::{HALL_Z, HALL_Z_IN_MAIN_ROOM},
            main_room::{MAIN_ROOM_Z, MAIN_ROOM_Z_WHEN_IN_SECRET_ROOM},
            *,
        },
    },
    GameState,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::{CollisionEvent, Sensor};

use self::{
    hall::{DoorCollider, Hall},
    main_room::Temple,
};

pub mod hall;
pub mod main_room;
pub mod secret_room;

#[derive(Component, Deref, DerefMut)]
pub struct ZPosition(f32);

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, Reflect, States)]
pub enum PlayerLocation {
    #[default]
    Hall,
    Temple,
    SecretRoom,
}

pub struct TemplePlugin;

impl Plugin for TemplePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<PlayerLocation>()
            .add_event::<secret_room::SecretRoomTriggerEvent>()
            .add_event::<secret_room::RemoveSecretRoomCoverEvent>()
            .add_event::<secret_room::AddSecretRoomCoverEvent>()
            .add_event::<DoorInteractEvent>()
            .add_event::<hall::PropsInteractionEvent>()
            .add_event::<main_room::SecretBannerEvent>()
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
                    overlapping_props,
                    location_event,
                    secret_room::remove_secret_room_cover,
                    secret_room::add_secret_room_cover
                ),
            )
            .add_systems(
                PostUpdate,
                (
                    door_interact,
                    open_close_door,
                    main_room::secret_banner_interaction,
                )
            )
            .add_systems(
                PostUpdate,
                (
                    hall::props_interaction_event,
                    hall::remove_balcony_cover,
                )
                    .distributive_run_if(in_hall),
            )
            .add_systems(
                OnEnter(PlayerLocation::Hall),
                control_wall_collider,
            )
            .add_systems(
                OnEnter(PlayerLocation::Temple),
                control_wall_collider,
            )
            .add_systems(
                OnEnter(PlayerLocation::SecretRoom),
                (
                    control_wall_collider,
                    // secret_room::remove_secret_room_cover,
                ),
            )
            // .add_systems(
            //     OnExit(PlayerLocation::SecretRoom),
            //     secret_room::add_secret_room_cover,
            // )
            ;
    }
}

pub fn _in_temple(location: Res<State<PlayerLocation>>, game_state: Res<State<GameState>>) -> bool {
    location.get() == &PlayerLocation::Temple && game_state.get() == &GameState::Playing
}

pub fn in_hall(location: Res<State<PlayerLocation>>, game_state: Res<State<GameState>>) -> bool {
    location.get() == &PlayerLocation::Hall && game_state.get() == &GameState::Playing
}

pub fn _in_secret_room(
    location: Res<State<PlayerLocation>>,
    game_state: Res<State<GameState>>,
) -> bool {
    location.get() == &PlayerLocation::SecretRoom && game_state.get() == &GameState::Playing
}

#[derive(Component)]
pub struct Chandelier;

#[derive(Component)]
pub struct Flame;

#[derive(Component)]
pub struct WallCollider(pub PlayerLocation);

#[derive(Component)]
pub struct LocationSensor {
    pub location: PlayerLocation,
}

/// Sprite which can change between, in front and dehind an entity.
#[derive(Copy, Clone, Reflect, Component)]
pub struct OverlappingProps {
    /// Prop's layer within the room.
    /// Used to mul to the const `PROPS_Z_BACK`,
    /// to order props together.
    ///
    /// # Exaustive table
    ///
    /// | layer   | result |
    /// | ------- | ------ |
    /// | First   | .5     |
    /// | Seconds | .4     |
    /// | Third   | .3     |
    /// | Fourth  | .2     |
    /// | Fifth   | .1     |
    ///
    /// # Example
    ///
    /// If x, layer: `Layer::First`
    /// and y, layer: `Layer::Second`,
    /// y will be behind x.
    pub layer: Layer,
    /// Put the switch limit at the correct height
    pub switch_offset_y: f32,
}

#[derive(Copy, Clone, Reflect)]
pub enum Layer {
    Fifth,
    Fourth,
    Third,
    Second,
    First,
}

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

pub fn overlapping_props(
    player_query: Query<&GlobalTransform, With<Player>>,
    mut overlapping_props_query: Query<(&mut Transform, &OverlappingProps)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (mut props_transform, props_properties) in overlapping_props_query.iter_mut() {
            if player_transform.translation().y + props_properties.switch_offset_y
                > props_transform.translation.y
            {
                props_transform.translation.z =
                    PLAYER_Z + (props_properties.layer as i32) as f32 * PROPS_Z_BACK;
            } else {
                props_transform.translation.z =
                    (props_properties.layer as i32) as f32 * PROPS_Z_BACK;
            }
        }
    }
}

/// Manage where characters are
pub fn location_event(
    mut collision_events: EventReader<CollisionEvent>,

    location_sensor_query: Query<(Entity, &LocationSensor)>,
    character_hitbox_query: Query<(Entity, &Parent), With<CharacterHitbox>>,

    location: Res<State<PlayerLocation>>,
    mut next_location: ResMut<NextState<PlayerLocation>>,
    mut temple_query: Query<&mut Transform, (With<Temple>, Without<Hall>)>,
    mut hall_query: Query<&mut Transform, (With<Hall>, Without<Temple>)>,
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
                                // REFACTOR: maybe but i'm lazy (or famish and sleepy idk)
                                let mut hall_transform = hall_query.single_mut();
                                let mut temple_transform = temple_query.single_mut();
                                match location_point.location {
                                    PlayerLocation::Hall => {
                                        hall_transform.translation.z = HALL_Z;
                                        temple_transform.translation.z = MAIN_ROOM_Z;
                                    }
                                    PlayerLocation::Temple => {
                                        hall_transform.translation.z = HALL_Z_IN_MAIN_ROOM;
                                        temple_transform.translation.z = MAIN_ROOM_Z;
                                    }
                                    PlayerLocation::SecretRoom => {
                                        hall_transform.translation.z = HALL_Z_IN_MAIN_ROOM;
                                        temple_transform.translation.z =
                                            MAIN_ROOM_Z_WHEN_IN_SECRET_ROOM;
                                    }
                                }
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

fn control_wall_collider(
    mut commands: Commands,
    player_location: Res<State<PlayerLocation>>,
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

pub fn door_interact(
    mut commands: Commands,
    mut door_interact_events: EventReader<DoorInteractEvent>,
    mut doors_query: Query<(Entity, &mut DoorState, Option<&mut DoorInteract>, &Children)>,
    door_collider_query: Query<Entity, With<DoorCollider>>,
) {
    for DoorInteractEvent {
        door_entity,
        open_delta_s,
    } in door_interact_events.iter()
    {
        let (entity, mut door_state, door_interact, children) =
            doors_query.get_mut(*door_entity).unwrap();

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
                    match door_collider_query.get(children[1]) {
                        Err(e) => warn!("{}", e),
                        Ok(collider) => {
                            commands.entity(collider).insert(Sensor);
                        }
                    }
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
    door_collider_query: Query<Entity, With<DoorCollider>>,
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
                    match door_collider_query.get(children[1]) {
                        Err(e) => warn!("{}", e),
                        Ok(collider) => {
                            commands.entity(collider).insert(Sensor);
                        }
                    }
                    *door_state = DoorState::Opened;
                }
            } else if *door_state == DoorState::Closing {
                sprite.index -= 1;

                if sprite.index == 0 {
                    commands.entity(entity).remove::<DoorInteract>();
                    match door_collider_query.get(children[1]) {
                        Err(e) => warn!("{}", e),
                        Ok(collider) => {
                            commands.entity(collider).remove::<Sensor>();
                        }
                    }
                    *door_state = DoorState::Closed;
                }
            }
        }
    }
}

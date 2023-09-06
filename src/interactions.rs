use crate::{
    characters::npcs::CharacterInteractionEvent,
    constants::{
        character::npc::{CHARACTER_INTERACT_BUTTON_POSITION, NPC_TALK_INTERACTION_ID},
        interactions::INTERACT_BUTTON_SCALE,
        locations::{
            hall::{BOX_INTERACTION_ID, DOOR_INTERACTION_ID, DOOR_OPEN_DELTA_S},
            main_room::{BANNER_INTERACTION_ID, BANNER_OPEN_DELTA_S},
        },
    },
    controls::KeyBindings,
    locations::temple::{
        hall::{PropsInteractionEvent, TempleDoor},
        main_room::{SecretBanner, SecretBannerEvent},
        DoorInteractEvent, DoorState,
    },
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::CollisionEvent;

pub struct InteractionsPlugin;

impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InteractionIconEvent>()
            .add_systems(Startup, setup_interactions)
            .add_systems(
                Update,
                (interaction_icon_events, interaction_icon, interaction),
            );
    }
}

#[derive(Debug, Event)]
pub struct InteractionIconEvent {
    pub entering_range: bool,
    pub entity: Entity,
}

/// # Constraint
///
/// The first children must be the interaction sensor
/// REFACTOR: foolproof the children sensor obligation (by pointing at it directly)
#[derive(Debug, Default, Component)]
pub struct Interactible {
    pub icon_translation: Vec3,
    pub interaction_id: u32,
    pub in_range: bool,
}

#[derive(Component)]
pub struct InteractionSensor;

#[derive(Component)]
pub struct InteractIcon;

impl Interactible {
    pub fn new(icon_translation: Vec3, interaction_id: u32) -> Self {
        Self {
            icon_translation,
            interaction_id,
            in_range: false,
        }
    }

    pub fn new_npc() -> Self {
        Self {
            icon_translation: CHARACTER_INTERACT_BUTTON_POSITION.into(),
            interaction_id: NPC_TALK_INTERACTION_ID,
            in_range: false,
        }
    }
}

#[derive(Resource)]
pub struct InteractionResources {
    interact_button: Handle<Image>,
}

pub fn setup_interactions(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button = asset_server.load("textures/hud/interact_button.png");
    commands.insert_resource(InteractionResources {
        interact_button: button,
    });
}

/// REFACTOR: to only the character hitbox triggers it
fn interaction_icon_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut interaction_icon_event: EventWriter<InteractionIconEvent>,
    // mut secret_room_trigger_event: EventWriter<SecretRoomTriggerEvent>,
    interactibles_query: Query<(Entity, &Children), With<Interactible>>,
    // secret_room_sensor_query: Query<Entity, With<SecretRoomSensor>>,
    interaction_sensor_query: Query<Entity, With<InteractionSensor>>,
) {
    for collision_event in collision_events.iter() {
        // info!("{:#?}", collision_event);
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                for (entity, children) in interactibles_query.iter() {
                    match interaction_sensor_query.get(children[0]) {
                        Err(e) => error!("hint: The Interactible must have as first children an Interaction Sensor.\n{}",e),
                        Ok(interaction_sensor) => if *e1 == interaction_sensor || *e2 == interaction_sensor {
                            interaction_icon_event.send(InteractionIconEvent {
                                entering_range: true,
                                entity,
                            });
                        },
                    }
                }
            }
            CollisionEvent::Stopped(e1, e2, _) => {
                for (entity, children) in interactibles_query.iter() {
                    match interaction_sensor_query.get(children[0]) {
                        Err(e) => error!("hint: The Interactible must have as first children an Interaction Sensor.\n{}",e),
                        Ok(interaction_sensor) =>
                            if *e1 == interaction_sensor || *e2 == interaction_sensor {
                                interaction_icon_event.send(InteractionIconEvent {
                                    entering_range: false,
                                    entity,
                                });
                            },
                    }
                }
            }
        }
    }
}

pub fn interaction_icon(
    mut commands: Commands,
    mut interaction_icon_events: EventReader<InteractionIconEvent>,
    mut interactibles_query: Query<(&Children, &mut Interactible)>,
    interaction_resources: Res<InteractionResources>,
    interact_icon_query: Query<Entity, With<InteractIcon>>,
) {
    for InteractionIconEvent {
        entering_range,
        entity,
    } in interaction_icon_events.iter()
    {
        let (children, mut interactible) = interactibles_query.get_mut(*entity).unwrap();
        interactible.in_range = *entering_range;

        if *entering_range {
            commands.entity(*entity).with_children(|parent| {
                parent.spawn((
                    SpriteBundle {
                        texture: interaction_resources.interact_button.clone(),
                        transform: Transform {
                            translation: interactible.icon_translation,
                            scale: Vec3::splat(INTERACT_BUTTON_SCALE),
                            ..default()
                        },
                        ..default()
                    },
                    InteractIcon,
                ));
            });
        } else {
            let mut found = false;
            for child in children {
                if let Ok(interact_icon) = interact_icon_query.get(*child) {
                    commands.entity(interact_icon).despawn();
                    found = true;
                    break;
                }
            }
            if !found {
                warn!("There is no Interaction Icon in {:?}", *entity)
            }
        }
    }
}

/// TODO: Only interact with the closest interactible
pub fn interaction(
    key_bindings: Res<KeyBindings>,
    keyboard_input: Res<Input<KeyCode>>,
    interactibles_query: Query<(Entity, &Interactible)>,

    temple_door_query: Query<Entity, With<TempleDoor>>,
    banner_door_query: Query<(Entity, &DoorState), With<SecretBanner>>,

    mut door_interact_event: EventWriter<DoorInteractEvent>,
    mut secret_banner_event: EventWriter<SecretBannerEvent>,
    mut props_interaction_event: EventWriter<PropsInteractionEvent>,
    mut character_interact_event: EventWriter<CharacterInteractionEvent>,
) {
    if keyboard_input.any_just_pressed(key_bindings.interact()) {
        for (entity, interactible) in interactibles_query.iter() {
            if interactible.in_range {
                match interactible.interaction_id {
                    BOX_INTERACTION_ID => {
                        props_interaction_event.send(PropsInteractionEvent);
                    }
                    DOOR_INTERACTION_ID => {
                        let temple_door = temple_door_query.single();
                        door_interact_event.send(DoorInteractEvent {
                            door_entity: temple_door, // entity,
                            open_delta_s: DOOR_OPEN_DELTA_S,
                        });
                    }
                    BANNER_INTERACTION_ID => {
                        let (secret_banner, door_state) = banner_door_query.single();
                        door_interact_event.send(DoorInteractEvent {
                            door_entity: secret_banner, // entity,
                            open_delta_s: BANNER_OPEN_DELTA_S,
                        });
                        secret_banner_event.send(SecretBannerEvent(*door_state));
                    }
                    NPC_TALK_INTERACTION_ID => {
                        character_interact_event.send(CharacterInteractionEvent(entity));
                    }
                    id => error!("Unknown interaction id {id}"),
                }
            }
        }
    }
}

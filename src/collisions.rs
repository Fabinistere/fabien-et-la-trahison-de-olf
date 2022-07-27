use crate::{
    interactions::{Interactible, InteractionIconEvent},
    locations::temple::{
        main_room::{EnterMainRoomEvent, EnterMainRoomSensor},
        secret_room::{SecretRoomSensor, SecretRoomTriggerEvent},
    },
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(collision_events);
    }
}

fn collision_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut interaction_icon_event: EventWriter<InteractionIconEvent>,
    mut secret_room_trigger_event: EventWriter<SecretRoomTriggerEvent>,
    mut enter_main_room_event: EventWriter<EnterMainRoomEvent>,
    interactibles_query: Query<(Entity, &Children), With<Interactible>>,
    secret_room_sensor_query: Query<Entity, With<SecretRoomSensor>>,
    enter_main_room_sensor_query: Query<Entity, With<EnterMainRoomSensor>>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                for (entity, children) in interactibles_query.iter() {
                    if *e1 == children[0] || *e2 == children[0] {
                        interaction_icon_event.send(InteractionIconEvent {
                            entering_range: true,
                            entity,
                        });
                    }
                }

                let secret_room_sensor = secret_room_sensor_query.single();
                let enter_main_room_sensor = enter_main_room_sensor_query.single();

                if *e1 == secret_room_sensor || *e2 == secret_room_sensor {
                    secret_room_trigger_event.send(SecretRoomTriggerEvent { started: true });
                } else if *e1 == enter_main_room_sensor || *e2 == enter_main_room_sensor {
                    enter_main_room_event.send(EnterMainRoomEvent);
                }
            }
            CollisionEvent::Stopped(e1, e2, _) => {
                for (entity, children) in interactibles_query.iter() {
                    if *e1 == children[0] || *e2 == children[0] {
                        interaction_icon_event.send(InteractionIconEvent {
                            entering_range: false,
                            entity,
                        });
                    }
                }

                let secret_room_sensor = secret_room_sensor_query.single();
                let enter_main_room_sensor = enter_main_room_sensor_query.single();

                if *e1 == secret_room_sensor || *e2 == secret_room_sensor {
                    secret_room_trigger_event.send(SecretRoomTriggerEvent { started: false });
                } else if *e1 == enter_main_room_sensor || *e2 == enter_main_room_sensor {
                    enter_main_room_event.send(EnterMainRoomEvent);
                }
            }
        }
    }
}

// fn manage_collision(e1: Entity, e2: Entity, started: bool) {}

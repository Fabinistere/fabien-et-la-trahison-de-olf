use crate::{
    constants::locations::temple::{first_corridor, second_corridor},
    controls::KeyBindings,
    locations::temple::{
        first_corridor::PropsInteractionEvent, second_corridor::DoorInteractEvent,
    },
};
use bevy::prelude::*;

pub struct InteractionsPlugin;

impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InteractionIconEvent>()
            .add_startup_system(setup_interactions)
            .add_systems((interaction_icon, interaction));
    }
}

#[derive(Debug)]
pub struct InteractionIconEvent {
    pub entering_range: bool,
    pub entity: Entity,
}

#[derive(Component, Debug)]
pub struct Interactible {
    pub icon_translation: Vec3,
    pub interaction_id: u32,
    pub in_range: bool,
}

impl Interactible {
    pub fn new(icon_translation: Vec3, interaction_id: u32) -> Self {
        Self {
            icon_translation,
            interaction_id,
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

pub fn interaction_icon(
    mut commands: Commands,
    mut interaction_icon_events: EventReader<InteractionIconEvent>,
    mut interactibles_query: Query<(&Children, &mut Interactible)>,
    interaction_resources: Res<InteractionResources>,
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
                parent.spawn(SpriteBundle {
                    texture: interaction_resources.interact_button.clone(),
                    transform: Transform {
                        translation: interactible.icon_translation,
                        scale: Vec3::new(2.0, 2.0, 1.0),
                        ..Transform::default()
                    },
                    ..SpriteBundle::default()
                });
            });
        } else {
            commands.entity(children[children.len() - 1]).despawn();
        }
    }
}

pub fn interaction(
    key_bindings: Res<KeyBindings>,
    keyboard_input: Res<Input<KeyCode>>,
    interactibles_query: Query<&Interactible>,
    mut door_interact_event: EventWriter<DoorInteractEvent>,
    mut props_interaction_event: EventWriter<PropsInteractionEvent>,
) {
    if keyboard_input.any_just_pressed(key_bindings.interact()) {
        for interactible in interactibles_query.iter() {
            if interactible.in_range {
                match interactible.interaction_id {
                    first_corridor::PROPS_INTERACTION_ID => {
                        props_interaction_event.send(PropsInteractionEvent);
                    }
                    second_corridor::DOOR_INTERACTION_ID => {
                        door_interact_event.send(DoorInteractEvent)
                    }
                    id => error!("Unknown interaction id {id}"),
                }
            }
        }
    }
}

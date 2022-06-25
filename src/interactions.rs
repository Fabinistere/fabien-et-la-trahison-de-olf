use crate::{controls::KeyBindings, player::Player};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct InteractionsPlugin;

impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InteractionEvent>()
            .add_startup_system(setup_interactions)
            .add_system(interact_icon);
    }
}

pub trait InteractionEvent<E: Send + Sync> {
    fn interaction_event() -> E;
}

#[derive(Component)]
pub struct Interactible {
    pub range: f32,
    pub icon_position: Vec3,
    pub event: Box<dyn InteractionEvent>,
}

pub struct InteractionResources {
    interact_button: Handle<Image>,
}

pub fn setup_interactions(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button = asset_server.load("textures/hud/interact_button.png");
    commands.insert_resource(InteractionResources {
        interact_button: button,
    });
}

pub fn interact_icon(
    mut commands: Commands,
    key_bindings: Res<KeyBindings>,
    player_query: Query<&Transform, With<Player>>,
    interactibles_query: Query<(Entity, &Transform, &Interactible)>,
    interaction_resources: Res<InteractionResources>,
) {
    for (entity, transform, interactible) in interactibles_query.iter() {
        let player_transform = player_query.single();

        if player_transform.translation.x < transform.translation.x + interactible.range
            && player_transform.translation.x > transform.translation.x - interactible.range
            && player_transform.translation.y < transform.translation.y + interactible.range
            && player_transform.translation.y > transform.translation.y - interactible.range
        {
            commands.entity(entity).with_children(|parent| {
                parent.spawn_bundle(SpriteBundle {
                    texture: interaction_resources.interact_button.clone(),
                    transform: Transform::from_translation(interactible.icon_position),
                    ..SpriteBundle::default()
                });
            });
        }
    }
}

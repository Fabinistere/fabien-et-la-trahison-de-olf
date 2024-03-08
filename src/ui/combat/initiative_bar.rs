//! Display the Initiative Vertical Bar
//! All Set action and interaction systems

use bevy::prelude::*;

use crate::{
    characters::{FabiensInfos, PersonalInfos},
    combat::{CombatResources, InCombat},
    // constants::character::npcs::FABIEN_STARTING_ANIM,
    ui::combat::combat_panel::ActionDisplayer,
};

use super::combat_panel::Portrait;

#[derive(Component)]
pub struct InitiativeBar;

/// Disables empty action,
/// (invisible == disable).
/// And update the text on the Button and the sprite of it.
///
/// Prevents checking a index in the action list.
pub fn action_visibility(
    combat_resources: Res<CombatResources>,
    mut action_button_query: Query<(&ActionDisplayer, &mut Visibility, &Children), With<Button>>,
    // mut action_sprite_query: Query<&mut TextureAtlasSprite, Without<InCombat>>,
    mut action_image_query: Query<&mut UiImage, (Without<Portrait>, Without<InCombat>)>,
    mut text_query: Query<&mut Text>,
    caster_name_query: Query<(&Name, &TextureAtlasSprite), With<InCombat>>,

    asset_server: Res<AssetServer>,
    fabiens_infos: Res<FabiensInfos>,
) {
    if combat_resources.is_changed() {
        for (action_number, mut visibility, action_children) in action_button_query.iter_mut() {
            // let mut action_sprite = action_sprite_query.get_mut(action_children[1]).unwrap();
            let mut action_image = action_image_query.get_mut(action_children[1]).unwrap();

            let old_visibility = *visibility;

            let mut text = text_query.get_mut(action_children[0]).unwrap();

            *visibility = if action_number.0 < combat_resources.history.len() {
                let (caster_name, _caster_sprite) = caster_name_query
                    .get(combat_resources.history[action_number.0].caster)
                    .unwrap();
                text.sections[0].value = caster_name.to_string();

                // action_sprite.index = caster_sprite.index;
                action_image.texture = if let Some(PersonalInfos {
                    title: _,
                    sprite_path,
                }) = fabiens_infos.get(&caster_name.to_string())
                {
                    // println!("{}", asset_path);
                    asset_server.load(sprite_path)
                } else {
                    warn!(
                        "Action Sprite Asset Not Found/Associated With {}",
                        caster_name
                    );
                    asset_server.load("textures/characters/idle/idle_Fabien_Loyal.png")
                };

                // --- Visibility ---
                Visibility::Inherited
            } else {
                // useless --vv
                text.sections[0].value = "None".to_string();
                // action_sprite.index = FABIEN_STARTING_ANIM;
                // useless --^^
                Visibility::Hidden
            };

            // --- Logs ---
            if old_visibility != *visibility {
                // DEBUG: Actions' Visibility switcher
                // info!(
                //     "action °{} visibility switch: {:?}",
                //     action_number.0, *visibility
                // );
            }
        }
    }
}

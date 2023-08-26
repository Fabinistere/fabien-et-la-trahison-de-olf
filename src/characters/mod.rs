pub mod movement;
pub mod player;

use crate::constants::character::player::{PLAYER_HEIGHT, PLAYER_WIDTH};
use bevy::prelude::*;
use std::collections::HashMap;

pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(player::PlayerPlugin)
            .add_systems(Startup, setup_character_textures);
    }
}

#[derive(Component)]
pub struct CharacterHitbox;

#[derive(Deref, DerefMut, Resource)]
pub struct CharacterTextures(HashMap<Character, CharacterTexture>);

#[derive(Debug)]
pub struct CharacterTexture {
    pub normal: Handle<TextureAtlas>,
    pub icon: Handle<Image>,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Character {
    FabienLInformaticien,
    Olf,
    Panneau,
}

fn setup_character_textures(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let mut character_textures = CharacterTextures(HashMap::new());

    character_textures.insert(
        Character::FabienLInformaticien,
        CharacterTexture {
            normal: texture_atlases.add(TextureAtlas::from_grid(
                asset_server.load("textures/characters/fabien_info_spritesheet.png"),
                Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT),
                4,
                4,
                None,
                None,
            )),
            icon: asset_server.load("textures/characters/fabien_random_icon.png"),
        },
    );

    character_textures.insert(
        Character::Panneau,
        CharacterTexture {
            normal: texture_atlases.add(TextureAtlas::from_grid(
                asset_server.load("textures/characters/panneau_spritesheet.png"),
                Vec2::new(16., 13.),
                1,
                1,
                None,
                None,
            )),
            icon: asset_server.load("textures/characters/panneau_icon.png"),
        },
    );

    commands.insert_resource(character_textures);
}

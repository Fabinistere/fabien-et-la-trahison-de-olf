use crate::constants::player::{PLAYER_HEIGHT, PLAYER_WIDTH};
use bevy::prelude::*;
use std::collections::HashMap;

pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_character_textures);
    }
}

#[derive(Deref, DerefMut)]
pub struct CharacterTextures(HashMap<Character, CharacterTexture>);

#[derive(Debug, Resource)]
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
                Vec2::new(16.0, 13.0),
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

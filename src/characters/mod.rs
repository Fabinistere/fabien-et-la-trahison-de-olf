pub mod movement;
pub mod player;

use bevy::prelude::*;

pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(player::PlayerPlugin);
    }
}

#[derive(Component)]
pub struct CharacterHitbox;

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

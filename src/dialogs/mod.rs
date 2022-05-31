pub mod dialog_box;
pub mod language;

use crate::GameState;
use bevy::prelude::*;
use language::Language;
use serde::Deserialize;
use std::collections::HashMap;
use strum_macros::EnumIter;

pub struct DialogsPlugin;

impl Plugin for DialogsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Dialogs(
            ron::de::from_bytes(include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/data/dialogs.ron"
            )))
            .unwrap(),
        ))
        .init_resource::<Language>()
        .add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(dialog_box::create_dialog_box_on_key_press),
        )
        .add_system(dialog_box::update_dialog_box)
        .add_system(dialog_box::create_dialog_box_on_key_press)
        .add_system(dialog_box::destroy_dialog_box);
    }
}

pub type Dialog = HashMap<Language, String>;

#[derive(Deserialize, EnumIter, Debug, Copy, Clone, Eq, PartialEq, Hash, Component)]
pub enum DialogId {
    MenuTitle,
    MenuTitle01,
    MenuTitle02,
    MenuPlay,
}

#[derive(Deserialize, Debug, Deref, DerefMut)]
pub struct Dialogs(HashMap<DialogId, Dialog>);

impl Dialogs {
    pub fn get(&self, id: DialogId, language: Language) -> String {
        self[&id][&language].clone()
    }
}

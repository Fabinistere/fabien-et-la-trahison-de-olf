pub mod language;
pub mod dialog_box;

use std::collections::HashMap;
use strum_macros::EnumIter;
use serde::Deserialize;
use bevy::prelude::*;
use language::Language;
use crate::GameState;

pub struct DialogsPlugin;

impl Plugin for DialogsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(
            Dialogs(ron::de::from_bytes(include_bytes!(
                concat!(env!("CARGO_MANIFEST_DIR"), "/data/dialogs.ron")
            )).unwrap()))
            .init_resource::<Language>()
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(dialog_box::create_dialog_box_on_key_press.system())
            )
            .add_system(dialog_box::update_dialog_box.system())
            .add_system(dialog_box::create_dialog_box_on_key_press.system())
            .add_system(dialog_box::destroy_dialog_box.system());
    }
}

pub type Dialog = HashMap<Language, String>;

#[derive(Deserialize, EnumIter, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum DialogId {
    MenuTitle,
    MenuTitle01,
    MenuTitle02,
    MenuPlay,
}

#[derive(Deserialize, Debug)]
pub struct Dialogs(HashMap<DialogId, Dialog>);

impl Dialogs {
    pub fn get(&self, id: DialogId, language: Language) -> String {
        // self.0
        //     .get(&id)
        //     .unwrap()
        //     .get(&language)
        //     .unwrap()
        //     .clone()
        self.0[&id][&language].clone()
    }
}

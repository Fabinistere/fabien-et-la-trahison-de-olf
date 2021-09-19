pub mod language;
pub mod dialog_box;

use std::collections::HashMap;
use strum_macros::EnumIter;
use serde::Deserialize;
use bevy::prelude::*;
use language::Language;

pub struct DialogsPlugin;

impl Plugin for DialogsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(
            Dialogs(ron::de::from_bytes(include_bytes!(
                concat!(env!("CARGO_MANIFEST_DIR"), "/data/dialogs.ron")
            )).unwrap()))
            .init_resource::<Language>();
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

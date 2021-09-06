use std::{ fs, collections::HashMap };
use serde::Deserialize;
use bevy::prelude::*;

pub struct DialogsPlugin;

impl Plugin for DialogsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Dialogs>()
            .init_resource::<Language>();
    }
}

#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Language {
    Francais,
    English,
    FabienAncien,
}

impl Default for Language {
    fn default() -> Self {
        Language::FabienAncien
    }
}

pub type Dialog = HashMap<Language, String>;

#[derive(Deserialize, Debug, Eq, PartialEq, Hash)]
pub enum DialogId {
    MenuTitle01,
    MenuTitle02,
    MenuPlay,
}

#[derive(Deserialize, Debug)]
pub struct Dialogs(HashMap<DialogId, Dialog>);

impl Dialogs {
    pub fn get(&self, id: DialogId, language: Language) -> String {
        self.0
            .get(&id)
            .unwrap()
            .get(&language)
            .unwrap()
            .clone()
    }
}

impl Default for Dialogs {
    fn default() -> Self {
        let dialogs_str = fs::read_to_string("assets/dialogs.ron").unwrap();
        Dialogs(ron::from_str(&dialogs_str).unwrap())
    }
}

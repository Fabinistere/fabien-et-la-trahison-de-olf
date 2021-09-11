pub mod dialogs;
mod menu;
mod game;

use bevy::prelude::*;

pub use crate::{
    dialogs::{ Dialogs, DialogId, Language },
};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Menu,
    Playing,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(dialogs::DialogsPlugin)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(game::GamePlugin)
        .add_state(GameState::Playing)
        .run();
}

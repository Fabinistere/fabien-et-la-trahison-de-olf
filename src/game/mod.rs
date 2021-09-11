use bevy::prelude::*;

mod player;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(player::PlayerPlugin);
    }
}

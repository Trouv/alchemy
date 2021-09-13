use bevy::prelude::*;
use witchcraft::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Brewing)
        .add_plugin(alchemy::BrewingPlugin)
        .run();
}

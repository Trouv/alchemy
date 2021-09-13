use witchcraft::*;

use bevy::prelude::*;

fn main() {
    println!("cauldron");
    App::build()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Brewing)
        .add_plugin(alchemy::debug::BrewingPluginDebug)
        .run();
}

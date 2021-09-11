pub mod alchemy;

use bevy::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    Brewing,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Brewing)
        .add_plugin(alchemy::BrewingPlugin)
        .run();
}

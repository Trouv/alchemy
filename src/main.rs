pub mod alchemy;

use alchemy::{brewing::*, compounds::Compound};
use bevy::prelude::*;
use std::convert::TryFrom;
use std::fs;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    Brewing,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(load_reaction_rules())
        .add_state(AppState::Brewing)
        .add_plugin(alchemy::BrewingPlugin)
        .run();
}

fn load_reaction_rules() -> Vec<ReactionRule> {
    let data = std::fs::read_to_string("assets/design/reaction_rules.json").unwrap();
    serde_json::from_str(&data).unwrap()
}

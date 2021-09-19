use crate::AppState;
use bevy::{core::FixedTimestep, prelude::*};

pub mod components;
pub mod compound;
#[cfg(feature = "dev")]
pub mod debug;
mod element;
mod element_counts;
pub mod resources;
pub mod systems;

pub struct BrewingPlugin;

impl Plugin for BrewingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(resources::insert_reaction_rules.system())
            .add_system_set(
                SystemSet::on_update(AppState::Brewing)
                    .with_run_criteria(FixedTimestep::step(0.1))
                    .with_system(systems::brewing.system()),
            );
    }
}

trait AltonWeighable {
    fn weight(&self) -> u32;
}

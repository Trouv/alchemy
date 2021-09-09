use crate::AppState;
use bevy::{core::FixedTimestep, prelude::*};

pub mod brewing;
pub mod compounds;
pub mod transitions;

pub struct BrewingPlugin;

impl Plugin for BrewingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Brewing)
                .with_system(transitions::spawn_test_compounds.system())
                .with_system(transitions::spawn_cauldron.system())
                .with_system(transitions::spawn_rank_display.system())
                .with_system(transitions::spawn_camera.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Brewing)
                .with_run_criteria(FixedTimestep::step(0.1))
                .with_system(brewing::brewing.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Brewing)
                .with_system(brewing::compound_rank_display.system())
                .with_system(brewing::reaction_test_input.system()),
        );
    }
}

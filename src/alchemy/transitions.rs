use crate::alchemy::{brewing::*, compounds::Compound};
use bevy::prelude::*;

pub fn spawn_test_compounds(mut commands: Commands) {
    for _ in 0..20 {
        commands.spawn().insert(Compound::try_from("2abc").unwrap())
    }
    for _ in 0..30 {
        commands.spawn().insert(Compound::try_from("cd").unwrap())
    }
    for _ in 0..30 {
        commands.spawn().insert(Compound::try_from("be").unwrap())
    }
}

pub fn spawn_cauldron(mut commands: Commands) {
    commands
        .spawn()
        .insert(Cauldron)
        .insert(StirMethod::ZeroStir);
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}

pub fn spawn_rank_display(mut commands: Commands) {
    commands
        .spawn_bundle(TextBundle::default())
        .insert(RankDisplayer);
}

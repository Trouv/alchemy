use crate::alchemy::{components::*, compounds::Compound};
use bevy::prelude::*;
use std::str::FromStr;

pub fn spawn_test_compounds(mut commands: Commands) {
    for _ in 0..20 {
        commands.spawn().insert(Compound::from_str("2abc").unwrap());
    }
    for _ in 0..30 {
        commands.spawn().insert(Compound::from_str("cd").unwrap());
    }
    for _ in 0..30 {
        commands.spawn().insert(Compound::from_str("be").unwrap());
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

pub fn spawn_rank_display(mut commands: Commands, assets: Res<AssetServer>) {
    commands
        .spawn_bundle(TextBundle {
            text: Text::with_section(
                "",
                TextStyle {
                    font: assets.load("fonts/FreeMono.otf"),
                    font_size: 30.0,
                    color: Color::BLACK,
                },
                TextAlignment::default(),
            ),

            ..Default::default()
        })
        .insert(RankDisplayer);
}

use crate::{alchemy::BrewingPlugin, AppState};
use bevy::prelude::*;

pub struct BrewingPluginDebug;

impl Plugin for BrewingPluginDebug {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(BrewingPlugin)
            .add_system_set(
                SystemSet::on_enter(AppState::Brewing)
                    .with_system(transitions::spawn_test_compounds.system())
                    .with_system(transitions::spawn_cauldron.system())
                    .with_system(transitions::spawn_rank_display.system())
                    .with_system(transitions::spawn_camera.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::Brewing)
                    .with_system(systems::compound_rank_display.system())
                    .with_system(systems::reaction_test_input.system()),
            );
    }
}

mod transitions {
    use crate::alchemy::{components::*, compound::Compound};
    use bevy::prelude::*;
    use std::str::FromStr;

    pub fn spawn_test_compounds(mut commands: Commands) {
        for _ in 0..20 {
            commands
                .spawn()
                .insert(Compound::<7>::from_str("a3b").unwrap());
        }
        for _ in 0..30 {
            commands
                .spawn()
                .insert(Compound::<7>::from_str("7a").unwrap());
        }
        for _ in 0..30 {
            commands
                .spawn()
                .insert(Compound::<7>::from_str("be").unwrap());
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
}

mod systems {
    use crate::alchemy::{components::*, compound::Compound};
    use bevy::prelude::*;
    use std::{cmp::Ordering, collections::HashMap};

    pub fn compound_rank_display(
        compound_query: Query<&Compound<7>>,
        mut rank_display_query: Query<&mut Text, With<RankDisplayer>>,
    ) {
        for mut rank_text in rank_display_query.iter_mut() {
            let mut compound_counter: HashMap<String, u32> = HashMap::new();
            for compound in compound_query.iter() {
                *compound_counter.entry(compound.to_string()).or_insert(0) += 1;
            }

            let mut compound_counts = compound_counter.into_iter().collect::<Vec<(String, u32)>>();
            compound_counts.sort_by(|(s1, v1), (s2, v2)| match v1.cmp(v2) {
                Ordering::Equal => s1.cmp(s2),
                other => other,
            });
            compound_counts.reverse();

            let mut result = "".to_string();
            compound_counts
                .into_iter()
                .map(|(s, v)| result = format!("{}{} - {}\n", result, v, s))
                .for_each(drop);

            rank_text.sections[0].value = result;
        }
    }

    pub fn reaction_test_input(
        mut cauldron_query: Query<(Entity, Option<&mut Heat>, &mut StirMethod), With<Cauldron>>,
        mut commands: Commands,
        input: Res<Input<KeyCode>>,
    ) {
        if let Some((entity, heat, mut stir_method)) = cauldron_query.iter_mut().next() {
            if input.just_pressed(KeyCode::Key0) {
                *stir_method = StirMethod::ZeroStir;
            } else if input.just_pressed(KeyCode::Key1) {
                *stir_method = StirMethod::SingleStir;
            } else if input.just_pressed(KeyCode::Key2) {
                *stir_method = StirMethod::DoubleStir;
            } else if input.just_pressed(KeyCode::Key4) {
                *stir_method = StirMethod::QuadrupleStir;
            }

            if input.pressed(KeyCode::B) {
                if let Some(mut heat) = heat {
                    *heat = Heat::Boiling;
                } else {
                    commands.entity(entity).insert(Heat::Boiling);
                }
            } else if input.pressed(KeyCode::S) {
                if let Some(mut heat) = heat {
                    *heat = Heat::Simmering;
                } else {
                    commands.entity(entity).insert(Heat::Simmering);
                }
            } else if heat.is_some() {
                commands.entity(entity).remove::<Heat>();
            }
        }
    }
}

use crate::alchemy::compounds::Compound;
use bevy::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Heat may or may not be present on a Cauldron,
/// If it's not present, no reaction should occur.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
pub enum Heat {
    Simmering,
    Boiling,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
pub enum StirMethod {
    /// As opposed to Heat, reactions may occur when there's no stirring,
    /// represented by this variant.
    ZeroStir,
    SingleStir,
    DoubleStir,
    QuadrupleStir,
}

#[derive(Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct ReactionRule {
    pub compound: Compound,
    /// Setting to None means this compound reacts under any heat
    pub heat: Option<Heat>,
    /// Setting to None means this compound reacts under any stir method
    pub stir_method: Option<StirMethod>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
pub struct Cauldron;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
pub struct RankDisplayer;

fn get_reactive_compounds(
    reaction_rules: Vec<ReactionRule>,
    stir_method: StirMethod,
    heat: Heat,
) -> Vec<Compound> {
    reaction_rules
        .clone()
        .into_iter()
        .filter(|rule| {
            (match rule.heat {
                Some(h) => h == heat,
                None => true,
            }) && (match rule.stir_method {
                Some(sm) => sm == stir_method,
                None => true,
            })
        })
        .map(|rule| rule.compound)
        .collect::<Vec<Compound>>()
}

const COLLISION_CHANCE: f32 = 0.1;
/// Assumes there will only ever be one cauldron.
/// In this case, we could technically handle it as a Resource, but I prefer the ergonomics of
/// having it represented by many components.
pub fn brewing(
    mut compound_query: Query<&mut Compound>,
    cauldron_query: Query<(&Heat, &StirMethod), With<Cauldron>>,
    reaction_rules: Res<Vec<ReactionRule>>,
) {
    if let Some((heat, stir_method)) = cauldron_query.iter().next() {
        let reactive_compounds =
            get_reactive_compounds(reaction_rules.clone(), *stir_method, *heat);

        let mut rng = rand::thread_rng();
        let mut colliding_compounds = compound_query
            .iter_mut()
            .filter(|compound| reactive_compounds.contains(compound))
            .filter(|_| rng.gen::<f32>() <= COLLISION_CHANCE);

        while let (Some(mut left), Some(mut right)) =
            (colliding_compounds.next(), colliding_compounds.next())
        {
            left.react(&mut right);
        }
    }
}

pub fn compound_rank_display(
    compound_query: Query<&Compound>,
    mut rank_display_query: Query<&mut Text, With<RankDisplayer>>,
) {
    for mut rank_text in rank_display_query.iter_mut() {
        let mut compound_counter: HashMap<String, u32> = HashMap::new();
        for compound in compound_query.iter() {
            *compound_counter.entry(compound.to_string()).or_insert(0) += 1;
        }

        let mut compound_counts = compound_counter.into_iter().collect::<Vec<(String, u32)>>();
        compound_counts.sort_by(|(_, v1), (_, v2)| v1.cmp(v2));
        compound_counts.reverse();

        let mut result = "".to_string();
        compound_counts
            .into_iter()
            .map(|(s, _)| result = format!("{}{}\n", result, s))
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

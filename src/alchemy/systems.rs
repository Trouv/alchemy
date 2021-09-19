use crate::alchemy::{components::*, compound::Compound, resources::ReactionRule};
use bevy::prelude::*;
use rand::Rng;

pub fn get_reactive_compounds(
    reaction_rules: &Vec<ReactionRule>,
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
        let reactive_compounds = get_reactive_compounds(&reaction_rules, *stir_method, *heat);

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

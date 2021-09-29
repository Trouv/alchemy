use crate::alchemy::{components::*, compound::Compound, resources::ReactionRule};
use bevy::prelude::*;
use rand::Rng;

/// Get all compounds that react under the given criteria according to the reaction rules.
/// `stir_method` and `heat` are optional,
/// where None means there is no requirement for that criteria.
/// For example, `stir_method=None` will only filter out compounds based on heat criteria.
pub fn get_reactive_compounds(
    reaction_rules: &Vec<ReactionRule>,
    stir_method: Option<StirMethod>,
    heat: Option<Heat>,
) -> Vec<Compound> {
    reaction_rules
        .clone()
        .into_iter()
        .filter(|rule| {
            // Rustfmt seems to convert &&s between match objects to double references.
            // So unfortunately using lets here.
            let stir_match = match stir_method {
                Some(sm) => match rule.stir_method {
                    Some(rule_sm) => sm == rule_sm,
                    None => true,
                },
                None => true,
            };
            let heat_match = match heat {
                Some(h) => match rule.heat {
                    Some(rule_h) => h == rule_h,
                    None => true,
                },
                None => true,
            };
            stir_match && heat_match
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
            get_reactive_compounds(&reaction_rules, Some(*stir_method), Some(*heat));

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

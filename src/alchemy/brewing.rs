use crate::alchemy::compounds::Compound;
use bevy::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
pub enum Heat {
    Sitting,
    Simmering,
    Boiling,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
pub enum StirMethod {
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
struct Cauldron;

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
